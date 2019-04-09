use hal::{Device, Surface, Adapter, Backend, format , pass, image, pso};
use format::{ChannelType, Format};
use pso::{GraphicsPipelineDesc, GraphicsShaderSet, EntryPoint};
use super::shader::Shader;

type RenderPass = <vulkan::Backend as Backend>::RenderPass;
type GraphicsPipeline = <vulkan::Backend as Backend>::GraphicsPipeline;

pub struct RenderPipeline<'a> {
    render_pass : RenderPass,
    // pipeline : GraphicsPipeline,
    shaders : Vec<Shader<'a>>
}

impl<'a> RenderPipeline<'a> {
    pub fn new (device : &vulkan::Device, surface : &Box<Surface<vulkan::Backend>>, adapter : &Adapter<vulkan::Backend>) -> RenderPipeline<'a> {

        // let pipeline_desc = GraphicsPipelineDesc {
    // pub shaders: GraphicsShaderSet<'a, B>,
    // pub rasterizer: Rasterizer,
    // pub vertex_buffers: Vec<VertexBufferDesc>,
    // pub attributes: Vec<AttributeDesc>,
    // pub input_assembler: InputAssemblerDesc,
    // pub blender: BlendDesc,
    // pub depth_stencil: DepthStencilDesc,
    // pub multisampling: Option<Multisampling>,
    // pub baked_states: BakedStates,
    // pub layout: &'a B::PipelineLayout,
    // pub subpass: Subpass<'a, B>,
    // pub flags: PipelineCreationFlags,
    // pub parent: BasePipeline<'a, B::GraphicsPipeline>,
        // };

        // let pipeline = device.create_graphics_pipeline(&pipeline_desc, None);
        let shaders = Vec::new();
        let s = Shader::new(device, "main", "../assets/triangle.vert.glsl");

        RenderPipeline {
            render_pass : Self::new_render_pass(device, surface, adapter),
            shaders
            // pipeline
        }
    }

    fn get_format(formats : Option<Vec<Format>> ) -> Format{
        formats.map_or(Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .map(|format| *format)
                .unwrap_or(formats[0])
        })
    }

    fn new_render_pass(device : &vulkan::Device, surface : &Box<Surface<vulkan::Backend>>, adapter : &Adapter<vulkan::Backend>) -> RenderPass {
        let (_caps, formats, _present_modes, _composite_alpha) = surface.compatibility(&adapter.physical_device);

        let attachment = pass::Attachment {
            format : Some(Self::get_format(formats)),
            samples : 1,
            ops: pass::AttachmentOps::new(
                pass::AttachmentLoadOp::Clear,
                pass::AttachmentStoreOp::Store
            ),
            stencil_ops: pass::AttachmentOps::DONT_CARE,
            layouts: image::Layout::Undefined..image::Layout::Present
        };

        unsafe{
            device.create_render_pass(&[attachment], &[], &[])
        }
        .expect("Couldn't create Renderpass")

    }

    // fn new_GraphicsShaderSet() -> GraphicsShaderSet {

    //     let vertex = Shader::new();
    //     let frag = Shader::new();
    //     GraphicsShaderSet{
    //         vertex,
    //         frag : Some(frag)
    //     }
    // }
}

impl<'a> Drop for RenderPipeline<'a> {
    fn drop(&mut self) {

    }
}