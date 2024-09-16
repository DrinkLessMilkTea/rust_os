#![no_std] // 不包含标准库
#![no_main] // 不使用 Rust 的入口点
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

// 定义 panic 处理函数
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

// 定义入口点
#[no_mangle] // 不改变函数名
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_os::init();

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }

    #[cfg(test)]
    test_main();

    println!("it did not halt");
    loop {}
}
