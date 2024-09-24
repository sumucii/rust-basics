fn main() {
    //concurrency并发编程，并在每一行后添加中文注释

    //创建一个新的线程
    std::thread::spawn(|| {
        //在新线程中打印一条消息
        println!("Hello from a thread!");
    });

    //在主线程中打印一条消息
    println!("Hello from main thread!");

    //等待新线程结束
    std::thread::sleep(std::time::Duration::from_secs(1));

    //创建一个新的线程
    std::thread::spawn(|| {
        //在新线程中打印一条消息
        println!("Hello from another thread!");
    });

    //在主线程中打印一条消息
    println!("Hello from main thread!");

    //等待新线程结束
    std::thread::sleep(std::time::Duration::from_secs(1));
}