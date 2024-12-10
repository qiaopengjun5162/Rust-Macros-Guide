# macros

## 实操

```shell
➜ mkdir examples
➜ cargo add anyhow
➜ cargo run --example my_vec --quiet
➜ cargo run --example my_try --quiet
➜ cargo add futures --no-default-features
➜ cargo add tokio --features rt --features rt-multi-thread --features macros --features net --features fs
➜ cargo add proc-macro2
➜ cargo add syn --features extra-traits
➜ cargo add quote
➜ cargo install cargo-expand
➜ cargo expand --example enum_macro
```

## 参考
