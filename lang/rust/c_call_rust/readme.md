C 语言调用 rust
1. `cargo new foo --lib`
2. `cargo build foo`  # 生成静态库，注意要修改 toml
3. `cp c_call_rust/foo/target/debug/libfoo.a ./`
4. `gcc -o main main.c libfoo.a` 或者 `gcc -o main main.c libfoo.a -lpthread -ldl`
5. ./main
