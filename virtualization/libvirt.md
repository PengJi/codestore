一些典型的 libvirt xml
```xml
<!-- usb cdrom -->
<?xml version="1.0" encoding="utf-8"?>
<disk type="file" device="cdrom">
  <driver name="qemu" type="raw"/>
  <target dev="sdzy" bus="usb"/>
  <source file="/usr/share/smartx/images/vmtools/706dca56-06cc-4554-b74f-a686c8f973fb"/>
  <readonly/>
  <boot order='999'/>
  <address type='usb' bus='0' port='3'/>
</disk>

<!-- scsi cdrom -->
<?xml version="1.0" encoding="utf-8"?>
<disk type="file" device="cdrom">
  <driver name="qemu" type="raw"/>
  <target dev="sdzy" bus="scsi"/>
  <source file="/usr/share/smartx/images/8079ce65-0f2a-4059-bff0-2ffd4e264440"/>
  <readonly/>
  <boot order='999'/>
  <address type='drive' controller='0' bus='0' target='0' unit='2'/>
</disk>

<!-- usb disk -->
<disk type='file' device='disk'>
  <driver name='qemu' type='raw'/>
  <source file='/usr/share/smartx/images/ae5d0ad2-ee8b-4be8-a5e9-dd55f8beee62'/>
  <target dev='sdzy' bus='usb'/>
  <readonly/>
  <boot order='4'/>
  <address type='usb' bus='0' port='3'/>
</disk>
```

热添加
```bash
virsh attach-device 01d3658c-b11c-4e87-8214-512377513b31 usb_cdrom.xml --current
```

通过 libvirt 执行 qmp
```bash
# 使用 qmp 热添加 cdrom
virsh qemu-monitor-command ca4152ab-b978-4594-be4f-b41bb2532146 --pretty '{ "execute": "__com.redhat_drive_add", "arguments": { "id": "usb_cdrom_fastio_drive_id", "file":"/usr/share/smartx/images/d3420652-67a2-4121-a489-7d415039395d","media":"cdrom"}}'
virsh qemu-monitor-command ca4152ab-b978-4594-be4f-b41bb2532146 --pretty '{ "execute": "device_add", "arguments": { "driver": "usb-storage", "drive": "usb_cdrom_fastio_drive_id","bus": "usb.0", "port": "1" }}'

# 使用 hmp 热添加 cdrom
virsh qemu-monitor-command ca4152ab-b978-4594-be4f-b41bb2532146 --pretty '{ "execute": "human-monitor-command", "arguments": { "command-line": "drive_add auto id=usb_cdrom_drive,if=none,file=/usr/share/smartx/images/d3420652-67a2-4121-a489-7d415039395d,media=cdrom" } }'
virsh qemu-monitor-command ca4152ab-b978-4594-be4f-b41bb2532146 --pretty '{ "execute": "human-monitor-command", "arguments": { "command-line": "device_add usb-storage,id=usb_cdrom_device,drive=usb_cdrom_drive,bus=usb.0,port=2" }}'

# 使用 qmp 热删除 cdrom
virsh qemu-monitor-command ca4152ab-b978-4594-be4f-b41bb2532146 --pretty '{ "execute": "human-monitor-command", "arguments": { "command-line": "drive_del usb_cdrom_drive" } }'
virsh qemu-monitor-command ca4152ab-b978-4594-be4f-b41bb2532146 --pretty '{ "execute": "human-monitor-command", "arguments": { "command-line": "device_del usb_cdrom_device" } }'
```
