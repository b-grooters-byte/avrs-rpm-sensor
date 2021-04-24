#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

#[path="timer.rs"]
mod timer;

use arduino_uno::prelude::*;
use panic_halt as _;

// The AVR application entry point. The function signature 
// ````
//   fn main() -> !
// ````
// uses a feature of Rust to indicate that the function does not return. The
// '!' return type instructs that compiler that it should not expect a return
// value of any type from this function. This is known as the 'Never' type.
#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    // set up the serial port
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );
    // output startup message
    ufmt::uwriteln!(&mut serial, "RPM 0.1.0\n").void_unwrap();
    // set up a millisecond timer
    timer::millis_init(dp.TC0);
    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };
    // set up the hall effect sensor pin
    let sensor = pins.d2.into_pull_up_input(&mut pins.ddr);
    let mut pin_low = false;
    let mut prev = timer::millis();
    let mut revs: i32 = 0;

    loop {
        let now = timer::millis();
        let elapsed = now - prev;
        if elapsed >= 1000 {
            // RPM is converted back to i32 to display with ufmt 
            // the flywheel has 2 magnets so we multiply by 30_000 millis 
            let rpm = (revs as f32  / elapsed as f32 * 30_0000.0) as i32;
            revs = 0;
            prev = now;
            ufmt::uwriteln!(&mut serial, "RPM {}\n", rpm ).void_unwrap();
        }
        if sensor.is_low().unwrap() && !pin_low{
            revs +=1;
            pin_low = true;
        } else if sensor.is_high().unwrap() && pin_low {
            pin_low = false;
        }
    }
}


