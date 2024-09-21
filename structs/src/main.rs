fn main() {
    //结构体是一种自定义数据类型，用于封装多个数据字段
    //结构体通过struct关键字定义
    //结构体可以包含多个字段，每个字段可以有不同的类型
    //结构体的字段可以通过点号.访问
    //结构体的字段可以通过模式匹配解构

    //定义一个结构体
    struct Point {
        x: i32,
        y: i32,
    }

    //创建一个Point结构体实例
    let point = Point { x: 1, y: 2 };

    //访问Point结构体的字段
    println!("x: {}, y: {}", point.x, point.y);

    //模式匹配解构Point结构体
    let Point { x, y } = point;

    //打印解构后的字段值
    println!("x: {}, y: {}", x, y);

    //定义一个带有方法的结构体
    struct Rectangle {
        width: u32,
        height: u32,
    }

    //为Rectangle结构体实现一个方法

    impl Rectangle {
        //定义一个area方法，用于计算矩形的面积
        fn area(&self) -> u32 {
            self.width * self.height //返回矩形的面积
        }
    }

    //创建一个Rectangle结构体实例
    let rect = Rectangle { width: 3, height: 4 };

    //调用Rectangle结构体的area方法
    let area = rect.area();

    //打印矩形的面积
    println!("Area: {}", area);

    //定义一个带有关联函数的结构体
    struct Circle {
        radius: f64,
    }

    //为Circle结构体实现一个关联函数

    impl Circle {
        //定义一个new方法，用于创建一个Circle结构体实例
        fn new(radius: f64) -> Circle {
            Circle { radius } //返回一个Circle结构体实例
        }
    }

    //调用Circle结构体的关联函数new

    let circle = Circle::new(1.0); //调用Circle结构体的new方法，创建一个Circle结构体实例

    //打印圆的半径
    println!("Radius: {}", circle.radius);

    //定义一个带有泛型的结构体
    struct Pair<T> {
        first: T,
        second: T,
    }

    //创建一个Pair结构体实例
    let pair = Pair { first: 1, second: 2 };

    //打印Pair结构体的字段值
    println!("First: {}, Second: {}", pair.first, pair.second);

    //定义一个带有多个泛型的结构体

    struct Tuple<T, U> {
        first: T,
        second: U,
    }

    //创建一个Tuple结构体实例
    let tuple = Tuple { first: 1, second: "hello" };

    //打印Tuple结构体的字段值
    println!("First: {}, Second: {}", tuple.first, tuple.second);

    //定义一个带有默认泛型的结构体
    struct Default<T = i32> {
        value: T,
    }

    //创建一个Default结构体实例
    let default = Default { value: 42 };

    //打印Default结构体的字段值

    println!("Value: {}", default.value);

    //定义一个带有生命周期的结构体
    struct Ref<'a> {
        value: &'a i32,
    }

    //创建一个Ref结构体实例
    let value = 42;
    let r = Ref { value: &value };

    //打印Ref结构体的字段值
    println!("Value: {}", r.value);

    //定义一个带有生命周期和泛型的结构体
    struct RefPair<'a, T> {
        first: &'a T,
        second: &'a T,
    }

    //创建一个RefPair结构体实例
    let x = 1;
    let y = 2;
    let pair = RefPair { first: &x, second: &y };

    //打印RefPair结构体的字段值
    println!("First: {}, Second: {}", pair.first, pair.second);

    //定义一个带有生命周期和多个泛型的结构体
    struct RefTuple<'a, T, U> {
        first: &'a T,
        second: &'a U,
    }

    //创建一个RefTuple结构体实例
    let x = 1;
    let y = "hello";
    let tuple = RefTuple { first: &x, second: &y };

    //打印RefTuple结构体的字段值
    println!("First: {}, Second: {}", tuple.first, tuple.second);

    //定义一个带有生命周期和默认泛型的结构体
    struct RefDefault<'a, T = i32> {
        value: &'a T,
    }

    //创建一个RefDefault结构体实例
    let value = 42;
    let r = RefDefault { value: &value };

    //打印RefDefault结构体的字段值
    println!("Value: {}", r.value);
}