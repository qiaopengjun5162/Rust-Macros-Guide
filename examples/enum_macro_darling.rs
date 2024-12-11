use macros::EnumFromDarling;

#[allow(unused)]
#[derive(EnumFromDarling, Debug)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    // Right(u32), // if two values are the same type, it will be ignored
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    let direction: Direction<i32> = DirectionUp::new(10).into();
    println!("Direction: {:?}", direction);
    let left: Direction<i32> = 10.into();
    println!("Left: {:?}", left);
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

// cargo run --example enum_macro --quiet
