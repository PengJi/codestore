### 抛出异常
```py
with pytest.raises(SVTInvalidArgument) as excinfo:
    SVTPropertiesWrapper(vm_uuid, conn, "192.168.1.1").update_svt_properties(
        properties["properties"], ["hostname"], ""
    )
assert "Hostname can not be empty" in str(excinfo.value)
assert excinfo.value.user_code = ERROR_CODE
```