# best practices
1. 在函数内定义变量时，指定 `local`；


# 指定解释器路径
当以 ./file.py 方式执行的时候，会从以下命令行中寻找解释器
```sh
#!/usr/bin/env bash
#!/usr/bin/env python
```

# output redirection
```sh
# 禁止输出 stdout 和 stderr
>/dev/null 2>&1

# 输出到标准输出
> &1

# 输出到标准输入
> &2
```

# 控制流
## if
```sh
if [[ -f "/etc/os-release" ]]; then
    os_type=$(cat /etc/os-release | grep PRETTY_NAME)
    if [[ ${os_type} =~ "CentOS Linux 8" ]]; then
        echo "nm"
        return
    elif [[ ${os_type} =~ "ubuntu" ]]; then
        echo "ubuntu"
        return
    else
        echo "not found"
    fi
fi

if [ -n "${TEAM_MASTER}" ] && [ ! "${DEVICETYPE}" = "TeamPort" ] && [ -x ./ifup-TeamPort ]; then
    ./ifup-TeamPort ${CONFIG} $2
    ethtool_set
    exit 0
fi
```

# 数组
遍历数组
```sh
OLD_IFS=$IFS
IFS=","
for dns_server in ${dns_servers}; do
    tmp_dns_servers+=($dns_server)
done
IFS=$OLD_IFS
```


## switch
```sh
case "$(get_network_config)" in
    eni)
        by_eni $1 $2 $3 $4
        ;;
    sysconfig)
        by_sysconfig $1 $2 $3 $4
        ;;
    networkmanager)
        by_nm $1 $2 $3 $4
        ;;
    netplan)
        by_netplan $1 $2 $3 $4
        ;;
    *)
        echo "Unsupported network config system" >&2
        exit 1
        ;;
esac
```
