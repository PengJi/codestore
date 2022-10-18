# grep
搜索目标行和前后 10 行
`grep -10 dd.txt`

避免出现一些不可预知的问题使用 `-w`
`grep -w "a sentence" txt`


# sed
替换
```sh
# 将 `/dev/sd` 替换为 `/dev/vd`，用 `#` 当标识符 
sed -i "s#/dev/sd#/dev/vd#g" /etc/fstab
# 选项：
# -i 修改源文件 
```

删除
```sh
# 删除匹配行
sed -i '/%cdrom_caption%/d' /tmp/file
```


# awk
获取最后一列
`awk -F',' '{print $NF}'`

获取倒数第二列
`awk '{print $(NF-1)}'`


# 正则表达式
示例：有效的电话号码，格式为 `(xxx) xxx-xxxx` 或 `xxx-xxx-xxxx`（x 表示一个数字），比如：
```sh
987-123-4567  # 有效
123 456 7890  # 无效
(123) 456-7890  # 有效
```
shell 
```sh
grep -P '^([0-9]{3}-|\([0-9]{3}\) )[0-9]{3}-[0-9]{4}$' file.txt

awk '/^([0-9]{3}-|\([0-9]{3}\) )[0-9]{3}-[0-9]{4}$/' file.txt
```