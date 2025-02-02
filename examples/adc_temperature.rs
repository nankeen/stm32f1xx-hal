#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{adc, pac, prelude::*};

use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    // Acquire peripherals
    let p = pac::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(56.mhz())
        .pclk1(28.mhz())
        .adcclk(14.mhz())
        .freeze(&mut flash.acr);
    hprintln!("sysclk freq: {}", clocks.sysclk().0).unwrap();
    hprintln!("adc freq: {}", clocks.adcclk().0).unwrap();

    // Setup ADC
    let mut adc = adc::Adc::adc1(p.ADC1, clocks);

    // Read temperature sensor
    loop {
        let temp = adc.read_temp();

        hprintln!("temp: {}", temp).unwrap();
    }
}
