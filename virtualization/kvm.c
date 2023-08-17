#include <linux/kvm.h>
#include <stdlib.h>
#include <stdio.h>
#include <fcntl.h>
#include <sys/mman.h>

struct vm {
    int vm_fd;
    __u64 ram_size;
    __u64 ram_start;
    struct kvm_userspace_memory_region mem;
    struct vcpu *vcpus[1];
};


struct vcpu {
    int id;
    int fd;
    struct kvm_run *run;
    struct kvm_regs regs;
    struct kvm_sregs sregs;
}

int g_dev_fs;

int main(int argc, char **argv) {
    if(g_dev_fd = open("/dev/kvm", O_RDWR) < 0) {
        fprintf(stderr, "Could not open /dev/kvm.\n")
        exit(1);
    }

    struct  vm *vm = malloc(sizeof(struct vm));
    struct vcpu *vcpu = malloc(sizeof(struct vcpu));
    vcpu->id = 0;
    vm->vcpus[0] = vcpu;

    setup_vm(vm, 64 * 1024 * 1024);
    load_image(vm);
    run_vm(vm);

    return 0;
}

int setup_vm(sturct vm *vm, int ram_size) {
    int ret = 0;
    // 创建虚拟机实例
    if((vm->vm_fd = ioctl(g_dev_fd, KVM_CREATE_VM, 0)) < 0) {
        fprintf(stderr, "Could not create vm.\n");
        ret = -1;
        goto err;
    }

    // 创建内存
    vm->ram_size = ram_size;
    vm->ram_start = (__u64) mmap(NULL, vm->ram_size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE, -1, 0);
    if((void *)vm->ram_start == (__u64) MAP_FAILED) {
        fprintf(stderr, "Could not mmap ram.\n");
        ret = -1;
        goto err;
    }

    vm->mem.slot = 0;


    // 创建处理器
err:
    return ret;
}



