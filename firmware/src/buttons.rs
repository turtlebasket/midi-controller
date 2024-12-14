use esp_idf_hal::{gpio::*, io::Error, peripherals::Peripherals, sys::EspError};

pub fn init_buttons() -> Result<[PinDriver<'static, AnyInputPin, Pull>; 4], EspError> {
    let peripherals = match Peripherals::take() {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    // wired to GPIO0, GPIO1, GPIO2, GPIO3
    let mut mux_bits = [
        (match PinDriver::input(peripherals.pins.gpio0) {
            Ok(b) => b,
            Err(e) => return Err(e),
        }),
        (match PinDriver::input(peripherals.pins.gpio1) {
            Ok(b) => b,
            Err(e) => return Err(e),
        }),
        (match PinDriver::input(peripherals.pins.gpio2) {
            Ok(b) => b,
            Err(e) => return Err(e),
        }),
        (match PinDriver::input(peripherals.pins.gpio3) {
            Ok(b) => b,
            Err(e) => return Err(e),
        }),
    ];

    for mux_bit in mux_bits.iter_mut() {
        mux_bit.set_pull(Pull::Up);
    }

    Ok(mux_bits)
}

/// decode the number of the button being pressed from 16-way (4-bit) multiplexer output
pub fn decode_button_number(bits: &[bool; 4]) -> u8 {
    let mut button_number = 0;
    for (i, bit) in bits.iter().enumerate() {
        if *bit {
            button_number |= 1 << i;
        }
    }
    button_number
}
