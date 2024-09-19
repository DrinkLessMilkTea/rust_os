#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_os::{exit_qemu, serial_println, QEMUExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_panic();
    serial_println!("[test did not panic]");
    exit_qemu(QEMUExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QEMUExitCode::Success);
    loop {}
}

fn should_panic() {
    assert_eq!(0, 1);
}
