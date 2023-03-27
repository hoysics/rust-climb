use mini_grep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("解析参数出错: {}", err);
        process::exit(1)
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = mini_grep::run(config) {
        eprintln!("应用错误: {}", e);
        process::exit(1)
    }
}
