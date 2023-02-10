use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Stdin};
fn main() {
    println!("猜数字游戏 1.0");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("秘密数字是: {}", secret_number);
    loop {
        println!("请输入1个数字");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("读取用户输入错误");
        println!("你输入的数字是: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "to small".red()),
            Ordering::Greater => println!("{}", "to big".red()),
            Ordering::Equal => {
                println!("{}", "you win".green());
                break;
            }
        }
    }
}
