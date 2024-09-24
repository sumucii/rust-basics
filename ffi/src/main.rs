fn main() {
    //ffi外部函数接口，用于调用其他语言的函数
    //调用C语言的puts函数
    unsafe {
        libc::puts("Hello, world!\0".as_ptr() as *const i8);
    }
}