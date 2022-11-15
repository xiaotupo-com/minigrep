# Rust 官网 minigrep 例子项目

## poem.txt 
该文件中有几行文本，作用是让我们写的程序来使用。

## 运行程序

```shell
cargo run tell poem.txt
```
会输出 `poem.txt` 中包含 tell 的行内容，可以设置环境变量 `CASE_INSENSITIVE` 让程序忽略大小写。
