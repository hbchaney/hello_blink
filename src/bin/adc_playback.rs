#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32g4xx_hal::{
    adc::{self, config::SampleTime, config::Sequence, ClockSource, AdcClaim, Vref, config::Continuous, Temperature}, pac::{self, Peripherals}, prelude::*, pwr::PwrExt, rcc::Config, 
};

use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {

    rtt_init_print!(); 

    rprintln!("starting peripherals"); 
    let dp = Peripherals::take().unwrap(); 
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals"); 

    rprintln!("starting rcc stuff"); 

    let rcc = dp.RCC.constrain();
    let pwr = dp.PWR.constrain().freeze(); 
    let mut rcc = rcc.freeze(Config::hsi(), pwr); 

    rprintln!("gpio adc and dac"); 
    let gpioc = dp.GPIOC.split(&mut rcc); 
    let pc3 = gpioc.pc3.into_analog(); 

    rprintln!("Setup up Adc1"); 
    let mut delay = cp.SYST.delay(&rcc.clocks); 
    let mut adc = dp
        .ADC1
        .claim(ClockSource::SystemClock, &rcc, &mut delay, true);

    adc.enable_temperature(&dp.ADC12_COMMON);
    adc.enable_vref(&dp.ADC12_COMMON);
    adc.set_auto_delay(true);
    adc.set_continuous(Continuous::Continuous);
    adc.reset_sequence();
    adc.configure_channel(&pc3, Sequence::One, SampleTime::Cycles_640_5);
    // adc.configure_channel(&Vref, Sequence::Two, SampleTime::Cycles_640_5);
    // adc.configure_channel(&Temperature, Sequence::Three, SampleTime::Cycles_640_5); // not sure whats up with this 
    
    let adc = adc.enable(); 


    loop {

    }
}