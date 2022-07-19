# grep
搜索目标行和前后 10 行
`grep -10 dd.txt`


# sed
替换
```sh
# 将 `/dev/sd` 替换为 `/dev/vd`，用 `#` 当标识符 
sed -i "s#/dev/sd#/dev/vd#g" /etc/fstab
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