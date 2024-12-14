use macros::AutoDeref;

#[allow(unused)]
#[derive(AutoDeref, Debug)]
#[deref(field = "inner")] // mutable = true,
pub struct RespBulkString {
    inner: String,
    nothing: (),
}

fn main() {
    let resp = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
    };

    println!("resp: {:?}", resp);
}
