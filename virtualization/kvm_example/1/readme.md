编译 kernel
```bash
cd guest
make all
```

变异 kvm
```bash
gcc kvm.c -o kvm
sudo ./kvm
```