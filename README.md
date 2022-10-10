# Test

Arduino + Rust playground 🎉

## Overview

This project uses the [arduino-hal](https://github.com/Rahix/avr-hal) crate. Because Arduinos use AVR microcontrollers, it is based on the [avr-device](https://github.com/Rahix/avr-device) PAC, and provides a HAL for Arduino boards, defined by traits from [embedded-hal](https://github.com/rust-embedded/embedded-hal).

## Pipeline

### Build the program

```sh
cargo build --release
```

### Upload to microcontroller

```sh
avrdude -b 115200 -c arduino -D -p atmega328p -P /dev/tty.usbmodem14401 -U flash:w:target/avr-atmega328p/release/arduino-test.elf
```
