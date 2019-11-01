#![allow(dead_code)]
#![allow(unused_imports)]
// use std::ops::Drop;
// use std::os::raw::{c_char, c_void};
// mod ecs;
// mod handle_index;
// use ecs::Ecs;

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl ecs::Component for Point {}

// let mut s = Ecs::new();
// let entity = s.create_entity();
// let point = Point{x:1, y:2};
// let ok = s.add_comp(&entity, point);
// println!("{}", ok);
// let ok = s.add_comp(&entity, Point { x: 3, y: 8 });
// println!("{}", ok);

// let p = s.get_comp::<Point>(&entity).unwrap();
// println!("{:?}", p);

#[macro_use] extern crate ash;
extern crate winit;

mod window;
mod vulkan_state;
mod shader;
mod render_pipeline;

use window::Window;
use vulkan_state::VulkanState;
use shader::Shader;

use ash::vk;
use std::ffi::CString;

fn main() {

    let w = Window::new("window");
    let vk_state = VulkanState::new(&w);
    let vertex_shader = Shader::new("assets/vert.spv", shader::Type::Vertex, &vk_state.device);
    let fragment_shader = Shader::new("assets/frag.spv", shader::Type::Fragment, &vk_state.device);






    println!("Program End");
    // while w.poll_events() {
    // }
}
