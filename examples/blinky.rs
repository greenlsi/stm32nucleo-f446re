#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use hal::prelude::*; // Convenience re-export of multiple traits
use stm32f4xx_hal as hal;
//use stm32nucleo_f446re::hal as hal;
//use stm32f4xx_hal::peripherals; // needed for the GpioExt trait (-> .split)
use stm32nucleo_f446re::led::Led;

#[entry]
fn main() -> ! {
    let peripherals = hal::pac::Peripherals::take().unwrap();
    let gpioa = peripherals.GPIOA.split(); // + sets RCC->AHB1ENR GPIOA bit

    // .into_push_pull_output performs three steps
    // 1) set PUPDR: 00 -> no pull-up, no pull-down
    // 2) set OTYPER: 0 -> output push-pull
    // 3) set MODER: 01 -> general purpose output mode
    let mut led = Led::new(gpioa);
    //let mut led = gpioa.pa5.into_push_pull_output();

    let gpioc = peripherals.GPIOC.split();
    let button = gpioc.pc13; // pins are input by default

    loop {
        if button.is_high() {
            led.off();
        } else {
            led.on();
        }
    }
}
