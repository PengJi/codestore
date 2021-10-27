# grep
搜索目标行和前后 10 行：`grep -10 dd.txt`


# sed


# awk
获取最后一列：`awk -F',' '{print $NF}'`
获取倒数第二列：`awk '{print $(NF-1)}'`