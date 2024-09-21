fn main() {
    //数据类型是编程语言中用于表示数据的种类和结构的系统
    //在Rust中，数据类型主要包括基本数据类型、复合数据类型和自定义数据类型
    //基本数据类型包括整数、浮点数、布尔值、字符和空值
    //复合数据类型包括数组、元组、切片、字符串和引用
    //自定义数据类型包括结构体、枚举、联合和trait
    //下面是一个简单的数据类型示例：（每一行后添加中文注释）
    let integer: i32 = 42; //定义一个整数变量integer
    let float: f64 = 3.14; //定义一个浮点数变量float
    let boolean: bool = true; //定义一个布尔值变量boolean
    let character: char = 'A'; //定义一个字符变量character
    let unit: () = (); //定义一个空值变量unit
    let array: [i32; 3] = [1, 2, 3]; //定义一个整数数组array
    let tuple: (i32, f64, bool) = (1, 3.14, true); //定义一个元组tuple
    let slice: &[i32] = &array; //定义一个整数切片slice
    let string: String = String::from("Hello, Rust!"); //定义一个字符串string
    let reference: &i32 = &integer; //定义一个整数引用reference
    let mut struct_instance = MyStruct { //定义一个结构体实例struct_instance
        field1: 1, //结构体字段1
        field2: 2, //结构体字段2
    };
    struct_instance.field1 = 3; //修改结构体字段1的值
    let enum_instance = MyEnum::Variant1(42); //定义一个枚举实例enum_instance
    match enum_instance { //使用match表达式匹配枚举实例
        MyEnum::Variant1(x) => println!("Variant1: {}", x), //匹配Variant1时打印x的值
        MyEnum::Variant2 => println!("Variant2"), //匹配Variant2时打印Variant2
    }

    println!("integer: {}", integer);
    println!("float: {}", float);
    println!("boolean: {}", boolean);
    println!("character: {}", character);
    println!("unit: {:?}", unit);
    println!("array: {:?}", array);
    println!("tuple: {:?}", tuple);
    println!("slice: {:?}", slice);
    println!("string: {}", string);
    println!("reference: {:?}", reference);
    println!("struct_instance: {:?}", struct_instance);
    println!("enum_instance: {:?}", enum_instance);
}