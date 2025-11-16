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
    main,
    rmt::Rmt,
    time::{Duration, Instant, Rate},
};
use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
use smart_leds::{RGB8, SmartLedsWrite as _};
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    info!("start");

    // init peripherals
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 64 * 1024);

    //
    // --- RMT setup (choose the pattern that your esp-hal exposes) ---
    //

    // 1) Preferred: Rmt::new takes Rate directly (no unwrap). rmt.<channel> is available.
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80));
    // If your Rmt::new returns Result, change the line above to:
    // let mut rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();

    // Pattern A: channel is already ready to use (no configure method)
    // ---------------------------------------------------------------
    // Use this first. Most recent esp-hal variants pass channels directly to the adapter.
    let tx_channel = rmt.unwrap().channel0;

    // Pattern B: channel must be configured with TxChannelConfig (older/newer variants)
    // ---------------------------------------------------------------
    // If the compiler says `no method named configure` for the channel above,
    // try this block instead (uncomment, and comment Pattern A):
    //
    // use esp_hal::rmt::TxChannelConfig;
    // let cfg = TxChannelConfig::default(); // or .with_idle_output(false) if available
    // let tx_channel = rmt.channel0.configure(cfg);
    // // If configure returns Result, append `.unwrap()`.

    // Pattern C: channel provides a `configure_tx(...)` method on the RMT object
    // ---------------------------------------------------------------
    // Another small variation in some releases:
    // let cfg = esp_hal::rmt::TxChannelConfig::default();
    // let tx_channel = rmt.configure_tx(rmt.channel0, cfg).unwrap();

    //
    // --- Smart LED adapter & buffer (important: pass &mut buffer) ---
    //
    // Create a mutable buffer and pass a &mut reference to the adapter.
    let mut buf = smart_led_buffer!(1); // holds the generated RMT pulses for 1 LED
    let mut leds = SmartLedsAdapter::new(tx_channel, peripherals.GPIO8, &mut buf);

    //
    // --- Blink loop (uses Instant busy-wait to avoid depending on Delay API) ---
    //
    const LEVEL: u8 = 20;
    let mut color = RGB8 { r: LEVEL, g: 0, b: 0 };

    loop {
        info!("Hello world!");
        info!("write color r={} g={} b={}", color.r, color.g, color.b);
        leds.write(core::iter::once(color)).unwrap();

        let t0 = Instant::now();
        while t0.elapsed() < Duration::from_millis(500) {}

        // rotate colors
        let RGB8 { r, g, b } = color;
        color = RGB8 { r: g, g: b, b: r };
    }
}