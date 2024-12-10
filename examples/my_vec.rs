use anyhow::Result;

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
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

// my_vec! = my_vec! { 1, 2, 3 }; // Vec<i32>
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ( $( $x:expr ), +$(,)? ) => {{
        // let mut temp_vec = Vec::new();
        // $(
        //     temp_vec.push($x);
        // )*
        // temp_vec

        // vec![$($x),*]
        <[_]>::into_vec(Box::new([$($x),*]))
    }};
}
