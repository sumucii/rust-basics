use std::cmp::Ordering;
use std::io;
use colored::*;
use rand::Rng;

//引入colored库
fn main() {
    //猜数字游戏，使用不同的色彩提示玩家，并在每一行后添加中文注释
    println!("{}", "Guess the number!".on_bright_purple()); //提示玩家猜数字

    //生成一个随机数
    let secret_number = rand::thread_rng().gen_range(1..101); //生成一个1到100的随机数
    println!("{}{}", "The secret number is: ".on_bright_yellow(), secret_number); //提示生成的随机数

    println!("{}", "Please input your guess.".on_bright_cyan()); //提示玩家输入猜测的数字

    loop {
        //创建一个可变变量guess，用来存储玩家的猜测
        let mut guess = String::new(); //创建一个可变变量guess，用来存储玩家的猜测

        //读取玩家的输入
        io::stdin().read_line(&mut guess) //读取玩家的输入
            .expect("Failed to read line"); //读取失败时的错误提示

        //将玩家的输入转换为数字
        let guess: u32 = match guess.trim().parse() { //将玩家的输入转换为数字
            Ok(num) => num, //转换成功时返回数字
            Err(_) => continue, //转换失败时继续循环
        };

        //提示玩家猜测的数字
        println!("{},{}", "You guessed: ".on_bright_blue(), guess); //提示玩家猜测的数字

        //比较玩家的猜测和随机数
        match guess.cmp(&secret_number) { //比较玩家的猜测和随机数
            Ordering::Less => println!("{}", "Too small!".on_bright_red()), //猜小了
            Ordering::Greater => println!("{}", "猜的太大了".on_bright_red()), //猜大了
            Ordering::Equal => { //猜对了
                println!("{}", "You win!".on_bright_green());
                break; //退出循环
            }
        }
    }
}