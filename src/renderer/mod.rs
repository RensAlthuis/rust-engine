mod render_pipeline;
pub mod shader;

use super::window::Window;
use render_pipeline::RenderPipeline;
use std::rc::Rc;
use std::mem::ManuallyDrop;
use std::ptr::read;
use hal::{
Instance, Backend, Graphics, Device, Surface, QueueGroup,
CommandPool, Adapter, SwapchainConfig, Swapchain, SurfaceCapabilities,
Backbuffer,
command::{CommandBuffer, MultiShot, Primary, ClearValue, ClearColor},
pool::{CommandPoolCreateFlags},
format::{ChannelType, Format, Swizzle, Aspects},
window::{FrameSync, Extent2D},
image::{ViewKind, SubresourceRange, Extent},
pso::{Rect, PipelineStage},
queue::Submission
};
use failure::Error;
use arrayvec::ArrayVec;

pub struct Renderer<'a>{
    adapter : Adapter<vulkan::Backend>,
    device : ManuallyDrop<Rc<vulkan::Device>>,
    queue_group : QueueGroup<vulkan::Backend, Graphics>,
    swapchain : ManuallyDrop<<vulkan::Backend as Backend>::Swapchain>,
    render_pipeline : ManuallyDrop<RenderPipeline<'a>>,
    command_pool : ManuallyDrop<CommandPool<vulkan::Backend, Graphics>>,
    number_of_images : u8,
    current_frame : usize,
    fences : Vec<<vulkan::Backend as Backend>::Fence>,
    image_ready_semaphores : Vec<<vulkan::Backend as Backend>::Semaphore>,
    render_finished_semaphores : Vec<<vulkan::Backend as Backend>::Semaphore>,
    command_buffers : Vec<CommandBuffer<vulkan::Backend, Graphics, MultiShot, Primary>>,
    image_views: Vec<<vulkan::Backend as Backend>::ImageView>,
    framebuffers : Vec<<vulkan::Backend as Backend>::Framebuffer>,
    surface : <vulkan::Backend as Backend>::Surface,
    render_area : Rect,
    instance : ManuallyDrop<vulkan::Instance>,
}

impl<'a> Renderer<'a>{
    pub fn new(win : &Window) -> Renderer<'a> {
        let instance = vulkan::Instance::create("rust_engine", 1);
        let mut surface = instance.create_surface(&win.window);

        let adapter = instance.enumerate_adapters()
                            .pop()
                            .expect("[ERROR] Couldn't find a graphics adapter");
        println!("[INFO] Using graphics adapter: {:?}", adapter.info.name);


        let (device, queue_group) = {
            let (device, queue_group) = adapter.open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
                                            .expect("[ERROR] Couldn't find a suitable queue group");
            (Rc::new(device), queue_group)
        };

        let mut command_pool = unsafe {
            device.create_command_pool_typed(
                &queue_group,
                CommandPoolCreateFlags::RESET_INDIVIDUAL,
            )
        }.expect("[ERROR] failed to create command pool");

        let (caps, formats, _present_modes, _composite_alpha) = surface.compatibility(&adapter.physical_device);
        let format = Self::get_format(&formats);

        let render_pipeline = RenderPipeline::new(Rc::downgrade(&device), format);
        let (swapchain, backbuffer, number_of_images, extent) = unsafe { Self::make_swapchain(&device, &mut surface , format, &caps) };
        let (image_ready_semaphores, render_finished_semaphores, fences) = Self::make_synchronization_types(&device, number_of_images.into()).unwrap();
        let image_views = unsafe { Self::make_image_views(&device, &backbuffer, format).unwrap() };
        let framebuffers = unsafe {Self::make_framebuffers(&device, &image_views, &render_pipeline.render_pass, &extent.to_extent()).unwrap() };
        let command_buffers: Vec<_> = framebuffers.iter().map(|_| command_pool.acquire_command_buffer()).collect();
        let current_frame : usize = 0;
        let render_area = Rect {
            x : 0,
            y : 0,
            w : extent.width as i16,
            h : extent.height as i16
        };

