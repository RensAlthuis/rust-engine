use crate::shader::Shader;
use ash::vk;
use ash::version::DeviceV1_0;
use std::ffi::CString;

struct RenderPipeline<'d, D : DeviceV1_0> {
    shaders : &'d[Shader<'d, D>]
}

impl<'d, D : DeviceV1_0> RenderPipeline<'d, D> {

    pub fn new(name : &str, shaders : &'d[Shader<'d, D>], device : &'d D, extent : vk::Extent2D) -> RenderPipeline<'d, D> {

        let shader_stage_name = CString::new(name).unwrap();

        let shader_create_infos : Vec<vk::PipelineShaderStageCreateInfo> = shaders.iter().map(| shader | {
            vk::PipelineShaderStageCreateInfo::builder()
            .stage(shader.shader_type.into())
            .name(&shader_stage_name)
            .module(shader.module)
            .build()
        }).collect();

        let pipeline_vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&[])
            .vertex_binding_descriptions(&[]);

        let pipeline_input_assembly_state_create_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewport = vk::Viewport{
            x : 0.0,
            y : 0.0,
            width : extent.width as f32,
            height : extent.height as f32,
            min_depth : 0.0,
            max_depth : 1.0
        };

        let scissor = vk::Rect2D {
            extent : vk::Extent2D {width : extent.width as u32, height : extent.height as u32},
            offset : vk::Offset2D{x : 0, y : 0}
        };

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewport_count(1)
            .viewports(&[viewport])
            .scissor_count(1)
            .scissors(&[scissor]);

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::builder()
            .polygon_mode(vk::PolygonMode::FILL)
            .rasterizer_discard_enable(false)
            .depth_clamp_enable(false);


        // let pipeline =device.create_graphics_pipelines(pipeline_cache: vk::PipelineCache, create_infos: &[vk::GraphicsPipelineCreateInfo], Option::None);

        RenderPipeline{
            shaders : shaders
        }
    }
}