**mmap**  
* 作用：`mmap` 能将一个磁盘文件映射到存储空间中的一个缓冲区上。
当想缓冲区中取数据时，就相当于读文件中的相应字节，将数据存入缓冲区时，相应字节就自动写入文件。
这样，就可以在不使用 read 和 write 的情况下执行 I/O。
* 头文件：`#include<sys/mmap.h>`
* 函数签名：`void *mmap(void *addr, size_t len, int prot, int flags,int fd, off_t off)`
* 参数：
    * `addr` 用于指定映射存储区的起始地址。通常将其设置为0，这表示有系统选择该映射区的起始地址。
    * `len` 映射的字节数。
    * `prot` 指定映射区的保护要求，包含如下值：
        * `PROT_READ` 映射区可读
        * `PROT_WRITE`映射区可写
        * `PROT_EXEC` 映射区可执行
        * `PROT_NONE` 映射区不可访问
    * `flags` 设置映射存储区的属性
        * `MAP_FIXED`  返回值必须等于 addr。如果未指定此标志，而且 addr 非0，则内核只把 addr 视为在何处设置映射区的一种建议，但不保证会使用所要求的的地址。将 addr 指定为 0 可以获得最大可移植性。
        * `MAP_SHARED`  这一标志描述了本进程对映射区所进行的存储操作的配置。此标志指定存储操作修改映射文件，也就是，存储操作相当于对该文件的 write。
        * `MAP_PRIVATE`  本标志说明，对映射区的存储操作导致创建该映射文件的衣蛾私有副本，所有后来对该映射区的引用都是引用该副本。（此标志的一种用途是用于调试程序，它将程序文件的正文部分映射至存储区，但允许用户修改其中的指令，任何修改只影响程序文件的副本，而不影响源文件。）
    * `fd` 指定要被映射文件的描述符。在文件映射地到地址空间之前，必须先打开该文件（open 函数返回）。
    * `off` 要映射字节在文件中的起始偏移量，大小为 PAGE_SIZE 的整数倍。

    > 注意事项：  
    > 1. 可将 `prot` 参数指定为 PROT_NONE、PROT_READ或其他值的任意组合的按位或。
    > 2. 对存储映射区指定 prot 保护要求不能超过 open 模式访问权限。例如：若文件是只读打开的，那么对存储映射区就不能指定 `PROT_WRITE`。
    > 3. off 的值和 addr 的值（如果指定了 MAP_FIXED）通常被要求是系统虚拟存储页长度的整数倍。
* 返回值
映射区的起始地址，即被映射区的指针，表示需要映射的内核空间在用户空间的虚拟地址。

**应用**  
使用 mmap 复制文件（类似于 cp 命令）
```c
#include "apue.h"
#include <fcntl.h>
#include <sys/mman.h>

#define COPYINCR (1024*1024*1024)   /* 1 GB */

int
main(int argc, char *argv[])
{
    int         fdin, fdout;
    void        *src, *dst;
    size_t      copysz;
    struct stat sbuf;
    off_t       fsz = 0;

    if (argc != 3)
        err_quit("usage: %s <fromfile> <tofile>", argv[0]);

    if ((fdin = open(argv[1], O_RDONLY)) < 0)
        err_sys("can't open %s for reading", argv[1]);

    if ((fdout = open(argv[2], O_RDWR | O_CREAT | O_TRUNC, FILE_MODE)) < 0)
        err_sys("can't creat %s for writing", argv[2]);

    if (fstat(fdin, &sbuf) < 0)         /* need size of input file */
        err_sys("fstat error");

    if (ftruncate(fdout, sbuf.st_size) < 0) /* set output file size */
        err_sys("ftruncate error");

    while (fsz < sbuf.st_size) {
        if ((sbuf.st_size - fsz) > COPYINCR)
            copysz = COPYINCR;
        else
            copysz = sbuf.st_size - fsz;

        if ((src = mmap(0, copysz, PROT_READ, MAP_SHARED, fdin, fsz)) == MAP_FAILED)
            err_sys("mmap error for input");
        if ((dst = mmap(0, copysz, PROT_READ | PROT_WRITE, MAP_SHARED, fdout, fsz)) == MAP_FAILED)
            err_sys("mmap error for output");

        memcpy(dst, src, copysz);   /* does the file copy */
        munmap(src, copysz);
        munmap(dst, copysz);
        fsz += copysz;
    }
    exit(0);
}
```

《UNIX环境高级编程（第3版）》  
[mmap详解](https://nieyong.github.io/wiki_cpu/mmap%E8%AF%A6%E8%A7%A3.html#toc_0.1)
