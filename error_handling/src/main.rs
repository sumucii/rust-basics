fn main() {
    //错误处理是一种用于处理程序运行时错误的机制
    //在Rust中，错误处理主要通过Result枚举类型和panic宏来实现
    //Result枚举类型包含两个变体：Ok和Err，分别表示操作成功和操作失败
    //panic宏用于在程序出现无法恢复的错误时终止程序的执行

    //下面是一个简单的错误处理示例：（每一行后添加中文注释）

    //调用parse_int函数，并处理返回的Result类型
    match parse_int("42") { //调用parse_int函数，并处理返回的Result类型
        Ok(n) => println!("Parsed integer: {}", n), //解析成功时打印整数
        Err(e) => eprintln!("Error: {}", e), //解析失败时打印错误信息
    }

    //调用parse_int函数，并处理返回的Result类型
    match parse_int("abc") { //调用parse_int函数，并处理返回的Result类型
        Ok(n) => println!("Parsed integer: {}", n), //解析成功时打印整数
        Err(e) => eprintln!("Error: {}", e), //解析失败时打印错误信息
    }

    //使用unwrap方法获取Result类型的值
    let n = parse_int("42").unwrap(); //使用unwrap方法获取Result类型的值
    println!("Parsed integer: {}", n); //打印解析的整数

    //使用expect方法获取Result类型的值
    let n = parse_int("abc").expect("Failed to parse integer"); //使用expect方法获取Result类型的值
    println!("Parsed integer: {}", n); //打印解析的整数

    //使用?运算符简化错误处理
    let n = parse_int("42")?; //使用?运算符简化错误处理
    println!("Parsed integer: {}", n); //打印解析的整数

    //使用?运算符简化错误处理
    let n = parse_int("abc")?; //使用?运算符简化错误处理
    println!("Parsed integer: {}", n); //打印解析的整数

    //使用unwrap_or方法处理解析失败的情况
    let n = parse_int("abc").unwrap_or(0); //使用unwrap_or方法处理解析失败的情况
    println!("Parsed integer: {}", n); //打印解析的整数

    //使用unwrap_or_else方法处理解析失败的情况
    let n = parse_int("abc").unwrap_or_else(|e| { //使用unwrap_or_else方法处理解析失败的情况
        eprintln!("Error: {}", e); //打印错误信息
        0 //返回默认值
    });
    println!("Parsed integer: {}", n); //打印解析的整数

    //使用map方法处理解析成功的情况
    let n = parse_int("42").map(|n| n * 2); //使用map方法处理解析成功的情况
    println!("Parsed integer: {:?}", n); //打印解析的整数

    //使用map_err方法处理解析失败的情况
    let n = parse_int("abc").map_err(|e| e.to_string()); //使用map_err方法处理解析失败的情况

    //使用and_then方法处理解析成功的情况
    let n = parse_int("42").and_then(|n| Ok(n * 2)); //使用and_then方法处理解析成功的情况
    println!("Parsed integer: {:?}", n); //打印解析的整数

    //使用or_else方法处理解析失败的情况
    let n = parse_int("abc").or_else(|e| { //使用or_else方法处理解析失败的情况
        eprintln!("Error: {}", e); //打印错误信息
        Ok(0) //返回默认值
    });
    println!("Parsed integer: {:?}", n); //打印解析的整数

    //使用unwrap_err方法获取解析失败的错误信息
    let e = parse_int("abc").unwrap_err(); //使用unwrap_err方法获取解析失败的错误信息
    eprintln!("Error: {}", e); //打印错误信息

    //使用is_ok方法判断解析是否成功
    let result = parse_int("42"); //调用parse_int函数
    if result.is_ok() { //判断解析是否成功
        println!("Parsed integer: {}", result.unwrap()); //打印解析的整数
    } else {
        eprintln!("Error: {}", result.unwrap_err()); //打印错误信息
    }

    //使用is_err方法判断解析是否失败
    let result = parse_int("abc"); //调用parse_int函数
    if result.is_err() { //判断解析是否失败
        eprintln!("Error: {}", result.unwrap_err()); //打印错误信息
    } else {
        println!("Parsed integer: {}", result.unwrap()); //打印解析的整数
    }

    //使用ok方法获取解析成功的值
    let result = parse_int("42"); //调用parse_int函数
    if let Some(n) = result.ok() { //获取解析成功的值
        println!("Parsed integer: {}", n); //打印解析的整数
    } else {
        eprintln!("Error: {}", result.unwrap_err()); //打印错误信息
    }

    //使用err方法获取解析失败的值
    let result = parse_int("abc"); //调用parse_int函数
    if let Some(e) = result.err() { //获取解析失败的值
        eprintln!("Error: {}", e); //打印错误信息
    } else {
        println!("Parsed integer: {}", result.unwrap()); //打印解析的整数
    }

    //使用unwrap_or_default方法处理解析失败的情况
    let n = parse_int("abc").unwrap_or_default(); //使用unwrap_or_default方法处理解析失败的情况
    println!("Parsed integer: {}", n); //打印解析的整数

    //使用unwrap_or_default方法处理解析失败的情况
    let n = parse_int("abc").unwrap_or_default(); //使用unwrap_or_default方法处理解析失败的情况
    println!("Parsed integer: {}", n); //打印解析的整数
}

//定义一个函数，用于将字符串解析为整数
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>() //尝试将字符串解析为整数
}