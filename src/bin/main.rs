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

    // Configure GPIO8 as output:
    let mut pin8 = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    loop {
        info!("Opening relay");
        set_relay_state(&mut pin8, false);
        let mut t0 = Instant::now();
        while t0.elapsed() < Duration::from_millis(1000) {}

        info!("Closing relay");
        set_relay_state(&mut pin8, true);
        t0 = Instant::now();
        while t0.elapsed() < Duration::from_millis(1000) {}
    }
}

fn set_relay_state(relay_pin: &mut Output, close: bool) {
    if close {
        relay_pin.set_high();
    } else {
        relay_pin.set_low();
    }
}
