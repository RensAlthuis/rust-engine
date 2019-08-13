extern crate failure;

use gfx_backend_vulkan as vulkan;
use std::fs;
use std::mem::ManuallyDrop;
use std::ptr::read;
use std::io::Read;
use gfx_hal::{Backend, Device, pso};
use pso::EntryPoint;
use failure::Error;
use std::rc::Weak;

type ShaderModule = <vulkan::Backend as Backend>::ShaderModule;

pub use glsl_to_spirv::ShaderType;

pub struct Shader<'a> {
   entry :  &'a str,
   module : ManuallyDrop<ShaderModule>,
   specialization : pso::Specialization<'a>,
   device : Weak<vulkan::Device>
}

#[derive(Fail, Debug)]
enum ShaderError{
    #[fail(display = "Could not compile: {}", _0)]
    CompileError(String),
}

impl<'a> Shader<'a> {
    pub fn new(device : Weak<vulkan::Device>, entry_name : &'a str, path : &str, shader_type : ShaderType) -> Result<Shader<'a>, Error> {
        let glsl = fs::read_to_string(path)?;
        let bin = glsl_to_spirv::compile(glsl.as_str(), shader_type)
            .map_err(| err | {ShaderError::CompileError(err)})?;
        let bin : Vec<u8> = bin.bytes().map(| result | { result.unwrap() }).collect();
        let shader_module = unsafe {device.upgrade().expect("Shader got non existent device").create_shader_module(&bin)}?;
        let specialization = pso::Specialization::default();

        let shader = Shader {
            entry : entry_name,
            module : ManuallyDrop::new(shader_module),
            specialization,
            device
        };


        Ok(shader)

    }

    pub fn make_entry_point(&'a self) -> EntryPoint<'a, vulkan::Backend> {
        EntryPoint {
            entry: self.entry,
            module : &self.module,
            specialization : self.specialization.clone()
        }
    }

}


impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        unsafe {
            self.device.upgrade().expect("Shader called destroy with non existent device").destroy_shader_module(
                ManuallyDrop::into_inner(read(&self.module))
            );
        };
    }
}