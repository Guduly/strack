#![no_std]
#![no_main]

use cortex_m::peripheral::{DWT, Peripherals};
use cortex_m::register::msp;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use panic_probe as _;

fn read_profiler(last_cycles: u32) -> (u32, u32, u32) {
    let sram = 0x20020000;
    let stack_pos = msp::read();
    let cyc = DWT::cycle_count();
    let sram_pro = sram - stack_pos;
    let cyc_pro = cyc.wrapping_sub(last_cycles);

    (cyc_pro, sram_pro, cyc)
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    config.enable_debug_during_sleep = true; // <-- CRITICAL FIX
    let p = embassy_stm32::init(config);
    let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    let mut pro = Peripherals::take().unwrap();

    pro.DCB.enable_trace();
    pro.DWT.enable_cycle_counter();

    let mut cyc_coun = 0;
    let mut sram_coun;
    let mut last_cyc;

    (cyc_coun, sram_coun, last_cyc) = read_profiler(DWT::cycle_count());
    defmt::info!(
        "mhz: {} | sram_used: {} bytes | cycles: {}",
        cyc_coun / 1_000_000,
        sram_coun,
        cyc_coun
    );

    loop {
        led.toggle();
        Timer::after_millis(1000).await;
        (cyc_coun, sram_coun, last_cyc) = read_profiler(last_cyc);
        defmt::info!(
            "mhz: {} | sram_used: {} bytes | cycles: {}",
            cyc_coun / 1_000_000,
            sram_coun,
            cyc_coun
        );
    }
}
