fn main() {
    //slices的代码示例

    //定义一个数组
    let array = [1, 2, 3, 4, 5];
    //定义一个切片
    let slice = &array[1..4];
    //打印切片的值
    println!("{:?}", slice); //打印[2, 3, 4]

    //定义一个字符串
    let s = String::from("Hello, Rust!");
    //定义一个字符串切片
    let slice = &s[7..11];
    //打印字符串切片的值
    println!("{:?}", slice); //打印"Rust"
}