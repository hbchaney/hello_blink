#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32g4xx_hal::{
    gpio::GpioExt,
    pac::Peripherals,
    prelude::SetDutyCycle,
    pwm::PwmExt,
    pwr::PwrExt,
    rcc::{Config, PllConfig, PllMDiv, PllNMul, PllRDiv, PllSrc, RccExt},
    stm32::TIM1,
    time::RateExtU32,
};

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = Peripherals::take().expect("cannot take peripherals");
    let rcc = dp.RCC.constrain();
    let pll_conf = PllConfig {
        mux: PllSrc::HSI,
        m: PllMDiv::DIV_4,
        n: PllNMul::MUL_85,
        r: Some(PllRDiv::DIV_2),
        q: Some(stm32g4xx_hal::rcc::PllQDiv::DIV_2),
        p: Some(stm32g4xx_hal::rcc::PllPDiv::DIV_2),
    };

    let pll_conf = Config::pll().pll_cfg(pll_conf);
    let pwr = dp.PWR.constrain().freeze();

    let mut rcc = rcc.freeze(pll_conf, pwr);
    let gpioc = dp.GPIOC.split(&mut rcc);
    let pin = gpioc.pc3.into_alternate();

    let mut pwm = dp.TIM1.pwm(pin, 10.kHz(), &mut rcc);

    rprintln!("Current APB2 value {}", rcc.clocks.sys_clk.to_MHz());

    //attempt to reconfig the timer
    unsafe {
        let tim = &(*TIM1::ptr());
        //Channel 1
        //Disable the channel before configuring it
        tim.ccer.modify(|_, w| w.cc1e().clear_bit());

        tim.ccmr1_output().modify(|_, w| {
            w
                //Preload enable for channel
                .oc1pe()
                .set_bit()
                //Set mode for channel, the default mode is "frozen" which won't work
                .oc1m()
                .pwm_mode1()
        });

        tim.arr.modify(|_, w| w.arr().bits(100 - 1));
        tim.psc.modify(|_, w| w.psc().bits(170 - 1));

        //Enable the channel
        tim.ccer.modify(|_, w| w.cc1e().set_bit());

        //Enable the TIM main Output
        tim.bdtr.modify(|_, w| w.moe().set_bit());
    }

    pwm.set_duty_cycle(pwm.max_duty_cycle() / 4).unwrap();
    pwm.enable();

    loop {
        cortex_m::asm::nop();
    }
}
