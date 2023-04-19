#![no_std]
#![no_main]

use core::convert::Infallible;
use core::fmt::Write;
use cortex_m::Peripherals as CorePeripherals;
use cortex_m_rt::entry;
use dht_sensor::{dht22, DhtError, DhtReading};
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::{OpenDrain, Output, Pin, PushPull},
    pac::Peripherals as DevicePeripherals,
    prelude::*,
    rcc::RccExt,
    timer::SysDelay,
};

// example of a reading a dht22 sensor and sending the measurements via serial
// It shows how to use the dht22 sensor and how rust type systems and borrow checker
// enforces that the sensor can't be used in multiple places at the same time
// Serial is connected to the PA9 Pin. DHT22 one-wire is connected to the PB10 Pin
#[entry] // The entry point of the program
fn main() -> ! {
    let core_peripherals = CorePeripherals::take().unwrap(); // Take a single instance of the core peripherals
    let device_peripherals = DevicePeripherals::take().unwrap(); // Take a single instance of the device peripherals

    let rcc = device_peripherals.RCC.constrain(); // Constrain the Reset and Clock Control peripheral
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze(); // Configure the clock to 48 MHz and freeze it
    let mut delay = core_peripherals.SYST.delay(&clocks); // Create a delay abstraction based on the system timer

    let gpioa = device_peripherals.GPIOA.split(); // Split the GPIOA peripheral into its pins
    let gpiob = device_peripherals.GPIOB.split(); // Split the GPIOB peripheral into its pins
    let gpioc = device_peripherals.GPIOC.split(); // Split the GPIOC peripheral into its pins

    let tx_pin = gpioa.pa9.into_alternate(); // Configure the PA9 pin as an alternate function pin for serial
    let dht_pin = gpiob.pb10.into_open_drain_output(); // Configure the PB10 pin as an open drain output
    let led_pin = gpioc.pc13.into_push_pull_output(); // Configure the PC13 pin as a push-pull output

    let mut serial_tx = device_peripherals
        .USART1
        .tx(tx_pin, 9600.bps(), &clocks)
        .unwrap(); // Configure the USART1 peripheral to send serial data

    writeln!(serial_tx, "Hello, building IoT\r").unwrap();

    loop {}
}

type LedPin = Pin<'C', 13, Output<PushPull>>;
type Dht22Pin = Pin<'B', 10, Output<OpenDrain>>;

pub struct Dht22Measurement {
    pub temp: f32,
    pub hum: f32,
}

pub struct Dht22Sensor {
    pin: Dht22Pin,
}

impl Dht22Sensor {
    pub fn new(pin: Dht22Pin) -> Self {
        let mut sensor = Dht22Sensor { pin };
        sensor.pin.set_high();
        sensor
    }

    pub fn read(&mut self, delay: &mut SysDelay) -> Result<Dht22Measurement, DhtError<Infallible>> {
        let result = dht22::Reading::read(delay, &mut self.pin)?;
        Ok(Dht22Measurement {
            hum: result.relative_humidity,
            temp: result.temperature,
        })
    }
}

pub struct Led {
    pin: LedPin,
}

impl Led {
    fn new(pin: LedPin) -> Self {
        Led { pin }
    }

    fn on(&mut self) {
        self.pin.set_low();
    }

    fn off(&mut self) {
        self.pin.set_high();
    }
}
