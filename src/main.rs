extern crate gfx_backend_vulkan as vulkan;
extern crate gfx_hal as hal;
extern crate glsl_to_spirv;
#[macro_use] extern crate failure;

mod window;
mod renderer;

use window::Window;
use renderer::Renderer;

fn main() {


    let mut win = Window::new("rust_engine");
    let mut renderer = Renderer::new(&win);

    let mut running = true;
    let mut delta = 0.01;
    let r : f32 = 0.0;
    let g : f32 = 51.0;
    let mut b : f32 = 102.0;
    let a : f32 = 0.0;
    while running {
        running = win.poll_events();
        // b += delta;
        // if b >= 255.0 || b <= 0.0 { delta = -delta};
        renderer.draw_clear_colour([r/255.0, g/255.0, b/255.0, a]).unwrap();
    };
}
