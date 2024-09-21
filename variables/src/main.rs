fn main() {
    //variables的代码示例

    //定义一个变量
    let x = 5;
    //打印变量的值
    println!("x: {}", x); //打印"x: 5"

    //定义一个可变变量
    let mut y = 10;
    //修改可变变量的值
    y = 20;
    //打印可变变量的值
    println!("y: {}", y); //打印"y: 20"

    //定义一个常量
    const Z: i32 = 100;
    //打印常量的值
    println!("Z: {}", Z); //打印"Z: 100"

    //定义一个隐藏的变量
    //隐藏的作用：在同一作用域内，隐藏变量可以隐藏同名的变量，但不能隐藏同名的可变变量或常量
    let x = "Hello, Rust!";
    //打印隐藏的变量的值
    println!("x: {}", x); //打印"x: Hello, Rust!"

    //定义一个隐藏的可变变量
    let mut y = "Hello, World!";
    //修改隐藏的可变变量的值
    y = "Hello, Rust!";
    //打印隐藏的可变变量的值
    println!("y: {}", y); //打印"y: Hello, Rust!"

    //定义一个隐藏的常量
    const Z: &str = "Hello, Rust!";
    //打印隐藏的常量的值
    println!("Z: {}", Z); //打印"Z: Hello, Rust!"
}