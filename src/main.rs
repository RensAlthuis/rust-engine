extern crate gfx_backend_vulkan as vulkan;
extern crate gfx_hal as hal;
extern crate glsl_to_spirv;
#[macro_use] extern crate failure;

mod window;
mod renderer;

use window::Window;
use renderer::Renderer;
use renderer::shader;

fn main() {


    let win = Window::new("rust_engine");
    let mut renderer = Renderer::new(win);


    let mut running = true;
    while running {
        running = renderer.update();
    }
}
