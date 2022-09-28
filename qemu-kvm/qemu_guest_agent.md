# qemu-ga 介绍
添加设备的 controller 
```xml
<controller type='virtio-serial' index='0' ports='2'>
  <alias name='virtio-serial0'/>
  <address type='pci' domain='0x0000' bus='0x00' slot='0x08' function='0x0'/>
</controller>
```

添加串口设备
```xml
<channel type='unix'>
  <source mode='bind' path='/var/lib/libvirt/qemu/channel/target/domain-2-abc1f073-d7e6-4f0d-9/org.qemu.guest_agent.0'/>
  <target type='virtio' name='org.qemu.guest_agent.0' state='connected'/>
  <alias name='channel0'/>
  <address type='virtio-serial' controller='0' bus='0' port='1'/>
</channel>
```

上述 xml 转化为 qemu 参数如下：
```sh
-device virtio-serial-pci,id=virtio-serial0,max_ports=2,bus=pci.0,addr=0x8 
-chardev socket,id=charchannel0,fd=51,server,nowait 
-device virtserialport,bus=virtio-serial0.0,nr=1,chardev=charchannel0,id=channel0,name=org.qemu.guest_agent.0 
```

`-chardev` socket 指定了一个字符设备，其对应为 `unix socket`，名字为 `virio-serial0`，在宿主机中可以看到类似于 `/var/lib/libvirt/qemu/channel/target/domain-42-fea480aa-5e8d-4eda-8/org.qemu.guest_agent.0` 的 socket 文件。

`-device virtio-serial-pci` 创建一个 virtio-serial 的 PCI 代理设备，其初始化时会创建一条 virtio-serial-bus，用来挂载 virtioserialport 设备。

`-device virtserialport` 创建一个 virioserialport 设备，其对应的 chardev 是 virio-serial0，名字是 org.qemu_agent.0，该设备会挂到 virio-serial-bus 上面，在虚拟机中我们就可以看到 /dev/virtio-ports/org.qemu.guest_agent.0 设备。

qemu 创建好了上述的设备之后，在虚拟机中执行 `qemu-ga --method=virtio-serial --path=/dev/virtio-ports/org.qemu.guest_agent.0`，就可以连接 `/dev/virtio-ports/org.qemu.guest_agent.0` 串口设备。在宿主机端可以连接 `/var/lib/libvirt/qemu/channel/target/domain-42-fea480aa-5e8d-4eda-8/org.qemu.guest_agent.0` 这个 unix socket，从而与虚拟机内部的 qemu-ga 通信，比如在宿主机端执行 guest-ping，虚拟机端的 qemu-ga 就返回执行结果。


# qemu-ga 原理
qemug-ga 的架构如下：

qemu 创建一个 virtserialport 串口设备，该串口设备还有一个 chardev 设备，提供虚拟机与外部设备的连接、数据传输等，对应的后端为 `unix socket`，对应的文件是 `/tmp/qga.sock`，qemu 还会将该 socket 文件的 fd 加入事件监听的主循环中。

当向虚拟机发送 qga 请求命令时，比如设置或获取一些信息，就要向 unix socket 文件写入请求数据，数据的格式与 qmp 命令一样也是 json。当 socket 收到数据时就会唤醒 qemu 的主循环，串口设备读取数据，然后填写 virtio 的 ving，向设备注入一个中断。

虚拟机中的设备接收到这个中断之后会读取数据，并唤醒用户态的 qga 进程。qemu-ga 本身有一个事件循环来监听 `/dev/vport0p1` 的数据，当它被唤醒时就会处理请求并生成应答数据，应答数据的格式也是 json。 应答数据通过 virtio 串口设备向 qemu 发送数据，qemu 则通过 chardev 设备向宿主机上的 unix socket 文件发送其应答数据。

上面是 qemu-ga 的大概运行原理。接下来我们从源码角度具体分析其原理。

qemu-ga 的执行入口如下：
```c
int main(int argc, char **argv)
{
   int ret = EXIT_SUCCESS;
   GAState *s = g_new0(GAState, 1);
   GAConfig *config = g_new0(GAConfig, 1);
   int socket_activation;
   …
   qga_qmp_init_marshal(&ga_commands);

   init_dfl_pathnames();
   config_load(config);
   config_parse(config, argc, argv);
   …

   s->log_level = config->log_level;
   s->log_file = stderr;
#ifdef CONFIG_FSFREEZE
   s->fsfreeze_hook = config->fsfreeze_hook;
#endif
   s->pstate_filepath = g_strdup_printf("%s/qga.state", config->state_dir);
   s->state_filepath_isfrozen = g_strdup_printf("%s/qga.state.isfrozen",
                                                config->state_dir);
   s->frozen = check_is_frozen(s);
   …

   ret = run_agent(s, config, socket_activation);
   …
}
```

上述的代码分为两部分，第一部分是初始化，第二部分是运行 qemu-ga agent。
初始化的部分主要包括记录 qemu-ga 在运行期间的状态和配置以及通过 qapi 注册 qemu-ga 支持的各种命令。
记录 qemu-ga 状态的结构体如下：
```c
struct GAState {
   JSONMessageParser parser;
   GMainLoop *main_loop;
   GAChannel *channel;
   …
   bool frozen;
   GList *blacklist;
   char *state_filepath_isfrozen;
   …
#ifdef CONFIG_FSFREEZE
   const char *fsfreeze_hook;
#endif
   …
};
```

其中 parser 主要用来解析 json 对象并调用设置的回调函数。main_loop 表示 qga 的主循环。channel 表示与 virtioserial 通信的通道。frozen 表示当前文件系统是否被 freeze，blacklist 记录不可执行命令的黑名单。
记录配置信息的结构体如下：
```c
typedef struct GAConfig {
   char *channel_path;
   char *method;
   …
#ifdef CONFIG_FSFREEZE
   char *fsfreeze_hook;
#endif
   …
   gchar *bliststr; /* blacklist may point to this string */
   GList *blacklist;
   int daemonize;
   …
} GAConfig;
```

channel_path 表示串行设备的路径，method 表示串行端口的类型，daemonsize 表示是否已守护进程运行。
config_load 和 config_parse 会解析配置参数并分配给 GAConfig。


# qemu-ga qemu 侧原理
命令行 -device virtio-serial-pci 会创建 virtio-serial 对应的类型为 virtio-serial-pci 的 pci 代理设备，其实例初始化函数为 virtio_serial_pci_instance_init。
hw/virtio/virtio-pci.c
```c
static void virtio_serial_pci_instance_init(Object *obj)
{
   VirtIOSerialPCI *dev = VIRTIO_SERIAL_PCI(obj);
 
   virtio_instance_init_common(obj, &dev->vdev, sizeof(dev->vdev),
                               TYPE_VIRTIO_SERIAL);
}
```
