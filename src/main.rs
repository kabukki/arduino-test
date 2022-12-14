#![no_std]
#![no_main]

#[panic_handler]
fn panic (_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main () -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 56700);

    loop {
        led.set_high();
        serial.write_byte(b'A');
        arduino_hal::delay_ms(100);
        led.set_low();
        arduino_hal::delay_ms(900);
    }
}
