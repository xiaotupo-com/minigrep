use std::env;
use std::process;

use minigrep::{Config, run};


fn main() {

    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // 解析命令行参数
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("解析时出现错误: {}", err);
        process::exit(1);
    });

    // 错误处理
    if let Err(e) = run(config) {
        eprintln!("读取出现错误： {}", e);
        process::exit(1);
    }
}

