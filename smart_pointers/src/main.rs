fn main() {
    //smart_pointers的作用
    //智能指针是一种数据结构，它包含了指向堆内存中数据的指针，并且可以提供额外的元数据和功能。
    //智能指针可以在编译时检查内存安全性，避免内存泄漏和空指针引用等问题。
    //Rust中的智能指针有Box、Rc、Arc、Cell、RefCell等，这些智能指针提供了不同的功能和用法。

    //smart_pointers的代码示例

    //定义一个智能指针
    let x = Box::new(5);
    //打印智能指针的值
    println!("x: {}", x); //打印"x: 5"

    //定义一个智能指针
    let y = Box::new("Hello, Rust!");
    //打印智能指针的值
    println!("y: {}", y); //打印"y: Hello, Rust!"

    //定义一个智能指针
    let z = Box::new([1, 2, 3, 4, 5]);
    //打印智能指针的值
    println!("z: {:?}", z); //打印"z: [1, 2, 3, 4, 5]"
}