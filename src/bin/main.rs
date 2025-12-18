#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embedded_sht3x::{DEFAULT_I2C_ADDRESS, Repeatability, Sht3x};
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    i2c::master::{Config, I2c},
    main,
    time::{Duration, Instant},
};

use core::{cell::RefCell, fmt::Write};
use embedded_hal_bus::i2c::RefCellDevice;
use esp_println::println;
use heapless::String;

use grove_lcd_rgb::GroveLcd;
use thermostat::{OFF, ON, sensor, thermostat::*};
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Init peripherals
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let delay_lcd = Delay::new();
    let delay_temp = Delay::new();

    // Heap allocation
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    esp_alloc::heap_allocator!(size: 64 * 1024); //TODO: Added by esp-gen with "COEX needs more RAM - so we've added some more." Investigate.

    let mut thermostat = Thermostat {
        heat: OFF,
        ac: OFF,
        fan: OFF,
        fan_mode: FanMode::On,
        temp: 70.0,
        _humidity: 40.0,
        set_point_low: 70,
        set_point_high: 70,
        mode: Mode::Off,
        backlight: ON,
        backlight_since: Instant::now(),
        heat_pin: Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default()),
        ac_pin: Output::new(peripherals.GPIO3, Level::Low, OutputConfig::default()),
        fan_pin: Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default()),
        hysteresis: 0.9,
    };

    // Create I2C peripheral
    let sda = peripherals.GPIO6;
    let scl = peripherals.GPIO7;

    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .unwrap()
        .with_sda(sda)
        .with_scl(scl);

    // Wrap the I2C bus in RefCell for sharing
    let i2c_bus = RefCell::new(i2c);

    // Create shared devices for each peripheral
    let lcd_i2c = RefCellDevice::new(&i2c_bus);
    let i2c_temp = RefCellDevice::new(&i2c_bus);

    // Create LCD instance
    let mut lcd = GroveLcd::new(lcd_i2c, delay_lcd);

    // Initialize the LCD (16 columns, 2 rows)
    lcd.begin(16, 2).unwrap();

    // Set lcd backlight to white
    lcd.set_rgb(255, 255, 255).unwrap();

    // Initialize the SHT-31 sensor
    let mut sensor = Sht3x::new(i2c_temp, DEFAULT_I2C_ADDRESS, delay_temp);

    // Set measurement repeatability to High for best accuracy
    sensor.repeatability = Repeatability::High;

    println!("SHT-31 sensor initialized successfully!");
    println!("Repeatability: High (best accuracy)");
    println!("I2C Address: 0x{:02X}\n", DEFAULT_I2C_ADDRESS);

    loop {
        // TODO: Read buttons (if you don't do interrupts)

        // Read temperature and humidity
        if let Err(_) = sensor::read_and_update_sensor(&mut sensor, &mut thermostat) {
            esp_println::println!("Warning: Unable to read sensor data after retries");
            lcd.clear().unwrap();
            lcd.set_cursor(0, 0).unwrap();
            lcd.print("Sensor Error").unwrap();
            continue;
        }

        // Determine heating/cooling requirements based on mode
        thermostat.determine_hvac_action();

        // Control fan based on fan mode and HVAC state
        thermostat.control_fan();

        // Apply hardware states with safety checks
        thermostat.apply_hardware_states();

        // TODO: Update LCD
    }
}
