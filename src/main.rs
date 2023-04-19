#![no_std]
#![no_main]

use cortex_m::Peripherals as CorePeripherals;
use cortex_m_rt::entry;
// Halt on panic
use panic_halt as _;
use stm32f4xx_hal::{
    pac::Peripherals as DevicePeripherals, prelude::*, rcc::RccExt, timer::SysTimerExt,
};

#[entry] // The entry point of the program
fn main() -> ! {
    let core_peripherals = CorePeripherals::take().unwrap(); // Take a single instance of the core peripherals
    let device_peripherals = DevicePeripherals::take().unwrap(); // Take a single instance of the device peripherals

    let rcc = device_peripherals.RCC.constrain(); // Constrain the Reset and Clock Control peripheral
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze(); // Configure the clock to 48 MHz and freeze it
    let mut delay = core_peripherals.SYST.delay(&clocks); // Create a delay abstraction based on the system timer

    let gpioc = device_peripherals.GPIOC.split(); // Split the GPIOC peripheral into its pins
    let mut led_pin = gpioc.pc13.into_push_pull_output(); // Configure the PC13 pin as a push-pull output

    loop {
        delay.delay(1.secs()); // Delay for 1 second
        led_pin.set_low(); // Turn on the LED
        delay.delay(1.secs()); // Wait for 1 second
        led_pin.set_high(); // Turn off the LED
    }
}
