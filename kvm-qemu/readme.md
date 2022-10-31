介绍与使用虚拟化技术

## CPU 虚拟化
x86 架构下的 VMX 扩展  
VMX 下 vCPU 的完整生命周期  
Host 与 Guest 的切换  
指令的模拟  
KVM 虚拟多处理器  

## 内存虚拟化
操作系统如何为虚拟机呈现物理内存  
KVM 利用影子页表和 EPT 实现从 GVA 到 HPA 的2层地址映射  

## 中断虚拟化
8259A 的中断  
多核系统的 APIC   
I/O APIC  
从设备直接向 LAPIC 发送 MSI  
Intel 硬件层面支持虚拟化中断  

## 外设虚拟化
设备虚拟化的基本原理  
半虚拟化（virtio）  
Intel VT-d 硬件辅助虚拟化  
支持 SR-IOV 的 DMA 重映射  
中断重映射  

## 网络虚拟化
操作系统虚拟专用网络设备  

## tags
**MMU**  **IOMMU**  **DMA(Direct Memory Access)**  **中断重映射(Interrupt Remapping)**  **virtio**  **vhost**  **vhost-user** 


```sh
[root@jp-dev qemu-7.0.0]# make install
changing dir to build for make "install"...
make[1]: Entering directory '/home/qemu-7.0.0/build'
[1/20] Generating qemu-version.h with a custom command (wrapped by meson to capture output)
[1/2] Installing files.
Installing subdir /home/qemu-7.0.0/qga/run to /usr/local/var/run
Installing trace/trace-events-all to /usr/local/share/qemu
Installing qemu-system-riscv64 to /usr/local/bin
Installing qemu-riscv64 to /usr/local/bin
Installing qga/qemu-ga to /usr/local/bin
Installing qemu-keymap to /usr/local/bin
Installing qemu-img to /usr/local/bin
Installing qemu-io to /usr/local/bin
Installing qemu-nbd to /usr/local/bin
Installing storage-daemon/qemu-storage-daemon to /usr/local/bin
Installing qemu-edid to /usr/local/bin
Installing qemu-bridge-helper to /usr/local/libexec
Installing qemu-pr-helper to /usr/local/bin
Installing pc-bios/keymaps/ar to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/bepo to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/cz to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/da to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/de to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/de-ch to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/en-gb to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/en-us to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/es to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/et to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/fi to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/fo to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/fr to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/fr-be to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/fr-ca to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/fr-ch to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/hr to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/hu to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/is to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/it to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/ja to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/lt to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/lv to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/mk to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/nl to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/no to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/pl to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/pt to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/pt-br to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/ru to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/th to /usr/local/share/qemu/keymaps
Installing pc-bios/keymaps/tr to /usr/local/share/qemu/keymaps
Installing /home/qemu-7.0.0/include/qemu/qemu-plugin.h to /usr/local/include
Installing /home/qemu-7.0.0/ui/icons/qemu_16x16.png to /usr/local/share/icons/hicolor/16x16/apps
Installing /home/qemu-7.0.0/ui/icons/qemu_24x24.png to /usr/local/share/icons/hicolor/24x24/apps
Installing /home/qemu-7.0.0/ui/icons/qemu_32x32.png to /usr/local/share/icons/hicolor/32x32/apps
Installing /home/qemu-7.0.0/ui/icons/qemu_48x48.png to /usr/local/share/icons/hicolor/48x48/apps
Installing /home/qemu-7.0.0/ui/icons/qemu_64x64.png to /usr/local/share/icons/hicolor/64x64/apps
Installing /home/qemu-7.0.0/ui/icons/qemu_128x128.png to /usr/local/share/icons/hicolor/128x128/apps
Installing /home/qemu-7.0.0/ui/icons/qemu_256x256.png to /usr/local/share/icons/hicolor/256x256/apps
Installing /home/qemu-7.0.0/ui/icons/qemu_512x512.png to /usr/local/share/icons/hicolor/512x512/apps
Installing /home/qemu-7.0.0/ui/icons/qemu_32x32.bmp to /usr/local/share/icons/hicolor/32x32/apps
Installing /home/qemu-7.0.0/ui/icons/qemu.svg to /usr/local/share/icons/hicolor/scalable/apps
Installing /home/qemu-7.0.0/ui/qemu.desktop to /usr/local/share/applications
Installing /home/qemu-7.0.0/pc-bios/bios.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/bios-256k.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/bios-microvm.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/qboot.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/sgabios.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vgabios.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vgabios-cirrus.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vgabios-stdvga.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vgabios-vmware.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vgabios-qxl.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vgabios-virtio.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vgabios-ramfb.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vgabios-bochs-display.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vgabios-ati.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/openbios-sparc32 to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/openbios-sparc64 to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/openbios-ppc to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/QEMU,tcx.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/QEMU,cgthree.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/pxe-e1000.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/pxe-eepro100.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/pxe-ne2k_pci.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/pxe-pcnet.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/pxe-rtl8139.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/pxe-virtio.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/efi-e1000.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/efi-eepro100.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/efi-ne2k_pci.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/efi-pcnet.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/efi-rtl8139.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/efi-virtio.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/efi-e1000e.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/efi-vmxnet3.rom to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/qemu-nsis.bmp to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/bamboo.dtb to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/canyonlands.dtb to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/petalogix-s3adsp1800.dtb to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/petalogix-ml605.dtb to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/multiboot.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/multiboot_dma.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/linuxboot.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/linuxboot_dma.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/kvmvapic.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/pvh.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/s390-ccw.img to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/s390-netboot.img to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/slof.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/skiboot.lid to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/palcode-clipper to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/u-boot.e500 to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/u-boot-sam460-20100605.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/qemu_vga.ndrv to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/edk2-licenses.txt to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/hppa-firmware.img to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/opensbi-riscv32-generic-fw_dynamic.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/opensbi-riscv64-generic-fw_dynamic.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/npcm7xx_bootrom.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vof.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/vof-nvram.bin to /usr/local/share/qemu
Installing /home/qemu-7.0.0/pc-bios/keymaps/sl to /usr/local/share/qemu/keymaps
Installing /home/qemu-7.0.0/pc-bios/keymaps/sv to /usr/local/share/qemu/keymaps
Running custom install script '/usr/bin/python3 /home/qemu-7.0.0/meson/meson.py --internal gettext install --subdir=po --localedir=share/locale --pkgname=qemu'
make[1]: Leaving directory '/home/qemu-7.0.0/build'
```

## exercises
[使用Rust实现vhost-user设备](https://github.com/oscomp/proj129-vhost-user-devices-in-rust)  
[用Rust语言重写Linux kernel中的KVM](https://github.com/oscomp/proj178-kvm-in-rust)  
[深入 virio-fs]()
