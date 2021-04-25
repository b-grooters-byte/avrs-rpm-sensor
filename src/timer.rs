//! This basic millis() is directly based on the work by GitHub user 
//! [Rahix](https://github.com/Rahix). You may find more information on the 
//! implementation here: <https://blog.rahix.de/005-avr-hal-millis/>. 

use core::cell;

/// The prescaler is a multiplier that is used to change the frequency of the
/// timer tick count increment. 
const PRESCALER: u32 = 64;
/// The number of tick counts prior to resetting the timer count.
const TIMER_COUNTS: u32 = 250;
/// The clock frequency for an ATMega328P found in the Arduino Uno R3
const ATMEGA328P_FREQ: u32 = 16_000;
/// The timer counter increment value
const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / ATMEGA328P_FREQ;
static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

/// Initializes the millis implementation. The millis counter is initialized
/// to count on single milliseconds. The PRESCALER and TIMER_COUNTS constants
/// may be changed to adjust the freqnecy of updates to the counter.
pub fn millis_init(tc0: arduino_uno::pac::TC0) {
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(TIMER_COUNTS as u8) });
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());
    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

/// Gets the milliseconds offset since the initialization of the millis counter.
pub fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}
