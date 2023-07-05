# EXTENSIONS
python
代码静态检查：flake8、pylance、ruff
```python
"python.linting.enabled": true,
"python.linting.pycodestyleEnabled": false,
"python.linting.flake8Enabled": true,
"python.linting.flake8Args": [
    "--max-line-length=120",
    "--ignore=C901"
],
```

代码格式化：black
```python
"python.formatting.provider": "black",
"python.formatting.blackArgs": [
    "--line-length=120"
],
```


[ruff rules](https://beta.ruff.rs/docs/rules/#mccabe-c90)  
