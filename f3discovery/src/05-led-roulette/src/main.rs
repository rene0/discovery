#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    // order is N, NE, E, SE, S, SW, W, NW left-to-right
    let diag2: [u8;16] = [
        0b_11000000, //   0 ms
        0b_01000000, //  50 ms
        0b_01100000, // 100 ms
        0b_00100000, // 150 ms
        0b_00110000, // 200 ms
        0b_00010000, // 250 ms
        0b_00011000, // 300 ms
        0b_00001000, // 350 ms
        0b_00001100, // 400 ms
        0b_00000100, // 450 ms
        0b_00000110, // 500 ms
        0b_00000010, // 550 ms
        0b_00000011, // 600 ms
        0b_00000001, // 650 ms
        0b_10000001, // 700 ms
        0b_10000000, // 750 ms
    ];
    // Start with all LEDs off:
    for led in 0..=7 {
        leds[led].off().ok();
    }
    // Now roulette the LEDs according to above diagram:
    loop {
        for time in 0..=15 {
            for led in 0..=7 {
                // The left-most bit is most significant so has weigh 0x80,
                // the right-most bit has weigh 0x01, so index accordingly.
                if (diag2[time] & (1 << (7 - led))) != 0 {
                    leds[led].on().ok();
                } else {
                    leds[led].off().ok();
                }
            }
            delay.delay_ms(50_u16);
        }
    }
}
