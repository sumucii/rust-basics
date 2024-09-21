fn main() {
    //control_flow的作用以及完整的代码示例
    //控制流是编程语言中用于控制程序执行顺序的机制
    //在Rust中，控制流主要包括if表达式、循环和match表达式
    //下面是一个简单的控制流示例：（每一行后添加中文注释）
    let number = 3; //定义一个变量number
    if number < 5 { //如果number小于5
        println!("{} is less than 5", number); //打印number小于5
    } else { //否则
        println!("{} is greater than or equal to 5", number); //打印number大于等于5
    }
    //定义一个数组
    let a = [10, 20, 30, 40, 50]; //定义一个数组a
    //使用for循环遍历数组
    for element in a.iter() { //使用for循环遍历数组
        println!("The value is: {}", element); //打印数组元素的值
    }
    //使用match表达式匹配值
    let x = 5; //定义一个变量x
    match x { //使用match表达式匹配x的值
        1 => println!("One"), //匹配1时打印One
        2 => println!("Two"), //匹配2时打印Two
        3 => println!("Three"), //匹配3时打印Three
        _ => println!("Other"), //其他情况打印Other
    }
}