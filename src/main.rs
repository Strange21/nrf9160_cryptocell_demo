#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
// use nrf9160_pac as pac;
// use nrfxlib_sys as sys;
use nrf9160_hal as hal;

pub mod bl_cc310;
pub mod wraper_bl_cc310;

#[entry]
fn main() -> ! {
    // Initilize trustzone
    
    // enable crypto cell

    // verify integrity of firmware

    // jump to firmware

    loop {
        // your code goes here
    }
}