use esp_idf_hal::io::Error;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::*;
use esp_idf_hal::sys::EspError;

pub struct Display {}

pub fn init_display() -> Result<SpiDriver<'static>, EspError> {
    let peripherals = Peripherals::take()?;
    let spi = peripherals.spi2;

    let sclk = peripherals.pins.gpio15;
    let serial_in = peripherals.pins.gpio16; // SDI
    let serial_out = peripherals.pins.gpio17; // SDO
    let cs_1 = peripherals.pins.gpio18;
    let cs_2 = peripherals.pins.gpio19;

    println!("Starting SPI loopback test");

    let driver = SpiDriver::new::<SPI2>(
        spi,
        sclk,
        serial_out,
        Some(serial_in),
        &SpiDriverConfig::new(),
    )?;

    Ok(driver)
}
