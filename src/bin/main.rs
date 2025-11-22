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
    gpio::{Level, Output, OutputConfig},
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
    let delay = Delay::new();

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
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .unwrap()
        .with_sda(sda)  // SDA
        .with_scl(scl); // SCL

    // Create LCD instance
    let mut lcd = GroveLcd::new(i2c, delay);
    
    // Initialize the LCD (16 columns, 2 rows)
    lcd.begin(16, 2).unwrap();
    
    // Set backlight to cyan
    lcd.set_rgb(0, 255, 255).unwrap();
    
    // Print "Hello, ESP32!" on first line
    lcd.set_cursor(0, 0).unwrap();
    lcd.print("Hello, ESP32!").unwrap();
    
    // Print "Grove LCD" on second line
    lcd.set_cursor(0, 1).unwrap();
    lcd.print("Grove LCD").unwrap();
    
    let mut counter = 0u32;
    
    loop {
        // Update counter on second line
        lcd.set_cursor(0, 1).unwrap();
        
        // Format counter as string (simple approach for no_std)
        let mut buffer = [0u8; 16];
        let mut pos = 0;
        let mut n = counter;
        
        if n == 0 {
            buffer[pos] = b'0';
            pos += 1;
        } else {
            let mut divisor = 1_000_000_000;
            let mut started = false;
            
            while divisor > 0 {
                let digit = (n / divisor) as u8;
                if digit > 0 || started {
                    buffer[pos] = b'0' + digit;
                    pos += 1;
                    started = true;
                }
                n %= divisor;
                divisor /= 10;
            }
        }
        
        // Print counter
        for i in 0..pos {
            lcd.write(buffer[i]).unwrap();
        }
        
        // Clear rest of line
        for _ in pos..16 {
            lcd.write(b' ').unwrap();
        }
        
        // Change backlight color based on counter
        let phase = (counter / 10) % 6;
        match phase {
            0 => lcd.set_rgb(255, 0, 0).unwrap(),     // Red
            1 => lcd.set_rgb(255, 255, 0).unwrap(),   // Yellow
            2 => lcd.set_rgb(0, 255, 0).unwrap(),     // Green
            3 => lcd.set_rgb(0, 255, 255).unwrap(),   // Cyan
            4 => lcd.set_rgb(0, 0, 255).unwrap(),     // Blue
            5 => lcd.set_rgb(255, 0, 255).unwrap(),   // Magenta
            _ => {}
        }
        
        counter += 1;
        delay.delay_millis(100);
    }


    // loop {
    //     info!("Opening relay");
    //     thermostat.turn_heat_off();
    //     let mut t0 = Instant::now();
    //     while t0.elapsed() < Duration::from_millis(1000) {}

    //     info!("Closing relay");
    //     thermostat.turn_heat_on();
    //     t0 = Instant::now();
    //     while t0.elapsed() < Duration::from_millis(1000) {}

    // }
}
