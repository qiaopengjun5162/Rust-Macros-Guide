use anyhow::Result;
use macros::my_vec;

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?
    ];
    let v1 = my_vec!(1, 2, 3);
    let v2 = my_vec!["a", "b", "c"];
    let v3 = my_vec! {1, 2, 3};
    let v4 = my_vec![5; 3];
    println!("{:?}", v);
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    println!("{:?}", v4);
    Ok(())
}

// ➜ cargo run --example my_vec --quiet
