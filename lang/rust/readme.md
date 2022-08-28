### 安装
`curl https://sh.rustup.rs -sSf | sh`  
`source $HOME/.cargo/env`  
或手动添加环境变量  
`export PATH="$HOME/.cargo/bin:$PATH"`  

### 更新版本
rustup update

### 卸载
rustup self uninstall

### 显示版本
rustc --version

### cargo 使用
新建项目：`cargo new hello`  
只编译检查，不生成可执行文件：`cargo check`  
调试模式构建：`cargo build`  
发布模式构建：`cargo build --release`  
创建 lib：`cargo new --lib mylib`  
运行：`cargo run`  
打印堆栈：`RUST_BACKTRACE=1 cargo run`  
