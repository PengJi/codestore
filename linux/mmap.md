`mmap` 能将一个磁盘文件映射到存储空间中的一个缓冲区上。
当想缓冲区中取数据时，就相当于读文件中的相应字节，将数据存入缓冲区时，相应字节就自动写入文件。
这样，就可以在不使用 read 和 write 的情况下执行 I/O。


[mmap详解](https://nieyong.github.io/wiki_cpu/mmap%E8%AF%A6%E8%A7%A3.html#toc_0.1)