//#![deny(unsafe_code)]
#![no_main]
#![no_std]
//#[allow(unused_imports)]
//use core::fmt::{self, Write};
use aux11::{entry, iprintln};
use heapless::Vec;

/*
macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}
struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}
impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            while self.usart1.isr.read().txe().bit_is_clear() {}
            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(c)));
        }
        Ok(())
    }
}
*/
#[entry]
fn main() -> ! {
    let (usart1, _ /*mono_timer*/, mut itm) = aux11::init();
    let mut buffer: Vec<u8, 32> = Vec::new();

    /*
    let mut serial = SerialPort { usart1 };
    let instant = mono_timer.now();
    uprintln!(serial, "The answer is {}", 40 + 2);

    let elapsed = instant.elapsed(); // in ticks
    iprintln!(
        &mut itm.stim[0],
        "`for` loop took {} ticks ({} us)",
        elapsed,
        elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );
    */
    buffer.clear();
    loop {
        // Wait until there is data available
        while usart1.isr.read().rxne().bit_is_clear() {}
        // Retrieve the data
        let inbyte = usart1.rdr.read().rdr().bits() as u8;
        if inbyte == 0x0A || inbyte == 0x0D {
            // send back reversed string
            while let Some(b) = buffer.pop() {
                while usart1.isr.read().txe().bit_is_clear() {} // prevent missing output due to buffer overflows
                usart1.tdr.write(|w| w.tdr().bits(u16::from(b)));
            }
            while usart1.isr.read().txe().bit_is_clear() {} // prevent missing output due to buffer overflows
            usart1.tdr.write(|w| w.tdr().bits(0x0D));
            while usart1.isr.read().txe().bit_is_clear() {} // prevent missing output due to buffer overflows
            usart1.tdr.write(|w| w.tdr().bits(0x0A));
        } else if let Err(e) = buffer.push(inbyte) {
            iprintln!(
                &mut itm.stim[0],
                "Failed to push '{}' to buffer: {}",
                inbyte,
                e
            );
        }
    }
}
