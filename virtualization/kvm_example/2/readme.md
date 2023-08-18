编译汇编，向 I/O 端口 0xf1 写入字符串 "Hello\n"
```bash
as -32 test.S -o test.o
objcopy -O binary test.o test.bin
```

编译 kvm
```bash
gcc kvm.c -o kvm
./kvm
```

输出
```bash
Hello
KVM_EXIT_HLT
```
