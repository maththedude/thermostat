#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::*;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};
use thermostat::{OFF, ON, thermostat::*};
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    info!("start");

    // Init peripherals
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Heap allocation
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    esp_alloc::heap_allocator!(size: 64 * 1024); //TODO: Added by esp-gen with "COEX needs more RAM - so we've added some more." Investigate.

    let mut thermostat = Thermostat {
        heat: OFF,
        cool: OFF,
        fan: OFF,
        fan_mode: FanMode::Off,
        temp: 70,
        set_point_low: 70,
        set_point_high: 70,
        mode: Mode::Off,
        backlight: ON,
        backlight_since: Instant::now(),
        heat_pin: Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default()),
        cool_pin: Output::new(peripherals.GPIO3, Level::Low, OutputConfig::default()),
        fan_pin: Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default()),
    };

    loop {
        info!("Opening relay");
        thermostat.turn_heat_off();
        let mut t0 = Instant::now();
        while t0.elapsed() < Duration::from_millis(1000) {}

        info!("Closing relay");
        thermostat.turn_heat_on();
        t0 = Instant::now();
        while t0.elapsed() < Duration::from_millis(1000) {}
    }
}
