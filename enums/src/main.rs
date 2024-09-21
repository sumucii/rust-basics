fn main(){
    //枚举是一种用于表示多种可能性的数据类型
    //枚举的每个值称为一个变体（variant）
    //枚举的每个变体可以包含不同类型的数据
    //枚举的类型取决于其变体的类型
    //枚举的变体可以通过模式匹配来访问

    //定义一个枚举类型
    enum MyEnum {
        Variant1(i32), //定义一个包含整数的变体Variant1
        Variant2, //定义一个不包含数据的变体Variant2
    }

    //使用match表达式匹配枚举实例
    match enum_instance {
        MyEnum::Variant1(x) => println!("Variant1: {}", x), //匹配Variant1时打印x的值
        MyEnum::Variant2 => println!("Variant2"), //匹配Variant2时打印Variant2
    }

    //定义一个枚举实例
    let enum_instance = MyEnum::Variant1(42);
    //打印枚举实例
    println!("enum_instance: {:?}", enum_instance);

    //定义一个枚举实例
    let enum_instance = MyEnum::Variant2;
    //打印枚举实例
    println!("enum_instance: {:?}", enum_instance);
}