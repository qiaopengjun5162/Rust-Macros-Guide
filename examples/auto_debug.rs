use macros::AutoDebug;

#[allow(unused)]
#[derive(AutoDebug)]
// #[debug(field = "inner")] // mutable = true,
pub struct RespBulkString {
    inner: String,
    #[debug(skip)]
    nothing: (),
    hello: String,
}

fn main() {
    let resp = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
        hello: "world".to_string(),
    };

    println!("resp: {:?}", resp);
}
