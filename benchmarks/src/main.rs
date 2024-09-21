//引入Criterion库
use criterion::{criterion_group, criterion_main, Criterion};
fn main() {
    //benchmark的作用以及完整的代码示例
    //benchmark是一种用于测试代码性能的工具，通常用于比较不同实现的性能差异。
    //在Rust中，通常使用Criterion库进行基准测试。
    //下面是一个简单的基准测试示例：（每一行后添加中文注释）
    criterion_group!(benches, fibonacci_benchmark); //定义一个基准测试组
    criterion_main!(benches); //运行基准测试组
}

fn fibonacci(n: u64) -> u64 { //定义一个递归函数，用于计算斐波那契数列
    if n == 0 { //当n等于0时
        return 0; //返回0
    } else if n == 1 { //当n等于1时
        return 1; //返回1
    } else { //其他情况
        return fibonacci(n - 1) + fibonacci(n - 2); //递归调用函数计算斐波那契数列
    }
}
fn fibonacci_benchmark(c: &mut Criterion) { //定义一个基准测试函数
    c.bench_function("fibonacci 20", |b| b.iter(|| fibonacci(20))); //对fibonacci函数进行基准测试
}