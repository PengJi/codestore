# best practices
1. 在函数内定义局部变量时，指定 `local`；
2. 定义全局变量时，指定`readonly`；
3. 尽量使用 `[[]]` 来代替 `[]`；
4. 简单的 `if` 尽量使用` && ||` 写成单行，比如 `[[ x > 2]] && echo x`；
5. 利用 `/dev/null` 过滤不友好的输出信息；
6. 利用命令的返回值判断命令的执行情况；
7. 使用文件前要判断文件是否存在，否则做好异常处理；
8. 使用 `mktemp` 生成临时文件或文件夹，
   参考：[mktemp 命令和 trap 命令教程](http://www.ruanyifeng.com/blog/2019/12/mktemp.html)；
9. 会使用 `trap` 捕获信号，并在接受到终止信号时执行一些收尾工作，例如：`trap 'rm -f "$TMPFILE"' EXIT`，
   参考：[trap信号捕捉用法详解](https://www.junmajinlong.com/shell/script_course/shell_trap/)；


# 指定解释器
当以 ./file.py 方式执行的时候，会从以下命令行中寻找解释器
```sh
#!/bin/bash
#!/usr/bin/env bash 
#!/usr/bin/env python
```


# 控制 shell 执行方式
`set -e`
如果脚本中任意命令执行失败（具有非零的退出状态），则 bash 立即退出。  
这种行为同编程语言类似，如：python、c 等，其中任何语句执行失败，则程序立即退出，不执行后续命令。  
默认情况下，在执行脚本时若遇到错误命令，则 bash 会继续执行后面命令，整个脚本退出码是成功的，这种行为容易忽略错误。
```sh
#!/bin/bash
set -e

hello  # 立即退出
echo "Hello"
```

`set -x`
将所有执行的命令都打印到终端。  
典型的用法：在执行脚本时，可以显示每个执行步骤。
```sh
#!/bin/bash
set -x

if [[ -f "/dev/sr0" ]]; then  # 输出控制流
    echo "hello"
else
    echo "world"
fi

# 输出：
# + [[ -f /dev/sr0 ]]
# + echo world
# world
```

`set -u`
执行脚本时，除了 $* 和 $@，遇到任何未定义的变量都会报错并立即退出。  
这种行为同编程语言类似，如：python、c 等，变量必须先定义再引用。
```sh
#!/usr/bin/env bash
echo $test  # 输出空

set -u
echo $hello  # 报错：hello: unbound variable
```

`set -o pipefail`
如果管道中任何命令失败，则失败命令的返回码将作为整个管道的返回码。  
默认情况下，管道的最后一条命令是管道的返回码。
```sh
#!/usr/bin/env bash
grep some-string /non/exist/file | sort
echo $?
# grep 的错误码为 2，错误信息写入 stderr，stdout 为空，则 sort 的输入为空，可正确执行，整条命令管道的返回码为 0。

set -o pipefail

grep some-string /non/exist/file | sort
echo $?
# 整条命令管道的返回码为 2。
```


# 输出重定向
```sh
# 禁止输出 stdout 和 stderr
>/dev/null 2>&1

# 输出到标准输出
> &1

# 输出到错误输出
> &2
```


# 数据结构

## 字符串
### 字符串包含
```sh
if [[ "hello world" =~ "hello" ]]; then
    echo "包含"
else
    echo "不包含"
fi
```

### 分割字符串
```sh
# 第一种方式
OLD_IFS=$IFS
IFS=","
for ntp_server in $1; do
    echo $ntp_server
done
IFS=$OLD_IFS

# 第二种方式
string="hello,shell,haha"
array=(${string//,/ }) 
for var in ${array[@]}
do
  echo $var
done

# 第三种方式
string="one,two,three,four,five"
array=(`echo $string | tr ',' ' '` ) 
for var in ${array[@]}
do
  echo $var
done
```

## 数组
### 遍历数组
```sh
OLD_IFS=$IFS
IFS=","
for dns_server in ${dns_servers}; do
    tmp_dns_servers+=($dns_server)
done
IFS=$OLD_IFS
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

if [[ -n "${TEAM_MASTER}" ]] && [[ ! "${DEVICETYPE}" = "TeamPort" ]]; then
    ./ifup-TeamPort ${CONFIG} $2
    ethtool_set
    exit 0
fi
```

## switch
```sh
case "$(get_network_config)" in
    rhel|centos||fedora)
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


# 其他
## 使用 heredocs
一种多行输入的方法。
```sh
cat>>/etc/rsyncd.conf << EOF
log file = /usr/local/logs/rsyncd.log
transfer logging = yes
log format = %t %a %m %f %b
syslog facility = local3
EOF
```
