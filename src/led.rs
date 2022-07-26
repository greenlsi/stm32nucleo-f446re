use crate::hal::gpio::{gpioa, ErasedPin, Output, Pin, PushPull};

/// One of the on-board user LEDs
pub struct Led {
    pin: ErasedPin<Output<PushPull>>,
}

impl Led {
    /// Creates a new Led struct for the LED LD2 of the STM32Nucle-f446re board.
    pub fn new(gpioa: gpioa::Parts) -> Self {
        let pin = gpioa.pa5.into_push_pull_output();
        pin.into()
    }

    /// Turns the LED off.
    pub fn off(&mut self) {
        self.pin.set_low();
    }

    /// Turns the LED on.
    pub fn on(&mut self) {
        self.pin.set_high();
    }

    /// Toggles the LED.
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}

impl<const P: char, const N: u8> From<Pin<P, N, Output>> for Led {
    /// Pin-to-LED conversion
    fn from(p: Pin<P, N, Output>) -> Self {
        Led { pin: p.erase() }
    }
}
