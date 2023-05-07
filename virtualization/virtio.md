# virtio-blk VS virtio-scsi
virtio-blk:
```
guest: app -> Block Layer -> virtio-blk
host: QEMU -> Block Layer -> Block Device Driver -> Hardware
```

virtio-scsi(using QEMU as SCSI target):
```
guest: app -> Block Layer -> SCSI Layer -> scsi_mod
host: QEMU -> Block Layer -> SCSI Layer -> Block Device Driver -> Hardware
```

看一个实际 VM 的例子（libvirt 的 xml）：
```xml
<devices>
    <emulator>/usr/libexec/qemu-kvm</emulator>
    <!-- virtio-blk 磁盘，使用 iscsi 协议，占用 pci 插槽 -->
    <disk type='network' device='disk'>
      <driver name='qemu' type='raw' cache='none' io='native'/>
      <source protocol='iscsi' name='iqn.2016-02.com.xxx:system:zbs-iscsi-datastore-1661482810725b/135'>
        <host name='127.0.0.1' port='3260'/>
        <initiator>
          <iqn name='iqn.2013-11.org.xxx:b28ae9a7-6ce9-433f-a69b-b5af8fec1eae.513f7ae2-d4e5-11ec-9890-b4055d3205f8.0'/>
        </initiator>
      </source>
      <target dev='vda' bus='virtio'/>
      <iotune>
        <group_name></group_name>
      </iotune>
      <serial>c4afbb62-bbb6-3b20-a8a5-9bb66fe0ddcb</serial>
      <boot order='1'/>
      <alias name='virtio-disk0'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x0a' function='0x0'/>
    </disk>
    <!-- virtio-blk 磁盘，使用 iscsi 协议，占用 pci 插槽 -->
    <disk type='network' device='disk'>
      <driver name='qemu' type='raw' cache='none' io='native'/>
      <source protocol='iscsi' name='iqn.2016-02.com.xxx:system:zbs-iscsi-datastore-1673338750834b/73'>
        <host name='127.0.0.1' port='3260'/>
        <initiator>
          <iqn name='iqn.2013-11.org.xxx:b28ae9a7-6ce9-433f-a69b-b5af8fec1eae.513f7ae2-d4e5-11ec-9890-b4055d3205f8.0'/>
        </initiator>
      </source>
      <target dev='vdb' bus='virtio'/>
      <iotune>
        <group_name></group_name>
      </iotune>
      <serial>45aabfca-d152-3fe6-b186-6ee02aceda54</serial>
      <boot order='4'/>
      <alias name='virtio-disk1'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x0c' function='0x0'/>
    </disk>
    <!-- virtio-scsi 磁盘，使用 iscsi 协议 -->
    <disk type='network' device='disk'>
      <driver name='qemu' type='raw' cache='none' io='native'/>
      <source protocol='iscsi' name='iqn.2016-02.com.xxx:system:zbs-iscsi-datastore-1652931133814c/247'>
        <host name='127.0.0.1' port='3260'/>
        <initiator>
          <iqn name='iqn.2013-11.org.xxx:b28ae9a7-6ce9-433f-a69b-b5af8fec1eae.513f7ae2-d4e5-11ec-9890-b4055d3205f8.0'/>
        </initiator>
      </source>
      <target dev='sda' bus='scsi'/>
      <iotune>
        <group_name></group_name>
      </iotune>
      <serial>00d36d31-9519-3fc5-bc0f-0b7cddffe795</serial>
      <wwn>24dc007f7f965d3e</wwn>
      <vendor>xxx</vendor>
      <product>xxx OS</product>
      <boot order='2'/>
      <alias name='scsi0-0-0-0'/>
      <address type='drive' controller='0' bus='0' target='0' unit='0'/>
    </disk>
    <!-- virtio-scsi 磁盘，使用 iscsi 协议 -->
    <disk type='network' device='disk'>
      <driver name='qemu' type='raw' cache='none' io='native'/>
      <source protocol='iscsi' name='iqn.2016-02.com.xxx:system:zbs-iscsi-datastore-1673336489316g/99'>
        <host name='127.0.0.1' port='3260'/>
        <initiator>
          <iqn name='iqn.2013-11.org.xxx:b28ae9a7-6ce9-433f-a69b-b5af8fec1eae.513f7ae2-d4e5-11ec-9890-b4055d3205f8.0'/>
        </initiator>
      </source>
      <target dev='sdb' bus='scsi'/>
      <iotune>
        <group_name></group_name>
      </iotune>
      <serial>5ab63453-07a5-303e-b34d-fef0aae58da0</serial>
      <wwn>24d2007f7fdfb583</wwn>
      <vendor>xxx</vendor>
      <product>xxx OS</product>
      <boot order='3'/>
      <alias name='scsi0-0-0-1'/>
      <address type='drive' controller='0' bus='0' target='0' unit='1'/>
    </disk>
    <!-- ide cdrom -->
    <disk type='file' device='cdrom'>
      <driver name='qemu' type='raw'/>
      <source file='/usr/share/xxx/images/6205cd6d-2bb2-4044-9b90-699e409617e8'/>
      <backingStore/>
      <target dev='hda' bus='ide'/>
      <readonly/>
      <boot order='5'/>
      <alias name='ide0-0-0'/>
      <address type='drive' controller='0' bus='0' target='0' unit='0'/>
    </disk>
    <!-- ide cdrom -->
    <disk type='file' device='cdrom'>
      <driver name='qemu'/>
      <target dev='hdb' bus='ide'/>
      <readonly/>
      <boot order='6'/>
      <alias name='ide0-0-1'/>
      <address type='drive' controller='0' bus='0' target='0' unit='1'/>
    </disk>
    <!-- usb 相关 controller，占用 pci 插槽 -->
    <controller type='usb' index='0' model='piix3-uhci'>
      <alias name='usb'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x01' function='0x2'/>
    </controller>
    <controller type='usb' index='1' model='nec-xhci' ports='15'>
      <alias name='usb1'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x05' function='0x0'/>
    </controller>
    <controller type='usb' index='2' model='ehci'>
      <alias name='usb2'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x06' function='0x0'/>
    </controller>
    <controller type='usb' index='3' model='piix3-uhci'>
      <alias name='usb3'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x07' function='0x0'/>
    </controller>
    <!-- 串口设备 controller，占用 pci 插槽 -->
    <controller type='virtio-serial' index='0' ports='2'>
      <alias name='virtio-serial0'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x08' function='0x0'/>
    </controller>
    <!-- 网络设备 controller，占用 pci 插槽 -->
    <controller type='pci' index='1' model='pci-bridge'>
      <model name='pci-bridge'/>
      <target chassisNr='1'/>
      <alias name='pci.1'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x03' function='0x0'/>
    </controller>
    <!-- scsi controller，占用 pci 插槽 -->
    <controller type='scsi' index='0' model='virtio-scsi'>
      <alias name='scsi0'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x09' function='0x0'/>
    </controller>
    <!-- pci root -->
    <controller type='pci' index='0' model='pci-root'>
      <alias name='pci.0'/>
    </controller>
    <!-- ide controller，占用 pci 插槽 -->
    <controller type='ide' index='0'>
      <alias name='ide'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x01' function='0x1'/>
    </controller>
    <!-- 网卡，占用 pci 插槽 -->
    <interface type='bridge'>
      <mac address='52:54:00:80:2e:36'/>
      <source bridge='ovsbr-mgt'/>
      <vlan>
        <tag id='0'/>
      </vlan>
      <virtualport type='openvswitch'>
        <parameters interfaceid='9f11f072-00da-40cf-b925-35a3b4072972' profileid='c8a1e42d-e0f3-4d50-a190-53209a98f157'/>
      </virtualport>
      <target dev='vnet415'/>
      <model type='virtio'/>
      <driver name='vhost' queues='4' rx_queue_size='1024'/>
      <link state='up'/>
      <alias name='net0'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x04' function='0x0'/>
    </interface>
    <!-- 其他设备，暂时不关注-->
    <serial type='pty'>
      <source path='/dev/pts/6'/>
      <target type='isa-serial' port='1'>
        <model name='isa-serial'/>
      </target>
      <alias name='serial0'/>
    </serial>
    <serial type='file'>
      <source path='/usr/share/xxx/vm/serial/b28ae9a7-6ce9-433f-a69b-b5af8fec1eae.log'/>
      <target type='isa-serial' port='0'>
        <model name='isa-serial'/>
      </target>
      <alias name='serial1'/>
    </serial>
    <console type='pty'>
      <source path='/dev/pts/6'/>
      <target type='serial' port='1'/>
      <alias name='serial0'/>
    </console>
    <channel type='unix'>
      <source mode='bind' path='/var/lib/libvirt/qemu/channel/target/domain-1568-b28ae9a7-6ce9-433f-a/org.qemu.guest_agent.0'/>
      <target type='virtio' name='org.qemu.guest_agent.0' state='connected'/>
      <alias name='channel0'/>
      <address type='virtio-serial' controller='0' bus='0' port='1'/>
    </channel>
    <input type='mouse' bus='ps2'>
      <alias name='input0'/>
    </input>
    <input type='tablet' bus='usb'>
      <alias name='input1'/>
      <address type='usb' bus='0' port='1'/>
    </input>
    <input type='keyboard' bus='ps2'>
      <alias name='input2'/>
    </input>
    <graphics type='vnc' port='5900' autoport='yes' listen='127.0.0.1'>
      <listen type='address' address='127.0.0.1'/>
    </graphics>
    <video>
      <model type='cirrus' vram='16384' heads='1' primary='yes'/>
      <alias name='video0'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x02' function='0x0'/>
    </video>
    <memballoon model='virtio'>
      <stats period='10'/>
      <alias name='balloon0'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x0b' function='0x0'/>
    </memballoon>
  </devices>
```
上面只列出了虚拟机的设备部分，该虚拟机有两块 virtio-blk 磁盘，可以看到所有的 virtio-scsi 使用同一个 controller，该 controller 占用一个 pci 插槽。
每个 virtio-blk 磁盘占用一个 pci 插槽



# 参考
[VIRTIO & VHOST](https://zhuanlan.zhihu.com/p/38370357)
[浅谈Linux设备虚拟化技术的演进之路](https://mp.weixin.qq.com/s/g1kt0DDJwDk2Lg56R5suKw)  
