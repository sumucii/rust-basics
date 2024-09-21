fn main() {
    //函数是一种用于封装可重用代码的机制
    //在Rust中，函数使用fn关键字定义
    //函数可以有参数和返回值
    //函数的参数和返回值类型可以通过类型推导省略

    //调用add函数，并打印结果
    let result = add(1, 2);
    println!("Result: {}", result);

    //调用sub函数，并打印结果
    let result = sub(3, 2);
    println!("Result: {}", result);

    //调用mul函数，并打印结果
    let result = mul(2, 3);
    println!("Result: {}", result);

    //调用div函数，并打印结果
    let result = div(6, 3);
    println!("Result: {}", result);

    //调用modulus函数，并打印结果
    let result = modulus(7, 3);
    println!("Result: {}", result);

    //调用power函数，并打印结果
    let result = power(2, 3);
    println!("Result: {}", result);

    //调用factorial函数，并打印结果
    let result = factorial(5);
    println!("Result: {}", result);

    //调用fibonacci函数，并打印结果
    let result = fibonacci(10);
    println!("Result: {}", result);

    //调用parse_int函数，并处理返回的Result类型
    match parse_int("42") {
        Ok(n) => println!("Parsed integer: {}", n),
        Err(e) => eprintln!("Error: {}", e),
    }
}

//定义一个add函数，用于计算两个整数的和
fn add(x: i32, y: i32) -> i32 {
    x + y //返回x和y的和
}

//定义一个sub函数，用于计算两个整数的差
fn sub(x: i32, y: i32) -> i32 {
    x - y //返回x和y的差
}

//定义一个mul函数，用于计算两个整数的积
fn mul(x: i32, y: i32) -> i32 {
    x * y //返回x和y的积
}

//定义一个div函数，用于计算两个整数的商
fn div(x: i32, y: i32) -> i32 {
    x / y //返回x和y的商
}

//定义一个modulus函数，用于计算两个整数的余数
fn modulus(x: i32, y: i32) -> i32 {
    x % y //返回x和y的余数
}

//定义一个power函数，用于计算一个整数的幂
fn power(x: i32, y: u32) -> i32 {
    x.pow(y) //返回x的y次幂
}

//定义一个factorial函数，用于计算一个整数的阶乘
fn factorial(x: u32) -> u32 {
    if x == 0 {
        1 //0的阶乘为1
    } else {
        x * factorial(x - 1) //递归计算x的阶乘
    }
}

//定义一个fibonacci函数，用于计算斐波那契数列的第n项
fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        0 //当n等于0时，返回0
    } else if n == 1 {
        1 //当n等于1时，返回1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2) //递归计算斐波那契数列的第n项
    }
}

//定义一个parse_int函数，用于将字符串解析为整数
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>() //尝试将字符串解析为整数
}