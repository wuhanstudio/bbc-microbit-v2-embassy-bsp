
#![no_main]
#![no_std]

use defmt::info;
use hal::twim;
use embassy_nrf::{self as hal, twim::Twim};
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use lsm303agr::Lsm303agr;

use defmt_rtt as _;
use panic_rtt_target as _; // Panic handler

hal::bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<hal::peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) -> ! {

    let _cp = cortex_m::Peripherals::take().unwrap();
    // Use ``dp` to get a handle to the peripherals
    let dp = hal::init(Default::default());

    info!("Starting");

    let config = twim::Config::default();
    let twim0 = Twim::new(dp.TWISPI0, Irqs, dp.P0_16, dp.P0_08, config);

    let mut sensor = Lsm303agr::new_with_i2c(twim0);
    // let id = sensor.magnetometer_id().unwrap();
    // info!(id);

    sensor.init().unwrap();
    sensor
        .set_mag_mode_and_odr(
            &mut Delay,
            lsm303agr::MagMode::HighResolution,
            lsm303agr::MagOutputDataRate::Hz10,
        )
        .unwrap();

    let Ok(mut sensor) = sensor.into_mag_continuous() else {
        panic!("Error enabling continuous mode")
    };
    sensor.mag_enable_low_pass_filter().unwrap();
    loop {
        if sensor.mag_status().unwrap().xyz_new_data() {
            let data = sensor.magnetic_field().unwrap();
            info!(
                "Magnetic field: x {} y {} z {}",
                data.x_nt() / 1000,
                data.y_nt() / 1000,
                data.z_nt() / 1000
            );
        } else {
            info!("No data")
        }
        Delay.delay_ms(200).await;
    }
}
