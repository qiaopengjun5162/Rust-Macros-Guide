use macros::EnumFrom;

#[allow(unused)]
#[derive(EnumFrom, Debug)]
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
    // let direction = Direction::Up(DirectionUp { speed: 10 });
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

// impl From<DirectionUp> for Direction {
//     fn from(direction: DirectionUp) -> Self {
//         Direction::Up(direction)
//     }
// }

// impl<T> From<i32> for Direction<T> {
//     fn from(value: i32) -> Self {
//         Direction::Left(value as u32)
//     }
// }

// impl<T> From<DirectionUp<T>> for Direction<T> {
//     fn from(direction: DirectionUp<T>) -> Self {
//         Direction::Up(direction)
//     }
// }
