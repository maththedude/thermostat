#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::*;
use esp_hal::{
    i2c::master::{Config, I2c},
    clock::CpuClock,
    gpio::{Io, Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
    delay::Delay,
};
use thermostat::{OFF, ON, thermostat::*};
use {esp_backtrace as _, esp_println as _};
use grove_lcd_rgb::GroveLcd;

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

    let sda = peripherals.GPIO6;
    let scl = peripherals.GPIO7;

    // Create I2C peripheral
    // Adjust pin numbers based on your wiring
    let mut i2c = I2c::new(peripherals.I2C0, Config::default())
        .unwrap()
        .with_sda(sda)  // SDA
        .with_scl(scl); // SCL

    // Create a delay provider
    let delay = Delay::new();
    
    delay.delay_millis(100);
    
    // Step 2: Scan I2C bus
    println!("\nStep 2: Scanning I2C bus...");
    let mut lcd_found = false;
    let mut rgb_found = false;
    
    for addr in 0x00..=0x7F {
        if i2c.write(addr, &[]).is_ok() {
            println!("  ✓ Found device at 0x{:02X}", addr);
            if addr == 0x3E {
                lcd_found = true;
                println!("    → LCD text controller detected");
            }
            if addr == 0x62 {
                rgb_found = true;
                println!("    → RGB backlight controller detected (v4)");
            }
            if addr == 0x30 {
                rgb_found = true;
                println!("    → RGB backlight controller detected (v5)");
            }
        }
    }
    
    if !lcd_found {
        println!("\n  ✗ ERROR: LCD not found at address 0x3E");
        println!("  Check:");
        println!("    - Is the LCD powered? (needs 3.3V or 5V)");
        println!("    - Are SDA and SCL connected correctly?");
        println!("    - Do you have pull-up resistors? (usually built into Grove modules)");
    }
    
    if !rgb_found {
        println!("\n  ⚠ WARNING: RGB backlight controller not found");
        println!("  The display may work but backlight might not");
    }

    // Step 3: Try basic LCD communication
    println!("\nStep 3: Testing LCD communication...");
    
    // Try to send a command to the LCD (function set)
    let cmd_data = [0x80, 0x38]; // Function set: 8-bit, 2 lines, 5x8 dots
    match i2c.write(0x3E, &cmd_data) {
        Ok(_) => println!("  ✓ Successfully sent command to LCD"),
        Err(e) => {
            println!("  ✗ Failed to send command: {:?}", e);
            loop {}
        }
    }

    // // Step 4: Test RGB controller
    // if rgb_found {
    //     println!("\nStep 4: Testing RGB backlight...");
        
    //     // Try to set RGB to red
    //     let rgb_addr = if i2c.write(0x30, &[]).is_ok() { 0x30 } else { 0x62 };
        
    //     match i2c.write(rgb_addr, &[0x04, 0xFF]) {  // Red register
    //         Ok(_) => println!("  ✓ Successfully set red channel"),
    //         Err(e) => println!("  ⚠ Failed to set red: {:?}", e),
    //     }
        
    //     match i2c.write(rgb_addr, &[0x03, 0x00]) {  // Green register
    //         Ok(_) => println!("  ✓ Successfully set green channel"),
    //         Err(e) => println!("  ⚠ Failed to set green: {:?}", e),
    //     }
        
    //     match i2c.write(rgb_addr, &[0x02, 0x00]) {  // Blue register
    //         Ok(_) => println!("  ✓ Successfully set blue channel"),
    //         Err(e) => println!("  ⚠ Failed to set blue: {:?}", e),
    //     }
        
    //     println!("  → Backlight should now be RED");
    // }

    // Step 5: Send a character to the LCD
    println!("\nStep 5: Writing character to LCD...");
    let char_data = [0x40, b'H']; // Data mode, character 'H'
    match i2c.write(0x3E, &char_data) {
        Ok(_) => {
            println!("  ✓ Successfully wrote character 'H' to LCD");
            println!("  → You should see 'H' on the display");
        },
        Err(e) => {
            println!("  ✗ Failed to write character: {:?}", e);
        }
    }


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
