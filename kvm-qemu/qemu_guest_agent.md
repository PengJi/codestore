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
各参数含义如下：  
`-chardev socket`  
指定了一个字符设备，其对应为 `unix socket`，名字为 `virio-serial0`，在宿主机中可以看到类似于 `/var/lib/libvirt/qemu/channel/target/domain-42-fea480aa-5e8d-4eda-8/org.qemu.guest_agent.0` 的 socket 文件。

`-device virtio-serial-pci`  
创建一个 `virtio-serial` 的 PCI 代理设备，其初始化时会创建一条 `virtio-serial-bus`，用来挂载 `virtioserialport` 设备。

`-device virtserialport`  
创建一个 `virioserialport` 设备，其对应的 chardev 是 `virio-serial0`，名字是 `org.qemu_agent.0`，该设备会挂到 `virio-serial-bus` 上面，在虚拟机中我们就可以看到 `/dev/virtio-ports/org.qemu.guest_agent.0` 设备。

qemu 创建好了上述的设备之后，在虚拟机中执行 `qemu-ga --method=virtio-serial --path=/dev/virtio-ports/org.qemu.guest_agent.0`，就可以连接 `/dev/virtio-ports/org.qemu.guest_agent.0` 串口设备。在宿主机端可以连接 `/var/lib/libvirt/qemu/channel/target/domain-42-fea480aa-5e8d-4eda-8/org.qemu.guest_agent.0` 这个 `unix socket`，从而与虚拟机内部的 qemu-ga 通信，比如在宿主机端执行 guest-ping，虚拟机端的 qemu-ga 就返回执行结果。


# qemu-ga 原理
qemug-ga 的架构如下：

![](./images/qemu-ga_qemu.jpg)

qemu 创建一个 `virtserialport` 串口设备，该串口设备还有一个 `chardev` 设备，提供虚拟机与外部设备的连接、数据传输等，对应的后端为 `unix socket`，对应的文件是 `/tmp/qga.sock`，qemu 还会将该 socket 文件的 fd 加入事件监听的主循环中。

当向虚拟机发送 qemu-ga 请求命令时，比如设置或获取一些信息，就要向 `unix socket` 文件写入请求数据，数据的格式与 qmp 命令一样也是 json。当 socket 收到数据时就会唤醒 qemu 的主循环，串口设备读取数据，然后填写 virtio 的 ving，向设备注入一个中断。

