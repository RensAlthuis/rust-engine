mod ecs;

use ecs::Ecs;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl ecs::Component for Point {}

fn main() {

}
