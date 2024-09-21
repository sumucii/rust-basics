fn main() {
    //泛型是一种用于编写可重用代码的机制
    //在Rust中，泛型通过类型参数来实现
    //泛型可以应用于函数、结构体、枚举等各种类型
    //泛型可以用于定义多种类型的函数、结构体、枚举等
    //泛型可以通过类型参数来指定具体的类型

    //调用泛型函数
    print(42); //调用泛型函数print，传入整数42
    print("hello"); //调用泛型函数print，传入字符串"hello"

    //定义一个泛型结构体
    let point = Point { x: 1, y: 2 }; //定义一个Point结构体，x和y字段的类型为整数
    println!("{:?}", point); //打印point的值

    //定义一个泛型枚举
    let some = Option::Some(42); //定义一个Some变体，包含整数42
    let none = Option::None; //定义一个None变体
    println!("{:?}, {:?}", some, none); //打印some和none的值

    //调用泛型trait
    let point = Point { x: 1, y: 2 }; //定义一个Point结构体
    point.method(42); //调用Point结构体的method方法，传入整数42

    //调用泛型函数
    let result = add(1, 2); //调用add函数，传入整数1和2
    println!("Result: {}", result); //打印add函数的结果

    let result = sub(3.3, 2.2); //调用sub函数，传入整数3和2
    println!("Result: {}", result); //打印sub函数的结果

    let result = mul(2.3, 3.2); //调用mul函数，传入整数2和3
    println!("Result: {}", result); //打印mul函数的结果

    let result = div(6, 3); //调用div函数，传入整数6和3
    println!("Result: {}", result); //打印div函数的结果
}

//定义一个泛型函数
fn print<T>(value: T) { //定义一个泛型函数print，接收一个类型为T的参数value
    println!("{:?}", value); //打印参数value的值
}

//定义一个泛型结构体
struct Point<T> { //定义一个泛型结构体Point，包含一个类型为T的字段
    x: T, //定义一个类型为T的字段x
    y: T, //定义一个类型为T的字段y
}

//定义一个泛型枚举
enum Option<T> { //定义一个泛型枚举Option，包含一个类型为T的变体
    Some(T), //定义一个包含数据的变体Some
    None, //定义一个不包含数据的变体None
}

//定义一个泛型trait
trait MyTrait<T> { //定义一个泛型trait MyTrait，包含一个类型为T的方法
    fn method(&self, value: T); //定义一个接收类型为T的参数value的方法method
}

//实现泛型trait
impl<T> MyTrait<T> for Point<T> { //为Point<T>实现MyTrait<T> trait
    fn method(&self, value: T) { //实现MyTrait<T> trait中的method方法
        println!("{:?}, {:?}", self, value); //打印self和value的值
    }
}

//定义一个泛型函数
fn add<T: std::ops::Add<Output=T>>(x: T, y: T) -> T { //定义一个泛型函数add，接收两个类型为T的参数x和y，并返回一个类型为T的值
    x + y //返回x和y的和
}

//定义一个泛型函数
fn sub<T: std::ops::Sub<Output=T>>(x: T, y: T) -> T { //定义一个泛型函数sub，接收两个类型为T的参数x和y，并返回一个类型为T的值
    x - y //返回x和y的差
}

//定义一个泛型函数
fn mul<T: std::ops::Mul<Output=T>>(x: T, y: T) -> T { //定义一个泛型函数mul，接收两个类型为T的参数x和y，并返回一个类型为T的值
    x * y //返回x和y的积
}

//定义一个泛型函数
fn div<T: std::ops::Div<Output=T>>(x: T, y: T) -> T { //定义一个泛型函数div，接收两个类型为T的参数x和y，并返回一个类型为T的值
    x / y //返回x和y的商
}