use std::io::Read;
use hal::{Backend, Device, pso};
use pso::EntryPoint;
use std::fs;
use failure::Error;

type ShaderModule = <vulkan::Backend as Backend>::ShaderModule;

pub struct Shader<'a> {
   entry :  &'a str,
   module : Option<Box<ShaderModule>>,
   specialization : pso::Specialization<'a>,
   device : &'a vulkan::Device
}

#[derive(Fail, Debug)]
enum ShaderError{
    #[fail(display = "No such file: {}", _0)]
    NoSuchFile(String),
    #[fail(display = "Could not compile: {}", _0)]
    CompileError(String),
    #[fail(display = "Entry Point not created: shader_module is None")]
    EntryPointError
}

impl<'a> Shader<'a> {
    pub fn new(device : &'a vulkan::Device, entry_name : &'a str, path : &str) -> Result<Shader<'a>, Error> {
        let glsl = fs::read_to_string(path)?;
        let bin = glsl_to_spirv::compile(glsl.as_str(), glsl_to_spirv::ShaderType::Fragment)
            .map_err(| err | {ShaderError::CompileError(err)})?;
        let bin : Vec<u8> = bin.bytes().map(| result | { result.unwrap() }).collect();
        let shader_module : Box<ShaderModule> = Box::new(unsafe {device.create_shader_module(&bin)}?);
        let specialization = pso::Specialization::default();

        let shader = Shader {
            entry : entry_name.clone(),
            module : Some(shader_module),
            specialization : specialization,
            device : device
        };


        Ok(shader)

    }

    fn make_entry_point(&'a self) -> Option<EntryPoint<'a, vulkan::Backend>> {
        if let Some(ref module) = self.module {
            return Some(EntryPoint {
                entry: self.entry,
                module : module,
                specialization : self.specialization
            });
        }
        None
    }

}


impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        let module = self.module.take().unwrap();
        unsafe {self.device.destroy_shader_module(*module)};
    }
}