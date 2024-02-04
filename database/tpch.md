下载 TPC-H
[https://www.tpc.org/tpc_documents_current_versions/current_specifications5.asp](https://www.tpc.org/tpc_documents_current_versions/current_specifications5.asp)

```bash
$ unzip TPC-H-Tool.zip
$ cd TPC-H_V3.0.1
```

修改 Makefile 文件中的 CC、DATABASE、MACHINE 和 WORKLOAD 等参数定义。
```bash
$ cp makefile.suite Makefile
$ vim Makefile
CC      = gcc  # 修改
Current values for DATABASE are: INFORMIX, DB2, TDAT (Teradata)
                                 SQLSERVER, SYBASE, ORACLE, VECTORWISE
Current values for MACHINE are:  ATT, DOS, HP, IBM, ICL, MVS,
                                 SGI, SUN, U2200, VMS, LINUX, WIN32
Current values for WORKLOAD are:  TPCH
DATABASE= MYSQL  # 修改
MACHINE = LINUX  # 修改
WORKLOAD = TPCH  # 修改
```

修改 tpcd.h 文件，并添加新的宏定义。
```bash
$ vim tpcd.h
#ifdef MYSQL
#define GEN_QUERY_PLAN ""
#define START_TRAN "START TRANSACTION"
#define END_TRAN "COMMIT"
#define SET_OUTPUT ""
#define SET_ROWCOUNT "limit %d;\n"
#define SET_DBASE "use %s;\n"
#endif
```

对文件进行编译。
```bash
make
```
编译完成后该目录下会生成以下两个可执行文件：
* dbgen：数据生成工具。在使用 InfiniDB 官方测试脚本进行测试时，需要用该工具生成 tpch 相关表数据。
* qgen：SQL 生成工具。生成初始化测试查询，由于不同的 seed 生成的查询不同，为了结果的可重复性，请使用附件提供的 22 个查询。

dbgen 命令可以生成指定大小的数据，生成环境测试建议不少于 1000G 。以 10G 为例。
```bash
$ ./dbgen -s 10
TPC-H Population Generator (Version 3.0.0)
Copyright Transaction Processing Performance Council 1994 - 2010
```

准备数据
```bash
mysql -uroot --local-infile

# 创建数据库
create database TPCD;
use TPCD;

# 创建表
\. /home/smartx/TPC-H_V3.0.1/dbgen/dss.ddl

# 创建索引、外键等
\. /home/smartx/TPC-H_V3.0.1/dbgen/dss.ri

# 将表名改为小写
rename table to customer;
rename table to lineitem;
rename table to nation;
rename table to orders;
rename table to part;
rename table to partsupp;
rename table to region;
rename table to supplier;

# 导入数据
load data local infile '/home/smartx/TPC-H_V3.0.1/dbgen/part.tbl' into table part fields terminated by '|';
load data local infile '/home/smartx/TPC-H_V3.0.1/dbgen/region.tbl' into table region fields terminated by '|';
load data local infile '/home/smartx/TPC-H_V3.0.1/dbgen/nation.tbl' into table nation fields terminated by '|';
load data local infile '/home/smartx/TPC-H_V3.0.1/dbgen/customer.tbl' into table customer fields terminated by '|';
load data local infile '/home/smartx/TPC-H_V3.0.1/dbgen/supplier.tbl' into table supplier fields terminated by '|';
load data local infile '/home/smartx/TPC-H_V3.0.1/dbgen/orders.tbl' into table orders fields terminated by '|';
load data local infile '/home/smartx/TPC-H_V3.0.1/dbgen/partsupp.tbl' into table partsupp fields terminated by '|';
load data local infile '/home/smartx/TPC-H_V3.0.1/dbgen/lineitem.tbl' into table lineitem fields terminated by '|';
```

执行测试
```bash
vim tpch-benchmark-olap.sh
#!/bin/bash
exec 3>&1 4>&2 1>> tpch-benchmark-olap-`date +'%Y%m%d%H%M%S'`.log 2>&1
I=1
II=3
while [ $I -le $II ]
do
N=1
T=23
while [ $N -lt $T ]
do
  if [ $N -lt 10 ] ; then
    NN='0'$N
  else
    NN=$N
  fi
  echo "query $NN starting"
  time mysql -uroot -DTPCD < ./queries/${NN}.sql
  echo "query $NN ended!"
  N=`expr $N + 1`
  echo -e
  echo -e
done
 I=`expr $I + 1`
done
```
