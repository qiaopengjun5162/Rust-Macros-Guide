use macros::EnumFrom;

#[allow(unused)]
#[derive(EnumFrom, Debug)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    // Right(u32), // if two values are the same type, it will be ignored
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

fn main() {
    // let direction = Direction::Up(DirectionUp { speed: 10 });
    let direction: Direction = DirectionUp::new(10).into();
    println!("Direction: {:?}", direction);
    let left: Direction = 10.into();
    println!("Left: {:?}", left);
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}

// cargo run --example enum_macro --quiet

// impl From<DirectionUp> for Direction {
//     fn from(direction: DirectionUp) -> Self {
//         Direction::Up(direction)
//     }
// }
