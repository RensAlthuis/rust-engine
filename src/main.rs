#![allow(dead_code)]

#[macro_use] extern crate failure_derive;

mod ecs;
mod handle_index;
mod window;
mod renderer;


use ecs::Ecs;
use window::Window;
use renderer::Renderer;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl ecs::Component for Point {}

fn main() {
    let mut s = Ecs::new();
    let entity = s.create_entity();
    let point = Point{x:1, y:2};
    let ok = s.add_comp(&entity, point);
    println!("{}", ok);
    let ok = s.add_comp(&entity, Point { x: 3, y: 8 });
    println!("{}", ok);

    let p = s.get_comp::<Point>(&entity).unwrap();
    println!("{:?}", p);

    let mut w = Window::new("window");
    let mut r = Renderer::new(&w);
    while w.poll_events() {
        r.draw_clear_colour([1.0,0.0,0.0,1.0]).expect("clear colour failed");
    }

}
