### 编译安装安装
```sh
# 安装编译所需的依赖包
sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev libsdl2-dev \
              git tmux python3 python3-pip ninja-build

# 下载源码包
# 提取码：jimc
wget https://download.qemu.org/qemu-7.0.0.tar.xz

# 解压
tar xvJf qemu-7.0.0.tar.xz

# 编译安装并配置 RISC-V 支持
cd qemu-7.0.0
./configure --target-list=riscv64-softmmu,riscv64-linux-user,\
x86_64-softmmu,x86_64-linux-user \
--enable-sdl  # 如果要支持图形界面，可添加 " --enable-sdl" 参数
#   Directories
#     Install prefix               : /usr/local
#     BIOS directory               : share/qemu
#     firmware path                : /usr/local/share/qemu-firmware
#     binary directory             : bin
#     library directory            : lib
#     module directory             : lib/qemu
#     libexec directory            : libexec
#     include directory            : include
#     config directory             : /usr/local/etc
#     local state directory        : /usr/local/var
#     Manual directory             : share/man
#     Doc directory                : /usr/local/share/doc
#     Build directory              : /home/jipeng/qemu-7.0.0/build
#     Source path                  : /home/jipeng/qemu-7.0.0
#     GIT submodules               : ui/keycodemapdb tests/fp/berkeley-testfloat-3 tests/fp/berkeley-softfloat-3 dtc capstone slirp

# 编译
make -j$(nproc)

# 安装
sudo make install
# 这种方式可能与安装的其他版本冲突，另外一种更灵活的做法是将编译生成二进制文件的路径添加到 PATH 环境变量中，例如：
vi /etc/profile
export PATH=$PATH:/path/to/qemu-7.0.0/build
source /etc/profile

# 确认安装的版本
qemu-system-riscv64 --version
qemu-riscv64 --version
qemu-system-x86_64 --version
qemu-x86_64 --version
```


[环境配置](http://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html)

### 快捷键（chardev/char-mux.c）
退出 qemu
`ctrl+a x`

查看配置
`ctrl+a c`
info registers

待探索
`ctrl+a h`
`ctrl+a s`
`ctrl+a b`
`ctrl+a t`
