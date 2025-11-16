use esp_hal::gpio::Output;

pub fn set_relay_state(relay_pin: &mut Output, close: bool) {
    if close {
        relay_pin.set_high();
    } else {
        relay_pin.set_low();
    }
}
