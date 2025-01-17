#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32g4xx_hal::{
    pac, prelude::*, pwr::PwrExt, rcc::Config, timer::Timer,
};

use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let device = pac::Peripherals::take().unwrap(); 
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let pwr = device.PWR.constrain().freeze();  
    let mut rcc = device.RCC.freeze(Config::hsi(), pwr);
    let mut sys_delay = cp.SYST.delay(&rcc.clocks); 
    let _tim = Timer::new(device.TIM6, &rcc.clocks); 

    let gpioa = device.GPIOA.split(&mut rcc); 

    let mut led = gpioa.pa5.into_push_pull_output();
    
    rprintln!("Hello, world!"); 
    let mut counter = 0; 

    loop {
        led.toggle().unwrap();
        sys_delay.delay_ms(1000);
        counter += 1; 
        rprintln!("hello again {}", counter); 
    }
}