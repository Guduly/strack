#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    config.enable_debug_during_sleep = true; // <-- CRITICAL FIX
    let p = embassy_stm32::init(config);
    let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
