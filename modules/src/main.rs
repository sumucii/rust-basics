fn main() {
    //模块是一种用于组织代码的机制
    //模块可以包含函数、结构体、枚举、trait等代码
    //模块可以嵌套定义，形成层次结构
    //模块可以通过use关键字引入到作用域中
    //模块可以通过pub关键字导出到外部作用域中
    //模块可以通过super关键字访问父模块中的代码
    //模块可以通过self关键字访问当前模块中的代码
    //模块可以通过crate关键字访问根模块中的代码
    //模块可以通过path路径访问任意模块中的代码

    //下面是一个简单的模块示例：（每一行后添加中文注释）
    my_module::my_function(); //调用my_module模块中的my_function函数
    my_module::MyStruct::my_method(); //调用my_module模块中MyStruct结构体的my_method方法
    my_module::MyEnum::Variant1(42); //调用my_module模块中MyEnum枚举的Variant1变体
    my_module::MyTrait::my_trait_method(); //调用my_module模块中MyTrait trait的my_trait_method方法
    my_module::nested::nested_function(); //调用my_module模块中nested模块的nested_function函数
    my_module::nested::NestedStruct::nested_method(); //调用my_module模块中nested模块的NestedStruct结构体的nested_method方法
    my_module::nested::NestedEnum::Variant1(42); //调用my_module模块中nested模块的NestedEnum枚举的Variant1变体
    my_module::nested::NestedTrait::nested_trait_method(); //调用my_module模块中nested模块的NestedTrait trait的nested_trait_method方法
}

//定义一个模块
mod my_module {
    //定义一个函数
    pub fn my_function() {
        println!("my_function");
    }

    //定义一个结构体
    pub struct MyStruct;

    //为结构体实现方法
    impl MyStruct {
        pub fn my_method() {
            println!("MyStruct::my_method");
        }
    }

    //定义一个枚举
    pub enum MyEnum {
        Variant1(i32),
        Variant2,
    }

    //定义一个trait
    pub trait MyTrait {
        fn my_trait_method();
    }

    //定义一个嵌套模块
    pub mod nested {
        //定义一个函数
        pub fn nested_function() {
            println!("nested_function");
        }

        //定义一个结构体
        pub struct NestedStruct;

        //为结构体实现方法
        impl NestedStruct {
            pub fn nested_method() {
                println!("NestedStruct::nested_method");
            }
        }

        //定义一个枚举
        pub enum NestedEnum {
            Variant1(i32),
            Variant2,
        }

        //定义一个trait
        pub trait NestedTrait {
            fn nested_trait_method();
        }
    }
}

//定义一个trait的实现
impl my_module::MyTrait for my_module::MyStruct {
    fn my_trait_method() {
        println!("MyStruct::my_trait_method");
    }
}

//定义一个嵌套模块的trait的实现
impl my_module::nested::NestedTrait for my_module::nested::NestedStruct {
    fn nested_trait_method() {
        println!("NestedStruct::nested_trait_method");
    }
}

