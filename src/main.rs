#![no_std]
#![no_main]

#[cfg(not(test))]
#[panic_handler]
fn panic (_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main () -> ! {
    loop {}
}
