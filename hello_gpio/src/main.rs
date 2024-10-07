#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let mut row1 = Output::new(p.P0_21, Level::Low, OutputDrive::Standard);
    let mut row2 = Output::new(p.P0_22, Level::Low, OutputDrive::Standard);
    let mut row3 = Output::new(p.P0_15, Level::Low, OutputDrive::Standard);
    let mut row4 = Output::new(p.P0_24, Level::Low, OutputDrive::Standard);
    let mut row5 = Output::new(p.P0_19, Level::Low, OutputDrive::Standard);

    row1.set_high();
    row2.set_high();
    row3.set_high();
    row4.set_high();
    row5.set_high();

    let mut col1 = Output::new(p.P0_28, Level::Low, OutputDrive::Standard);
    let mut col2 = Output::new(p.P0_11, Level::Low, OutputDrive::Standard);
    let mut col3 = Output::new(p.P0_31, Level::Low, OutputDrive::Standard);
    let mut col4 = Output::new(p.P1_05, Level::Low, OutputDrive::Standard);
    let mut col5 = Output::new(p.P0_30, Level::Low, OutputDrive::Standard);

    loop {
        col1.set_high();
        col2.set_high();
        col3.set_high();
        col4.set_high();
        col5.set_high();

        Timer::after_millis(300).await;

        col1.set_low();
        col2.set_low();
        col3.set_low();
        col4.set_low();
        col5.set_low();

        Timer::after_millis(300).await;
    }
}
