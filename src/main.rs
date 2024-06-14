#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::rcc::*;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    rcc::{Hse, HseMode},
    time::Hertz,
    Config,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    /*
     https://github.com/STMicroelectronics/STM32CubeH7/blob/master/Projects/STM32H735G-DK/Examples/GPIO/GPIO_EXTI/Src/main.c
     @brief  System Clock Configuration
        The system Clock is configured as follow :
        System Clock source            = PLL (HSE)
        SYSCLK(Hz)                     = 520000000 (CPU Clock)
        HCLK(Hz)                       = 260000000 (AXI and AHBs Clock)
        AHB Prescaler                  = 2
        D1 APB3 Prescaler              = 2 (APB3 Clock  130MHz)
        D2 APB1 Prescaler              = 2 (APB1 Clock  130MHz)
        D2 APB2 Prescaler              = 2 (APB2 Clock  130MHz)
        D3 APB4 Prescaler              = 2 (APB4 Clock  130MHz)
        HSE Frequency(Hz)              = 25000000
        PLL_M                          = 5
        PLL_N                          = 104
        PLL_P                          = 1
        PLL_Q                          = 4
        PLL_R                          = 2
        VDD(V)                         = 3.3
        Flash Latency(WS)              = 3
    */

    // setup power and clocks for an stm32h735g-dk run from an external 25 Mhz external oscillator
    let mut config = Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz::mhz(25),
        mode: HseMode::Oscillator,
    });
    config.rcc.hsi = None;
    config.rcc.csi = false;
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV5, // PLL_M
        mul: PllMul::MUL104,     // PLL_N
        divp: Some(PllDiv::DIV1),
        divq: Some(PllDiv::DIV4),
        divr: Some(PllDiv::DIV2),
    });
    config.rcc.voltage_scale = VoltageScale::Scale0;
    config.rcc.supply_config = SupplyConfig::DirectSMPS;
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.ahb_pre = AHBPrescaler::DIV2;
    config.rcc.apb1_pre = APBPrescaler::DIV2;
    config.rcc.apb2_pre = APBPrescaler::DIV2;
    config.rcc.apb3_pre = APBPrescaler::DIV2;
    config.rcc.apb4_pre = APBPrescaler::DIV2;
    let p = embassy_stm32::init(config);

    // LD1 led
    let mut led = Output::new(p.PC3, Level::High, Speed::Low);
    let mut counter = 0;

    loop {
        info!("blink: {}", counter);
        counter += 1;

        // on
        led.set_low();
        Timer::after(Duration::from_millis(50)).await;

        // off
        led.set_high();
        Timer::after(Duration::from_millis(450)).await;
    }
}
