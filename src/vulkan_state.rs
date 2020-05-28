use crate::window::Window;

use ash::{vk, Device, Entry, Instance};
use ash::extensions::khr::{XlibSurface, Surface, Swapchain};
use ash::extensions::ext::DebugReport;
use ash::version::{InstanceV1_0, DeviceV1_0, EntryV1_0};

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

fn create_instance(entry : &Entry ) -> Result<Instance, ash::InstanceError> {

    let app_name = CString::new("rust_engine").unwrap();

    let app_info = vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .engine_name(&app_name)
        .application_version(0)
        .engine_version(0)
        .api_version(vk::make_version(1,1,0));

    let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
    let layer_names_raw : Vec<*const c_char> = {
        layer_names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect()
    };

    let extension_names = vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugReport::name().as_ptr(),
    ];

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names)
        .enabled_layer_names(&layer_names_raw);

    unsafe {entry.create_instance(&create_info, Option::None)}
}

unsafe extern "system" fn debug_callback(
    _flags: vk::DebugReportFlagsEXT,
    _object_type: vk::DebugReportObjectTypeEXT,
    _object: u64,
    _location: usize,
    _message_code: i32,
    _p_layer_prefix: *const c_char,
    p_message: *const c_char,
    _p_user_data: *mut c_void
) -> vk::Bool32 {

    println!("{:?}", CStr::from_ptr(p_message));
    vk::FALSE
}

fn create_debug(loader : &DebugReport) -> ash::prelude::VkResult<vk::DebugReportCallbackEXT> {

    let create_info = vk::DebugReportCallbackCreateInfoEXT::builder()
        .flags( vk::DebugReportFlagsEXT::DEBUG |
                vk::DebugReportFlagsEXT::ERROR |
                vk::DebugReportFlagsEXT::INFORMATION |
                vk::DebugReportFlagsEXT::WARNING |
                vk::DebugReportFlagsEXT::PERFORMANCE_WARNING)
        .pfn_callback(Option::Some(debug_callback));

    unsafe {loader.create_debug_report_callback(&create_info, Option::None)}
}

fn create_surface<E: EntryV1_0, I: InstanceV1_0>(
    entry: &E,
    instance: &I,
    window: &winit::window::Window
) -> Result<vk::SurfaceKHR, vk::Result> {

    use winit::platform::unix::WindowExtUnix;
    let display = WindowExtUnix::xlib_display(window).unwrap();
    let window = WindowExtUnix::xlib_window(window).unwrap();

    let create_info = vk::XlibSurfaceCreateInfoKHR::builder()
        .window(window)
        .dpy(display as *mut vk::Display);

    let xlib_surface_loader = XlibSurface::new(entry, instance);
    unsafe {xlib_surface_loader.create_xlib_surface(&create_info, Option::None)}
}

fn get_pdevice(instance : &Instance, surface_loader : &Surface , surface : &vk::SurfaceKHR) -> (vk::PhysicalDevice, u32) {
    unsafe {
        let devices = instance.enumerate_physical_devices().unwrap();
        devices.iter().map(|pdevice| {
            instance.get_physical_device_queue_family_properties(*pdevice)
            .iter().enumerate()
            .filter_map(|(index, ref info)| {
                let support = info.queue_flags.contains(vk::QueueFlags::GRAPHICS) &&
                            surface_loader.get_physical_device_surface_support(*pdevice, index as u32, *surface).unwrap();
                match support {
                    true => Option::Some((*pdevice, index as u32)),
                    false => Option::None
                }
            }
            ).nth(0)
        })
        .filter_map(|v| v)
        .nth(0)
        .unwrap()
    }
}

fn create_device(instance : &Instance, pdevice : &vk::PhysicalDevice, queue_family_index : u32) -> Result<Device, vk::Result> {
    let queue_priorities = [1.0];
    let queue_info = [vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_index)
        .queue_priorities(&queue_priorities).build()];

    let layer_names = [Swapchain::name().as_ptr()];
    let features = vk::PhysicalDeviceFeatures::builder()
        .shader_clip_distance(true);
    let device_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_info)
        .enabled_extension_names(&layer_names)
        .enabled_features(&features);

    unsafe{instance.create_device(*pdevice, &device_info, Option::None)}
}

fn create_swapchain(
        loader : &Swapchain,
        surface : &vk::SurfaceKHR,
        capabilities : &vk::SurfaceCapabilitiesKHR,
        formats : &Vec<vk::SurfaceFormatKHR>,
        present_modes : &Vec<vk::PresentModeKHR>,
        window : &Window,
    ) -> (ash::prelude::VkResult<vk::SwapchainKHR>, vk::SurfaceFormatKHR)
{

    let mut image_count = capabilities.min_image_count + 1;
    if image_count > capabilities.max_image_array_layers {
        image_count = capabilities.max_image_count;
    }
    //grab the first format, if it is vk::FORMAT::UNDEFINED pick the most common one (i guess)
    let format = formats.iter().map( |fmt| {
        match fmt.format {
            vk::Format::UNDEFINED => vk::SurfaceFormatKHR {
                format: vk::Format::B8G8R8A8_UNORM,
                color_space: fmt.color_space
           },
           _ => fmt.clone()
        }
    }).nth(0).unwrap();

    let image_extent = match capabilities.current_extent.width {
       0xFFFFFFFF => {
           let (width, height) : (u32, u32) = window.extent.into();
           vk::Extent2D {width, height}
       }
       _ => capabilities.current_extent
    };

    let present_mode = present_modes.iter().cloned().
        find(|&mode| { mode == vk::PresentModeKHR::MAILBOX })
        .unwrap_or(vk::PresentModeKHR::FIFO);

    let create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(*surface)
        .min_image_count(image_count)
        .image_color_space(format.color_space)
        .image_format(format.format)
        .image_extent(image_extent)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .image_array_layers(1);

    let swapchain = unsafe { loader.create_swapchain(&create_info, Option::None) };
    (swapchain, format)
}

