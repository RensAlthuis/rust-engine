use gfx_backend_vulkan as vulkan;

use super::shader::Shader;
use super::shader::ShaderType;
use std::mem::ManuallyDrop;
use std::ptr::read;
use gfx_hal::{Device, Backend, Primitive, format , pass, image, pso};
use format::Format;
use pass::{SubpassDesc};
use gfx_hal::image::Layout;
use pso::{GraphicsPipelineDesc, GraphicsShaderSet};
use failure::Error;
use std::rc::Weak;



type RenderPass = <vulkan::Backend as Backend>::RenderPass;
type GraphicsPipeline = <vulkan::Backend as Backend>::GraphicsPipeline;
type PipelineLayout = <vulkan::Backend as Backend>::PipelineLayout;

pub struct RenderPipeline<'a> {
    pub render_pass : ManuallyDrop<RenderPass>,
    pipeline_layout : ManuallyDrop<PipelineLayout>,
    shaders : Vec<Shader<'a>>,
    pipeline : ManuallyDrop<GraphicsPipeline>,
    device : Weak<vulkan::Device>
}

impl<'a> RenderPipeline<'a> {
    pub fn new (device : Weak<vulkan::Device>, format : Format) -> RenderPipeline<'a> {

        let mut shaders = Vec::new();
        shaders.push(Shader::new(Weak::clone(&device), "main", "assets/triangle.vert.glsl", ShaderType::Vertex).unwrap());
        shaders.push(Shader::new(Weak::clone(&device), "main", "assets/triangle.frag.glsl", ShaderType::Fragment).unwrap());

        let render_pass =unsafe { RenderPipeline::new_render_pass(&device.upgrade().expect("RenderPipeline got non existent device"), format) }.unwrap();
        let pipeline_layout = unsafe { device.upgrade().expect("RenderPipeline got non existent device").create_pipeline_layout(&[], &[]).unwrap() };

        let mut pipeline_desc = GraphicsPipelineDesc::new(
            RenderPipeline::make_graphics_shader_set(&shaders),
            Primitive::TriangleList,
            pso::Rasterizer::FILL,
            &pipeline_layout,
            gfx_hal::pass::Subpass {
                index : 0,
                main_pass : &render_pass
            }
        );

        pipeline_desc.blender.targets.push(pso::ColorBlendDesc(
            pso::ColorMask::ALL,
            pso::BlendState::ALPHA,
        ));

        pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
            binding: 0,
            stride: 0,
            rate: pso::VertexInputRate::Vertex,
        });

        let pipeline = unsafe {
            let dev = device.upgrade();
            let dev = dev.expect("RenderPipeline got non existent device");
            let dev = dev.create_graphics_pipeline(&pipeline_desc, None).unwrap();
            dev
        };

        RenderPipeline {
            render_pass : ManuallyDrop::new(render_pass),
            pipeline_layout : ManuallyDrop::new(pipeline_layout),
            shaders,
            pipeline: ManuallyDrop::new(pipeline),
            device
        }

    }



    unsafe fn new_render_pass(device : &vulkan::Device, format : Format) -> Result<RenderPass, Error> {
        println!("{:?}", &format);
        let attachment = pass::Attachment {
            format : Some(format),
            samples : 1,
            ops: pass::AttachmentOps::new(
                pass::AttachmentLoadOp::Clear,
                pass::AttachmentStoreOp::Store
            ),
            stencil_ops: pass::AttachmentOps::DONT_CARE,
            layouts: image::Layout::Undefined..image::Layout::Present
        };

        let subpass = SubpassDesc {
            colors: &[(0, Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        let renderpass = device.create_render_pass(&[attachment], &[subpass], &[])?;

        Ok(renderpass)
    }

    fn make_graphics_shader_set(shaders : &'a [Shader<'a>]) -> GraphicsShaderSet<'a, vulkan::Backend> {
        GraphicsShaderSet{
            vertex : shaders[0].make_entry_point(),
            hull: None,
            domain: None,
            geometry: None,
            fragment : Some(shaders[1].make_entry_point())
        }
    }
}

impl<'a> Drop for RenderPipeline<'a> {
    fn drop(&mut self) {
        unsafe {
            self.device.upgrade().expect("RenderPipeline tried to destroy with non existent device").destroy_graphics_pipeline(
                ManuallyDrop::into_inner(read(&self.pipeline))
            );
            self.device.upgrade().expect("RenderPipeline tried to destroy with non existent device").destroy_pipeline_layout(
                ManuallyDrop::into_inner(read(&self.pipeline_layout))
            );
            self.device.upgrade().expect("RenderPipeline tried to destroy with non existent device").destroy_render_pass(
                ManuallyDrop::into_inner(read(&self.render_pass))
            );
        };
    }
}