### 测试 logging 调用
```py
with mock.patch("logging.warn") as mock_warn:
    mock_warn.assert_called_once_with("logging warn")
    mock_warn.assert_called_with("logging warn ")
```