        Renderer{
            instance : ManuallyDrop::new(instance),
            surface,
            adapter,
            device : ManuallyDrop::new(device),
            queue_group,
            swapchain : ManuallyDrop::new(swapchain),
            render_pipeline : ManuallyDrop::new(render_pipeline),
            command_pool: ManuallyDrop::new(command_pool),
            number_of_images,
            current_frame,
            fences,
            image_ready_semaphores,
            render_finished_semaphores,
            command_buffers,
            image_views,
            framebuffers,
            render_area
        }
    }

    fn get_format(formats : &Option<Vec<Format>> ) -> Format{
        match formats {
            None => Format::Rgba8Srgb,
            Some(formats) => match formats
            .iter()
            .find(|format| format.base_format().1 == ChannelType::Srgb)
            .cloned()
            {
            Some(srgb_format) => srgb_format,
            None => formats.get(0).cloned().unwrap(),
            },
        }
    }

    unsafe fn make_swapchain(device : &vulkan::Device, surface : &mut <vulkan::Backend as Backend>::Surface, format : Format, caps : &SurfaceCapabilities) -> (<vulkan::Backend as Backend>::Swapchain, hal::Backbuffer<vulkan::Backend>, u8, Extent2D) {
        let config = SwapchainConfig {
            present_mode : hal::window::PresentMode::Mailbox,
            composite_alpha : hal::window::CompositeAlpha::Opaque,
            format,
            extent : caps.extents.end,
            image_count : 3u32,
            image_layers : 1u16,
            image_usage : hal::image::Usage::COLOR_ATTACHMENT
        };

        let (swapchain, backbuffer) = device.create_swapchain(surface, config, None).unwrap();
        (swapchain, backbuffer, 3, caps.extents.end)
    }

    fn make_synchronization_types(device : &vulkan::Device, amount : u32) -> Result<(Vec<<vulkan::Backend as Backend>::Semaphore>, Vec<<vulkan::Backend as Backend>::Semaphore>, Vec<<vulkan::Backend as Backend>::Fence>), Error> {
        let mut image_ready_semaphores = Vec::new();
        let mut render_finished_semaphores = Vec::new();
        let mut fences = Vec::new();
        for _ in 0..amount {
            fences.push(device.create_fence(true)?);
            image_ready_semaphores.push(device.create_semaphore()?);
            render_finished_semaphores.push(device.create_semaphore()?);
        };
        Ok((image_ready_semaphores, render_finished_semaphores, fences))
    }

    unsafe fn make_image_views(device : &vulkan::Device, backbuffer : &Backbuffer<vulkan::Backend>, format : Format) -> Result<Vec<<vulkan::Backend as Backend>::ImageView>, Error> {
        match backbuffer {
            Backbuffer::Images(images) => {
                Ok(images.into_iter()
                      .map(| image | {
                          device.create_image_view(
                              &image,
                              ViewKind::D2,
                              format,
                              Swizzle::NO,
                              SubresourceRange {
                                  aspects: Aspects::COLOR,
                                  levels: 0..1,
                                  layers: 0..1
                              }
                          )
                      })
                      .collect::<Result<Vec<_>, hal::image::ViewError>>()?
                )
            },
            Backbuffer::Framebuffer(_) => Err(failure::err_msg("No Backbuffer Frambuffers"))
        }
    }

    unsafe fn make_framebuffers(device : &vulkan::Device, image_views : &Vec<<vulkan::Backend as Backend>::ImageView>, render_pass : &<vulkan::Backend as Backend>::RenderPass, extent : &Extent) -> Result<Vec<<vulkan::Backend as Backend>::Framebuffer>, Error> {
        Ok(image_views
            .iter()
            .map(| image_view | {
                device.create_framebuffer(
                    render_pass,
                    vec![image_view],
                    Extent {
                        width: extent.width as u32,
                        height: extent.height as u32,
                        depth: 1,
                    },
                )
        }).collect::<Result<Vec<_>, hal::device::OutOfMemory>>()?
        )
    }

    pub fn draw_clear_colour(&mut self, colour : [f32; 4]) -> Result<(), Error>{
        let image_available = &self.image_ready_semaphores[self.current_frame];
        let render_finished = &self.render_finished_semaphores[self.current_frame];
        self.current_frame = (self.current_frame + 1) % self.number_of_images as usize;

        let i_usize = unsafe {
            let image_index = self.swapchain
                .acquire_image(core::u64::MAX, FrameSync::Semaphore(image_available))
                .map_err(|_| failure::err_msg("Couldn't acquire an image from the swapchain!"))?;
            image_index as usize
        };

        let fence = &self.fences[i_usize];
        unsafe {
        self
            .device
            .wait_for_fence(fence, core::u64::MAX)
            .map_err(|_| failure::err_msg("Failed to wait on the fence!"))?;
        self
            .device
            .reset_fence(fence)
            .map_err(|_| failure::err_msg("Couldn't reset the fence!"))?;
        }

        unsafe{
            let buffer = &mut self.command_buffers[i_usize];
            let clear_values = [ClearValue::Color(ClearColor::Float(colour))];
            buffer.begin(false);
            buffer.begin_render_pass_inline(
                &self.render_pipeline.render_pass,
                &self.framebuffers[i_usize],
                self.render_area,
                clear_values.iter(),
            );
            buffer.finish();
        }


    // SUBMISSION AND PRESENT
    let command_buffers = &self.command_buffers[i_usize..=i_usize];
    let wait_semaphores: ArrayVec<[_; 1]> = [(image_available, PipelineStage::COLOR_ATTACHMENT_OUTPUT)].into();
    let signal_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
    // yes, you have to write it twice like this. yes, it's silly.
    let present_wait_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
    let submission = Submission {
      command_buffers,
      wait_semaphores,
      signal_semaphores,
    };
    let the_command_queue = &mut self.queue_group.queues[0];
    unsafe {
      the_command_queue.submit(submission, Some(fence));
      self
        .swapchain
        .present(the_command_queue, i_usize as u32, present_wait_semaphores)
        .map_err(|_| failure::err_msg("Failed to present into the swapchain!"))?;
    }
        Ok(())
    }
}

impl<'a> Drop for Renderer<'a> {
    fn drop(&mut self) {
        let _ = self.device.wait_idle();
        unsafe{
        for fence in self.fences.drain(..) {
            self.device.destroy_fence(fence)
        }
        for semaphore in self.render_finished_semaphores.drain(..) {
            self.device.destroy_semaphore(semaphore);
        }
        for semaphore in self.image_ready_semaphores.drain(..) {
            self.device.destroy_semaphore(semaphore);
        }
        for buffer in self.framebuffers.drain(..) {
            self.device.destroy_framebuffer(buffer);
        }
        for view in self.image_views.drain(..) {
            self.device.destroy_image_view(view);
        }
            self.device.destroy_command_pool(ManuallyDrop::into_inner(read(&mut self.command_pool)).into_raw());
            ManuallyDrop::drop(&mut self.render_pipeline);
            self.device.destroy_swapchain(ManuallyDrop::into_inner(read(&mut self.swapchain)));

            ManuallyDrop::drop(&mut self.device);
            ManuallyDrop::drop(&mut self.instance);
        }
    }
}