use ash::vk;
use ash::util::read_spv;
use std::fs::File;
use ash::version::DeviceV1_0;

#[derive(Copy, Clone)]
pub enum Type {
    Vertex,
    Fragment,
    Other
}

impl From<Type> for vk::ShaderStageFlags{
    fn from(t : Type) -> Self {
        match t {
            Type::Vertex => vk::ShaderStageFlags::VERTEX,
            Type::Fragment => vk::ShaderStageFlags::FRAGMENT,
            _ => vk::ShaderStageFlags::VERTEX,
        }
    }
}

pub struct Shader<'d, D : DeviceV1_0 > {
    pub module : vk::ShaderModule,
    pub shader_type : Type,
    device : &'d D
}

impl<'d, D : DeviceV1_0> Shader<'d, D>{
    pub fn new(path : &str, shader_type : Type, device  : &'d D) -> Shader<'d, D> {
        let mut file = File::open(path).unwrap();
        let words = read_spv(&mut file).unwrap();

        let create_info = vk::ShaderModuleCreateInfo::builder().code(&words);

        let module = unsafe {device.create_shader_module(&create_info, Option::None).unwrap()};
        Shader{
            module : module,
            shader_type : shader_type,
            device : device,
        }
    }
}

impl<'d, D : DeviceV1_0> Drop for Shader<'d, D> {
    fn drop (&mut self) {
        unsafe {
            self.device.destroy_shader_module(self.module, Option::None);
        }
    }
}