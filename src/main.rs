mod ecs;
mod handle_index;
mod generational_index;

use ecs::Ecs;

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

    let ok = s.add_comp(entity, point);
    println!("{}", ok);
    let ok = s.add_comp(entity, Point { x: 3, y: 8 });
    println!("{}", ok);

    let p = s.get_comp::<Point>(entity).unwrap();
    println!("{:?}", p);
}
