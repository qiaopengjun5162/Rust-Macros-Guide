// use std::task::Poll;

// my_vec! = my_vec! { 1, 2, 3 }; // Vec<i32>
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ( $( $x:expr ),* ) => {{
        // let mut temp_vec = Vec::new();
        // $(
        //     temp_vec.push($x);
        // )*
        // temp_vec

        // vec![$($x),*]
        <[_]>::into_vec(Box::new([$($x),*]))
    }};
}

// ? operator. How to simulate it?
#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {{
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    }};
}

// my_ready! => Poll::Ready / Poll::Pending
#[macro_export]
macro_rules! my_ready {
    ($e:expr) => {
        match $e {
            Poll::Ready(v) => Poll::Ready(v),
            Poll::Pending => return Poll::Pending,
        }
    };
}
