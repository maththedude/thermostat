#![no_std]
#![no_main]

use defmt::*;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    info!("start");

    // Init peripherals
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Heap allocation
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    esp_alloc::heap_allocator!(size: 64 * 1024);

    // Configure GPIO8 as output:
    let mut pin8 = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    loop {
        info!("Flipping");
        pin8.toggle();

        let t0 = Instant::now();
        while t0.elapsed() < Duration::from_millis(1000) {}
    }
}