虚拟机中的设备接收到这个中断之后会读取数据，并唤醒用户态的 qga 进程。qemu-ga 本身有一个事件循环来监听 `/dev/vport0p1` 的数据，当它被唤醒时就会处理请求并生成应答数据，应答数据的格式也是 json。 应答数据通过 virtio 串口设备向 qemu 发送数据，qemu 则通过 chardev 设备向宿主机上的 `unix socket` 文件发送其应答数据。

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
   // 注册 qmp 命令
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

   // 运行 agent
   ret = run_agent(s, config, socket_activation);
   …
}
```

上述的代码分为两部分，第一部分是初始化（配置、qmp 命令），第二部分是运行 qemu-ga。

第一部分初始化的部分主要包括记录 qemu-ga 在运行期间的状态和配置以及通过 qapi 注册 qemu-ga 支持的各种命令。

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
`parser` 主要用来解析 json 对象并调用设置的回调函数。  
`main_loop` 表示 qemu-ga 的主循环。  
`channel` 表示与 `virtioserial` 通信的通道。  
`frozen` 表示当前文件系统是否被 freeze。  
`blacklist` 记录不可执行命令的黑名单。

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
`channel_path` 表示串行设备的路径。  
`method` 表示串行端口的类型，可以使 virio-serial 或者 isa。  
`daemonsize` 表示是否已守护进程运行。

qemu-ga 的 main 函数首先会分配 GAState 和 GACconfig 结构，并调用     `config_load(config)` 和 `config_parse(config, argc, argv)` 把环境变量和命令行参数解析到 GACconfig，比如`--method=virtio-serial`会把值赋给 `config->method`，`--path=/dev/virtio-ports/org.qemu.guest_agent.0` 会把值赋给 `config->path`。 
`config_load` 和 `config_parse` 会解析配置参数并分配给 `GAConfig`。

初始化的第二部分工作还包括将 qemu-ga 支持的各个命令注册到一个链表中。其注册函数如下：
```c
void qmp_register_command(QmpCommandList *cmds, const char *name,
                          QmpCommandFunc *fn, QmpCommandOptions options)
{
    QmpCommand *cmd = g_malloc0(sizeof(*cmd));

    cmd->name = name;
    cmd->fn = fn;
    cmd->enabled = true;
    cmd->options = options;
    QTAILQ_INSERT_TAIL(cmds, cmd, node);
}
```
qemu-ga 会对所有支持的命令调用 `qmp_register_command`，即利用命令名称、处理函数以及命令参数构造一个 QmpCommand 结构然后插入 QmpCommandList 链表上。

在程序的初始化和 qmp 命令链表构造完成之后，qemu-ga 将开始运行，其主要调用 run_agent 函数。
```c
static int run_agent(GAState *s, GAConfig *config, int socket_activation)
{
    ga_state = s;

    ...

    config->blacklist = ga_command_blacklist_init(config->blacklist);
    if (config->blacklist) {
        GList *l = config->blacklist;
        s->blacklist = config->blacklist;
        do {
            g_debug("disabling command: %s", (char *)l->data);
            qmp_disable_command(&ga_commands, l->data);
            l = g_list_next(l);
        } while (l);
    }

    ...

    // 注册 process_event 函数
    json_message_parser_init(&s->parser, process_event);

    ...

    s->main_loop = g_main_loop_new(NULL, false);

    // 通道初始化
    if (!channel_init(ga_state, config->method, config->channel_path,
                      socket_activation ? FIRST_SOCKET_ACTIVATION_FD : -1)) {
        g_critical("failed to initialize guest agent channel");
        return EXIT_FAILURE;
    }
#ifndef _WIN32
    g_main_loop_run(ga_state->main_loop);
#else
    ...
#endif

    return EXIT_SUCCESS;
}
```
在解析的参数中会把禁止调用的 qga 命令加入到 `config->blacklist`，然后会调用 `qmp_disable_command` 把 `config->blacklist` 中的命令禁止掉（QmpCommand 中的 enabled 改为 false）。接着调用 `json_message_parser_init` 将 `s->parser` 的 emit 函数设置为 process_event，改函数主要用处处理请求。最后创建一个 main_loop，对通道进行初始化，调用 g_main_loop_run 让 qemu-ga 进程进入到事件监听循环中。

上述流程中通道初始化的流程如下：
```c
static gboolean channel_init(GAState *s, const gchar *method, const gchar *path,
                             int listen_fd)
{
    GAChannelMethod channel_method;

    if (strcmp(method, "virtio-serial") == 0) {
        s->virtio = true; /* virtio requires special handling in some cases */
        channel_method = GA_CHANNEL_VIRTIO_SERIAL;
    } else if (strcmp(method, "isa-serial") == 0) {
        channel_method = GA_CHANNEL_ISA_SERIAL;
    } else if (strcmp(method, "unix-listen") == 0) {
        channel_method = GA_CHANNEL_UNIX_LISTEN;
    } else if (strcmp(method, "vsock-listen") == 0) {
        channel_method = GA_CHANNEL_VSOCK_LISTEN;
    } else {
        g_critical("unsupported channel method/type: %s", method);
        return false;
    }

    s->channel = ga_channel_new(channel_method, path, listen_fd,
                                channel_event_cb, s);
    if (!s->channel) {
        g_critical("failed to create guest agent channel");
        return false;
    }

    return true;
}
```
初始化通道首先会根据传入的参数来决定通道的类型，然后调用 `ga_channel_new` 创建一个通道，其本质是调用 `ga_channel_open` 函数打开 virtio 串口设备，得到该设备的 fd，然后调用 `ga_channel_client_add` 将 fd 加入到 qemu-ga 的事件循环中。当该 fd 有数据到达时，会调用 `ga_channel_client_event`，该函数最终会调用到 channel_event_cb 函数，函数代码如下：
```c
static gboolean channel_event_cb(GIOCondition condition, gpointer data)
{
    GAState *s = data;
    gchar buf[QGA_READ_COUNT_DEFAULT+1];
    gsize count;
    // 接收数据
    GIOStatus status = ga_channel_read(s->channel, buf, QGA_READ_COUNT_DEFAULT, &count);
    switch (status) {
    case G_IO_STATUS_ERROR:
        g_warning("error reading channel");
        return false;
    case G_IO_STATUS_NORMAL:  // 接收到正常数据
        buf[count] = 0;
        g_debug("read data, count: %d, data: %s", (int)count, buf);
        json_message_parser_feed(&s->parser, (char *)buf, (int)count);
        break;
    case G_IO_STATUS_EOF:
        g_debug("received EOF");
        if (!s->virtio) {
            return false;
        }
        /* fall through */
    case G_IO_STATUS_AGAIN:
        /* virtio causes us to spin here when no process is attached to
         * host-side chardev. sleep a bit to mitigate this
         */
        if (s->virtio) {
            usleep(100*1000);
        }
        return true;
    default:
        g_warning("unknown channel read status, closing");
        return false;
    }
    return true;
}
```
该函数中，如果接收到正常数据，则会调用 `json_message_parser_feed` 来解析传过来的 json 数据并调用 `s->parser` 的 emit 函数（process_event），其核心代码如下：
```c
static void process_event(JSONMessageParser *parser, GQueue *tokens)
{
    GAState *s = container_of(parser, GAState, parser);
    QDict *qdict;
    Error *err = NULL;
    int ret;

    g_assert(s && parser);

    g_debug("process_event: called");
    qdict = qobject_to(QDict, json_parser_parse_err(tokens, NULL, &err));

    ...

    /* handle host->guest commands */
    if (qdict_haskey(qdict, "execute")) {
        process_command(s, qdict);
    } else {
        ...
    }

    qobject_unref(qdict);
}
```
上述中的 process_command 会调用 qmp_dispatch：
```c
static void process_command(GAState *s, QDict *req)
{
    QObject *rsp = NULL;
    int ret;

    g_assert(req);
    g_debug("processing command");
    rsp = qmp_dispatch(&ga_commands, QOBJECT(req));
    if (rsp) {
        ret = send_response(s, rsp);
        if (ret < 0) {
            g_warning("error sending response: %s", strerror(-ret));
        }
        qobject_unref(rsp);
    }
}
```
该函数调用 qmp_dispatch，会在 QmpCommandList 链表中找到要调用的命令与函数来执行，并将结果返回，接着则调用 send_response 返回数据。

# qemu-ga qemu 侧原理
命令行 `-device virtio-serial-pci` 会创建 `virtio-serial` 对应的类型为 `virtio-serial-pci` 的 pci 代理设备，其实例初始化函数为 `virtio_serial_pci_instance_init`。
```c
// hw/virtio/virtio-pci.c
static void virtio_serial_pci_instance_init(Object *obj)
{
   VirtIOSerialPCI *dev = VIRTIO_SERIAL_PCI(obj);
 
   virtio_instance_init_common(obj, &dev->vdev, sizeof(dev->vdev),
                               TYPE_VIRTIO_SERIAL);
}
```
该函数主要调用 `virtio_instance_init_common` 来创建一个 `virtio-serial-device` 设备。  
在 qemu 的 main 函数中，有如下步骤
```c
// vl.c
/* init generic devices */
rom_set_order_override(FW_CFG_ORDER_OVERRIDE_DEVICE);
if (qemu_opts_foreach(qemu_find_opts("device"),
                        device_init_func, NULL, NULL)) {
    exit(1);
}
```
其中 `device_init_func` 会将 `virtio-serical-pci` 设备具现化，调用的函数为 `virtio_serial_pci_realize`.
```c
// hw/virtio/virtio-pci.c
static void virtio_serial_pci_realize(VirtIOPCIProxy *vpci_dev, Error **errp)
{
    VirtIOSerialPCI *dev = VIRTIO_SERIAL_PCI(vpci_dev);
    DeviceState *vdev = DEVICE(&dev->vdev);
    DeviceState *proxy = DEVICE(vpci_dev);
    char *bus_name;

    ...

    qdev_set_parent_bus(vdev, BUS(&vpci_dev->bus));
    object_property_set_bool(OBJECT(vdev), true, "realized", errp);
}
```
接着调用 `virtio_serial_device_realize` 具现化 `virtio-serial-device` 设备，其代码如下：
```c
// hw/char/virtio-serial-bus.c
static void virtio_serial_device_realize(DeviceState *dev, Error **errp)
{
    VirtIODevice *vdev = VIRTIO_DEVICE(dev);
    VirtIOSerial *vser = VIRTIO_SERIAL(dev);
    uint32_t i, max_supported_ports;
    size_t config_size = sizeof(struct virtio_console_config);

    ...

    virtio_init(vdev, "virtio-serial", VIRTIO_ID_CONSOLE,
                config_size);

    /* Spawn a new virtio-serial bus on which the ports will ride as devices */
    qbus_create_inplace(&vser->bus, sizeof(vser->bus), TYPE_VIRTIO_SERIAL_BUS,
                        dev, vdev->bus_name);
    qbus_set_hotplug_handler(BUS(&vser->bus), DEVICE(vser), errp);
    vser->bus.vser = vser;
    QTAILQ_INIT(&vser->ports);

    vser->bus.max_nr_ports = vser->serial.max_virtserial_ports;
    vser->ivqs = g_malloc(vser->serial.max_virtserial_ports
                          * sizeof(VirtQueue *));
    vser->ovqs = g_malloc(vser->serial.max_virtserial_ports
                          * sizeof(VirtQueue *));

    /* Add a queue for host to guest transfers for port 0 (backward compat) */
    vser->ivqs[0] = virtio_add_queue(vdev, 128, handle_input);
    /* Add a queue for guest to host transfers for port 0 (backward compat) */
    vser->ovqs[0] = virtio_add_queue(vdev, 128, handle_output);
    /* control queue: host to guest */
    vser->c_ivq = virtio_add_queue(vdev, 32, control_in);
    /* control queue: guest to host */
    vser->c_ovq = virtio_add_queue(vdev, 32, control_out);

    for (i = 1; i < vser->bus.max_nr_ports; i++) {
        /* Add a per-port queue for host to guest transfers */
        vser->ivqs[i] = virtio_add_queue(vdev, 128, handle_input);
        /* Add a per-per queue for guest to host transfers */
        vser->ovqs[i] = virtio_add_queue(vdev, 128, handle_output);
    }

    vser->ports_map = g_malloc0((DIV_ROUND_UP(vser->serial.max_virtserial_ports, 32))
        * sizeof(vser->ports_map[0]));
    
    ...

    QLIST_INSERT_HEAD(&vserdevices.devices, vser, next);
}
```
上述代码中 `virtio_init` 初始化 `virtio-serial-device` 设备，调用 `qbus_create_inplace` 函数创建一条 virtio 串行总线，该总线上可以挂 virtio 串口设备，分配 virtio serial device 自己的 virtqueue（`vser->c_ivq` 和 `vser->c_ovq`） 来控制 virtioserialdevice 设备，分配并初始化 virtio 串口设备的 virtqueue（即 `vser->ivqs` 数组和 `vser->ovqs` 数组）来进行数据传输。  

从虚拟机到宿主机的 virtqueue 的处理函数是 handle_output，从宿主机到虚拟机的 virtqueue 的处理函数是 handle_input，handle_input 只在特殊情况下调用，如虚拟机由于长时间不读取 virtio 串口的数据，导致宿主机不能写，当虚拟机读取了一部分数据之后，就会调用 handle_input 通知宿主机继续写。
