# 函数定义
```sh
function get_network_config {
    echo "sysconfig"
    return
}
# 调用函数
$(get_network_config)
```

# 使用 main
```sh
#!/usr/bin/env bash
func1(){
    #do sth
}

func2(){
    #do sth
}

main(){
    func1
    func2
}
main "$@" 
```


# 逻辑运算符
```sh
"A ; B"  # Run A and then B, regardless of success of A
"A && B"  # Run B if A succeeded
"A || B"  # Run B if A failed
"A &"  # Run A in background.

false && echo howdy!
true && echo howdy!
howdy!

true || echo howdy!
false || echo howdy!
howdy!
```


# 重试
[How do I write a retry logic in script to keep retrying to run it upto 5 times?](https://unix.stackexchange.com/questions/82598/how-do-i-write-a-retry-logic-in-script-to-keep-retrying-to-run-it-upto-5-times)
```sh
for i in 1 2 3 4 5; do
    for eth_file in $(ls /sys/class/net/*/address); do
        if [[ "$1" == "$(cat $eth_file)" ]]; then
            interface=$(echo $eth_file | awk -F/ '{print $5}')
            break
        fi
    done
    [[ ! -z $interface ]] && break || sleep 1
done
```


# 校验命令是否执行成功
```sh
function check_cmd {
    "$@"
    local status=$?
    if [ $status -ne 0 ]; then
        echo "error with $@" >&2
        exit $status
    fi
}
check_cmd shell_command
```


# 解析 yaml 文件
```sh
function parse_yaml {
    local yaml_file=$1
    local parse_conf_result=$2
    local s
    local w
    local fs

    s='[[:space:]]*'
    w='[a-zA-Z0-9_.-]*'
    fs="$(echo @|tr @ '\034')"

    (
        sed -e '/- [^\�~@~\]'"[^\']"'.*: /s|\([ ]*\)- \([[:space:]]*\)|\1-\'$'\n''  \1\2|g' |

        sed -ne '/^--/s|--||g; s|\"|\\\"|g; s/[[:space:]]*$//g;' \
            -e 's/\$/\\\$/g' \
            -e "/#.*[\"\']/!s| #.*||g; /^#/s|#.*||g;" \
            -e "s|^\($s\)\($w\)$s:$s\"\(.*\)\"$s\$|\1$fs\2$fs\3|p" \
            -e "s|^\($s\)\($w\)${s}[:-]$s\(.*\)$s\$|\1$fs\2$fs\3|p" |

        awk -F"$fs" '{
            indent = length($1)/2;
            # if (length($2) == 0) { conj[indent]="+";} else {conj[indent]="";}
            vname[indent] = $2;
            for (i in vname) {if (i > indent) {delete vname[i]}}
                if (length($3) > 0) {
                    vn=""; for (i=0; i<indent; i++) {vn=(vn)(vname[i])(";")}
                    print vn, $2, $3 > "'$parse_conf_result'"
                }
            }' |

        sed -e 's/_=/+=/g' |

        awk 'BEGIN {
                FS="=";
                OFS="="
            }
            /(-|\.).*=/ {
                gsub("-|\\.", "_", $1)
            }
            { print }'
    ) < "$yaml_file"
}
# usage
parse_yaml source.yaml parse_result
```sh
