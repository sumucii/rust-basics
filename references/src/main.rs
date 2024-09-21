fn main() {
    //referrnces模块的代码示例，用于演示路径的使用，不可直接运行，需要将代码片段复制到其他项目中
    //定义一个结构体
    struct MyStruct;

    //为结构体实现方法
    impl MyStruct {
        fn my_method() {
            println!("MyStruct::my_method");
        }
    }

    //调用结构体的方法
    MyStruct::my_method(); //调用MyStruct结构体的my_method方法

    //调用模块中的函数

    //定义一个模块
    mod my_module {
        //定义一个函数
        pub fn my_function() {
            println!("my_function");
        }
    }

    //调用模块中的函数
    my_module::my_function(); //调用my_module模块中的my_function函数

    //使用super关键字访问父模块中的函数

    //定义一个模块
    mod my_module {
        //定义一个函数
        pub fn my_function() {
            println!("my_function");
        }
    }
}