fn create_image_views<D : DeviceV1_0> (images : &Vec<vk::Image>, swapchain_format : &vk::SurfaceFormatKHR, device : &D) -> Vec<vk::ImageView> {
        images.iter().map(| &image | {
            let create_info = vk::ImageViewCreateInfo::builder()
                .view_type(vk::ImageViewType::TYPE_2D)
                .components(vk::ComponentMapping{
                    r : vk::ComponentSwizzle::R,
                    g : vk::ComponentSwizzle::G,
                    b : vk::ComponentSwizzle::B,
                    a : vk::ComponentSwizzle::A,
                })
                .subresource_range(
                    vk::ImageSubresourceRange::builder()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .build()
                )
                .format(swapchain_format.format)
                .image(image);
                unsafe {device.create_image_view(&create_info, Option::None).unwrap()}
        }).collect()
}

pub struct VulkanState
{
    entry : Entry,
    instance : Instance,
    debug_callback : vk::DebugReportCallbackEXT,
    debug_loader : DebugReport,
    surface : vk::SurfaceKHR,
    surface_loader : Surface,
    pdevice : vk::PhysicalDevice,
    queue_family_index : u32,
    pub device : Device,
    swapchain : vk::SwapchainKHR,
    swapchain_loader : Swapchain,
    swapchain_format : vk::SurfaceFormatKHR,
    present_queue : vk::Queue,
    image_views : Vec<vk::ImageView>,
}

impl VulkanState
{
    pub fn new(window : &Window) -> Self {
        let entry = Entry::new().unwrap();
        let instance = create_instance(&entry).expect("instance creation failed");
        match entry.try_enumerate_instance_version().unwrap() {
            Option::Some(version) => {
                let major = vk::version_major(version);
                let minor = vk::version_minor(version);
                let patch = vk::version_patch(version);
                println!("Using vulkan api: v{}_{}_{}", major, minor, patch);
            },
            Option::None => {println!("Using vulkan api: v1_0_0");}
        };

        let debug_loader = DebugReport::new(&entry, &instance);
        let debug_callback = create_debug(&debug_loader).expect("failed to create callback");

        let surface = create_surface(&entry, &instance, &window.window).expect("surface creation failed");
        let surface_loader = Surface::new(&entry, &instance);

        let (pdevice, queue_family_index) = get_pdevice(&instance, &surface_loader, &surface);
        let device = create_device(&instance, &pdevice, queue_family_index).expect("device creation failed");

        let capabilities = unsafe { surface_loader.get_physical_device_surface_capabilities(pdevice, surface).expect("couldn't get capabilities")};
        let formats = unsafe { surface_loader.get_physical_device_surface_formats(pdevice, surface).expect("couldn't get formats")};
        let present_modes = unsafe { surface_loader.get_physical_device_surface_present_modes(pdevice, surface).expect("couldn't get present modes")};

        let swapchain_loader = Swapchain::new(&instance, &device);
        let (swapchain, swapchain_format) = create_swapchain(&swapchain_loader, &surface, &capabilities, &formats, &present_modes, &window);
        let swapchain = swapchain.expect("swapchain creation failed");

        let present_queue = unsafe { device.get_device_queue(queue_family_index as u32, 0)};
        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };
        let image_views = create_image_views(&images, &swapchain_format, &device);

        VulkanState {
            entry : entry,
            instance : instance,
            debug_callback : debug_callback,
            debug_loader : debug_loader,
            surface : surface,
            surface_loader : surface_loader,
            pdevice : pdevice,
            queue_family_index : queue_family_index,
            device : device,
            swapchain : swapchain,
            swapchain_loader : swapchain_loader,
            swapchain_format : swapchain_format,
            present_queue : present_queue,
            image_views : image_views,
        }
    }

}

impl Drop for VulkanState {
    fn drop(&mut self){
        unsafe {
            for &imageview in self.image_views.iter() {
                self.device.destroy_image_view(imageview, Option::None);
            }
            self.swapchain_loader.destroy_swapchain(self.swapchain, Option::None);
            self.device.destroy_device(Option::None);
            self.surface_loader.destroy_surface(self.surface, Option::None);
            self.debug_loader.destroy_debug_report_callback(self.debug_callback, None);
            self.instance.destroy_instance(Option::None);
        }
    }
}