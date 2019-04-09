mod render_pipeline;
pub mod shader;

use super::window::Window;
use hal::{Instance, Graphics, Device, pool::CommandPoolCreateFlags, Adapter};
type Surface = hal::Surface<vulkan::Backend>;
type QueueGroup = hal::QueueGroup<vulkan::Backend, Graphics>;
type CommandPool = hal::CommandPool<vulkan::Backend, Graphics>;
use render_pipeline::RenderPipeline;


pub struct Renderer<'a>{
    instance : vulkan::Instance,
    window : Window,
    surface : Box<Surface>,
    adapter : Adapter<vulkan::Backend>,
    pub device : vulkan::Device,
    queue_group : QueueGroup,
    command_pool : Option<CommandPool>,
    render_pipeline : RenderPipeline<'a>
}

impl<'a> Renderer<'a>{
    pub fn new(win : Window) -> Renderer<'a> {
        let instance = vulkan::Instance::create("rust_engine", 1);
        let surface : Box<Surface> = Box::new(instance.create_surface(&win.window));

        let adapter = instance.enumerate_adapters()
                            .pop()
                            .expect("[ERROR] Couldn't find a graphics adapter");
        println!("[INFO] Using graphics adapter: {:?}", adapter.info.name);


        let (device, mut queue_group) = adapter.open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
                                            .expect("[ERROR] Couldn't find a suitable queue group");
        println!("[INFO] queue family id: {:?}", queue_group.family());
        let mut command_pool = unsafe {
            device.create_command_pool_typed(
                &queue_group,
                CommandPoolCreateFlags::empty(),
            )
        }.expect("[ERROR] failed to create command pool");
        let render_pipeline = RenderPipeline::new(&device, &surface, &adapter);
        Renderer{
            instance,
            window : win,
            adapter,
            device,
            queue_group,
            command_pool: Some(command_pool),
            surface,
            render_pipeline
        }
    }

    pub fn update(&mut self)-> bool {
        self.window.poll_events()
    }
}

impl<'a> Drop for Renderer<'a> {
    fn drop(&mut self) {
        let pool = self.command_pool.take();
        unsafe{
            self.device.destroy_command_pool(pool.unwrap().into_raw());
        }
    }
}