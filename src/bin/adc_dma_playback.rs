#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32g4xx_hal::{
    adc::{config::{Continuous, SampleTime, Sequence}, 
        AdcClaim, ClockSource}, 
    pac::Peripherals, 
    prelude::*, 
    pwr::PwrExt, 
    rcc::Config , 
    dac::{ DacOut, DacExt}
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
    let gpioa = dp.GPIOA.split(&mut rcc); 
    let dac = dp.DAC1.constrain(gpioa.pa4,&mut rcc); 
    
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
    adc.configure_channel(&pc3, Sequence::One, SampleTime::Cycles_12_5);
    
    let adc = adc.enable(); 
    let mut adc = adc.start_conversion(); 

    let mut dac_manual = dac.calibrate_buffer(&mut delay).enable(); 


    // let mut count = 0; 
    loop {

        adc = adc.wait_for_conversion_sequence().unwrap_active(); 
        let vref = adc.current_sample(); 

        // if count > 10000 {
        //     count = 0; 
        //     rprintln!("voltage out : {}", Vref::sample_to_millivolts(vref)); 
        // }

        dac_manual.set_value(vref);

        // count += 1; 
        // delay.delay_ms(1000);
    }
}