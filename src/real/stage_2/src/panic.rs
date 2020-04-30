use core::panic::PanicInfo;
use shared::println;
use shared::utils;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("[Panic] {}", info);

    loop {
        utils::hlt()
    }
}
