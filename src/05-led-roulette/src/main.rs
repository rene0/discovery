#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    // order is N, NE, E, SE, S, SW, W, NW left-to-right
    let timing_diagram: [u8; 16] = [
        0b_1100_0000, //   0 ms
        0b_0100_0000, //  50 ms
        0b_0110_0000, // 100 ms
        0b_0010_0000, // 150 ms
        0b_0011_0000, // 200 ms
        0b_0001_0000, // 250 ms
        0b_0001_1000, // 300 ms
        0b_0000_1000, // 350 ms
        0b_0000_1100, // 400 ms
        0b_0000_0100, // 450 ms
        0b_0000_0110, // 500 ms
        0b_0000_0010, // 550 ms
        0b_0000_0011, // 600 ms
        0b_0000_0001, // 650 ms
        0b_1000_0001, // 700 ms
        0b_1000_0000, // 750 ms
    ];
    // Start with all LEDs off:
    for led in &mut leds {
        led.off().ok();
    }
    // Now roulette the LEDs according to above diagram:
    loop {
        for time in &timing_diagram {
            for led in 0..=7 {
                // The left-most bit is most significant so has weigh 0x80,
                // the right-most bit has weigh 0x01, so index accordingly.
                if (time & (1 << (7 - led))) != 0 {
                    leds[led].on().ok();
                } else {
                    leds[led].off().ok();
                }
            }
            delay.delay_ms(50_u16);
        }
    }
}
