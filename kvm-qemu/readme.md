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


## exercises
[使用Rust实现vhost-user设备](https://github.com/oscomp/proj129-vhost-user-devices-in-rust)  
[用Rust语言重写Linux kernel中的KVM](https://github.com/oscomp/proj178-kvm-in-rust)  
[深入 virio-fs]
