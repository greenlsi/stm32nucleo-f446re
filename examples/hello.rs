#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry; // Required for the #[entry] macro
use panic_rtt_target as _; // Required for panicking using RTT
use rtt_target::{rprintln, rtt_init_print}; // Functions for logging using RTT
use stm32f4xx_hal as _; // Required for dealing with the hardware

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");
    loop {}
}
