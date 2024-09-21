fn main() {
    //iterators的代码示例

    //定义一个数组
    let array = [1, 2, 3, 4, 5];
    //定义一个迭代器
    let iter = array.iter();
    //打印迭代器的值
    for i in iter {
        println!("array中的值：{}", i); //打印1、2、3、4、5
    }

    //定义一个字符串
    let s = String::from("Hello, Rust!");
    //定义一个字符迭代器
    let iter = s.chars();
    //打印字符迭代器的值
    for c in iter {
        println!("s中的字符：{}", c); //打印'H'、'e'、'l'、'l'、'o'、','、' '、'R'、'u'、's'、't'、'!'
    }
}