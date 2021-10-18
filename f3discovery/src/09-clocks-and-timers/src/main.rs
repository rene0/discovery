#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // set timer to go off in `ms` ticks (= ms)
    tim6.arr.write(|w| w.arr().bits(ms));
    // CEN: enable timer
    tim6.cr1.modify(|_, w| w.cen().set_bit());
    // wait until alarm goes off
    while !tim6.sr.read().uif().bit_is_set() {}
    // reset update event flag for next delay() call
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // initialize TIM6
    // power on TIM6
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());
    // let TIM6 operate in one-pulse mode
    // OPM: select one-pulse mode
    // CEN: keep counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    // configure pre-scalar to have counter operate at 1 kHz
    tim6.psc.write(|w| w.psc().bits(/* psc */ 7999));
    // abp1 = 8M, counterfreq=abp/(psc+1) -> psc+1=8M/1k so psc+1=8000

    let ms = 50;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
