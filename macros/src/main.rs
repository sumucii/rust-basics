fn main() {
    //宏是一种用于生成代码的工具
    //宏可以接受任意数量和类型的参数
    //宏可以生成任意类型的代码
    //宏可以在编译时或运行时执行
    //宏可以定义在任何地方，包括函数、模块和文件中

    //定义一个简单的宏
    macro_rules! hello {
        () => {
            println!("Hello, world!");
        };
    }

    //调用宏
    hello!(); //调用宏hello，打印"Hello, world!"

    //定义一个带参数的宏
    macro_rules! greet {
        ($name:expr) => {
            println!("Hello, {}!", $name);
        };
    }

    //调用带参数的宏
    greet!("Alice"); //调用宏greet，传入字符串"Alice"，打印"Hello, Alice!"
    
    //定义一个带多个参数的宏
    macro_rules! add {
        ($x:expr, $y:expr) => {
            $x + $y
        };
    }

    //调用带多个参数的宏
    let result = add!(1, 2); //调用宏add，传入整数1和2，返回3
    println!("Result: {}", result); //打印add宏的结果

    //定义一个重复调用的宏
    macro_rules! repeat {
        ($($expr:expr),*) => {
            $(println!("{}", $expr);)*
        };
    }

    //调用重复调用的宏
    repeat!("Hello", "World", "!"); //调用宏repeat，重复打印"Hello"、"World"和"!"

    //定义一个递归调用的宏
    macro_rules! countdown {
        ($n:expr) => {
            println!("{}", $n);
        };
        ($n:expr, $($rest:expr),*) => {
            println!("{}", $n);
            countdown!($($rest),*);
        };
    }

    //调用递归调用的宏
    countdown!(3, 2, 1); //调用宏countdown，递归打印3、2和1

    //定义一个模式匹配的宏
    macro_rules! match_number {
        (1) => {
            println!("One");
        };
        (2) => {
            println!("Two");
        };
        (3) => {
            println!("Three");
        };
        ($n:expr) => {
            println!("Other: {}", $n);
        };
    }

    //调用模式匹配的宏
    match_number!(1); //调用宏match_number，匹配1，打印"One"
    match_number!(2); //调用宏match_number，匹配2，打印"Two"
    match_number!(3); //调用宏match_number，匹配3，打印"Three"

    //定义一个可变参数的宏
    macro_rules! sum {
        ($($n:expr),*) => {
            {
                let mut total = 0;
                $(total += $n;)*
                total
            }
        };
    }

    //调用可变参数的宏
    let result = sum!(1, 2, 3, 4, 5); //调用宏sum，传入整数1、2、3、4和5，返回15
    println!("Result: {}", result); //打印sum宏的结果
}