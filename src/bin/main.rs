// #![no_std]
// #![no_main]
// #![deny(
//     clippy::mem_forget,
//     reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
//     holding buffers for the duration of a data transfer."
// )]

// use defmt::*;
// use esp_hal::{
//     i2c::master::{Config, I2c},
//     clock::CpuClock,
//     gpio::{Level, Output, OutputConfig},
//     main,
//     time::{Duration, Instant},
//     delay::Delay,
// };
// use thermostat::{OFF, ON, thermostat::*};
// use {esp_backtrace as _, esp_println as _};
// use grove_lcd_rgb::GroveLcd;

// extern crate alloc;

// // This creates a default app-descriptor required by the esp-idf bootloader.
// // For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
// esp_bootloader_esp_idf::esp_app_desc!();

// #[main]
// fn main() -> ! {
//     info!("start");

//     // Init peripherals
//     let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
//     let peripherals = esp_hal::init(config);

//     // Heap allocation
//     esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
//     esp_alloc::heap_allocator!(size: 64 * 1024); //TODO: Added by esp-gen with "COEX needs more RAM - so we've added some more." Investigate.

//     let mut thermostat = Thermostat {
//         heat: OFF,
//         cool: OFF,
//         fan: OFF,
//         fan_mode: FanMode::Off,
//         temp: 70,
//         set_point_low: 70,
//         set_point_high: 70,
//         mode: Mode::Off,
//         backlight: ON,
//         backlight_since: Instant::now(),
//         heat_pin: Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default()),
//         cool_pin: Output::new(peripherals.GPIO3, Level::Low, OutputConfig::default()),
//         fan_pin: Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default()),
//     };

//     let sda = peripherals.GPIO6;
//     let scl = peripherals.GPIO7;

//     // // Create I2C peripheral
//     // // Adjust pin numbers based on your wiring
//     // let mut i2c = I2c::new(peripherals.I2C0, Config::default())
//     //     .unwrap()
//     //     .with_sda(sda)  // SDA
//     //     .with_scl(scl); // SCL

//     // // Create a delay provider
//     // let delay = Delay::new();
    
//     // delay.delay_millis(100);
    
//     // // Step 2: Scan I2C bus
//     // println!("\nStep 2: Scanning I2C bus...");
//     // let mut lcd_found = false;
//     // let mut rgb_found = false;
    
//     // for addr in 0x00..=0x7F {
//     //     if i2c.write(addr, &[]).is_ok() {
//     //         info!("  ✓ Found device at 0x{:02X}", addr);
//     //         if addr == 0x3E {
//     //             lcd_found = true;
//     //             info!("    → LCD text controller detected");
//     //         }
//     //         if addr == 0x62 {
//     //             rgb_found = true;
//     //             info!("    → RGB backlight controller detected (v4)");
//     //         }
//     //         if addr == 0x30 {
//     //             rgb_found = true;
//     //             info!("    → RGB backlight controller detected (v5)");
//     //         }
//     //     }
//     // }
    
//     // if !lcd_found {
//     //     println!("\n  ✗ ERROR: LCD not found at address 0x3E");
//     //     println!("  Check:");
//     //     println!("    - Is the LCD powered? (needs 3.3V or 5V)");
//     //     println!("    - Are SDA and SCL connected correctly?");
//     //     println!("    - Do you have pull-up resistors? (usually built into Grove modules)");
//     // }
    
//     // if !rgb_found {
//     //     println!("\n  ⚠ WARNING: RGB backlight controller not found");
//     //     println!("  The display may work but backlight might not");
//     // }

//     // // Step 3: Try basic LCD communication
//     // println!("\nStep 3: Testing LCD communication...");
    
//     // // Try to send a command to the LCD (function set)
//     // let cmd_data = [0x80, 0x38]; // Function set: 8-bit, 2 lines, 5x8 dots
//     // match i2c.write(0x3E, &cmd_data) {
//     //     Ok(_) => println!("  ✓ Successfully sent command to LCD"),
//     //     Err(e) => {
//     //         println!("  ✗ Failed to send command: {:?}", e);
//     //         //loop {}
//     //     }
//     // }

//     // // Step 4: Test RGB controller
//     // if rgb_found {
//     //     println!("\nStep 4: Testing RGB backlight...");
        
//     //     // Try to set RGB to red
//     //     let rgb_addr = if i2c.write(0x30, &[]).is_ok() { 0x30 } else { 0x62 };
        
//     //     match i2c.write(rgb_addr, &[0x04, 0xFF]) {  // Red register
//     //         Ok(_) => println!("  ✓ Successfully set red channel"),
//     //         Err(e) => println!("  ⚠ Failed to set red: {:?}", e),
//     //     }
        
//     //     match i2c.write(rgb_addr, &[0x03, 0x00]) {  // Green register
//     //         Ok(_) => println!("  ✓ Successfully set green channel"),
//     //         Err(e) => println!("  ⚠ Failed to set green: {:?}", e),
//     //     }
        
//     //     match i2c.write(rgb_addr, &[0x02, 0x00]) {  // Blue register
//     //         Ok(_) => println!("  ✓ Successfully set blue channel"),
//     //         Err(e) => println!("  ⚠ Failed to set blue: {:?}", e),
//     //     }
        
//     //     println!("  → Backlight should now be RED");
//     // }


//     // loop {
//     //     info!("Opening relay");
//     //     thermostat.turn_heat_off();
//     //     let mut t0 = Instant::now();
//     //     while t0.elapsed() < Duration::from_millis(1000) {}

//     //     info!("Closing relay");
//     //     thermostat.turn_heat_on();
//     //     t0 = Instant::now();
//     //     while t0.elapsed() < Duration::from_millis(1000) {}
//     // }

    
// }

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
use esp_println::{print, println};

#[esp_hal::main]
fn main() -> ! {
    println!("\n╔═══════════════════════════════════════╗");
    println!("║   Separate LCD/RGB Device Test       ║");
    println!("╚═══════════════════════════════════════╝\n");
    
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    // Heap allocation
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    esp_alloc::heap_allocator!(size: 64 * 1024); //TODO: Added by esp-gen with "COEX needs more RAM - so we've added some more." Investigate.
    
    // Create I2C
    let mut i2c = I2c::new(peripherals.I2C0, Config::default())
        .expect("Failed to create I2C")
        .with_sda(peripherals.GPIO6)
        .with_scl(peripherals.GPIO7);
    
    println!("I2C initialized at default speed on GPIO6 (SDA) and GPIO7 (SCL)\n");
    
    // Power-on delay
    println!("Waiting for devices to power up...");
    delay.delay_millis(500);
    
    // Initial scan
    println!("\nInitial device scan:");
    let lcd_present = scan_for_device(&mut i2c, &mut delay, 0x3E, "LCD Controller");
    let rgb_v4_present = scan_for_device(&mut i2c, &mut delay, 0x62, "RGB v4");
    let rgb_v5_present = scan_for_device(&mut i2c, &mut delay, 0x30, "RGB v5");
    
    println!("\n╔═══════════════════════════════════════╗");
    println!("║           Diagnosis                   ║");
    println!("╚═══════════════════════════════════════╝\n");
    
    match (lcd_present, rgb_v4_present || rgb_v5_present) {
        (true, true) => {
            println!("✓ Both LCD and RGB detected - this should work!");
            test_full_display(&mut i2c, &mut delay, rgb_v5_present);
        },
        (true, false) => {
            println!("⚠ LCD found but RGB missing");
            println!("\nPossible causes:");
            println!("  1. RGB controller is dead/damaged");
            println!("  2. Different I2C bus (some boards have 2 buses)");
            println!("  3. RGB needs different initialization");
            println!("\nTesting LCD only...");
            test_lcd_only(&mut i2c, &mut delay);
        },
        (false, true) => {
            println!("⚠ RGB found but LCD missing");
            println!("\nPossible causes:");
            println!("  1. LCD controller is dead/damaged");
            println!("  2. LCD and RGB on separate I2C buses");
            println!("  3. LCD needs reset/power cycle");
            println!("\nTesting RGB only...");
            test_rgb_only(&mut i2c, &mut delay, rgb_v5_present);
        },
        (false, false) => {
            println!("✗ Neither device detected!");
            println!("\nChecklist:");
            println!("  □ Is LCD powered? (check VCC)");
            println!("  □ Are SDA/SCL connected correctly?");
            println!("  □ Are you using the right GPIO pins?");
            println!("  □ Does LCD work with Arduino?");
            println!("\nTrying recovery...");
            attempt_recovery(&mut i2c, &mut delay);
        },
    }
    
    loop {}
}

fn scan_for_device(
    i2c: &mut I2c<'_, esp_hal::Blocking>,
    delay: &mut Delay,
    addr: u8,
    name: &str,
) -> bool {
    print!("  Checking 0x{:02X} ({})... ", addr, name);
    
    // Try multiple times
    let mut found = false;
    for _ in 0..5 {
        if i2c.write(addr, &[]).is_ok() {
            found = true;
            break;
        }
        delay.delay_millis(10);
    }
    
    if found {
        println!("✓ FOUND");
    } else {
        println!("✗ Not found");
    }
    
    found
}

fn test_lcd_only(i2c: &mut I2c<'_, esp_hal::Blocking>, delay: &mut Delay) {
    println!("\n--- Testing LCD Controller Only ---\n");
    
    // Send initialization sequence
    println!("Initializing LCD...");
    
    delay.delay_millis(50);
    
    // Function set commands
    let _ = i2c.write(0x3E, &[0x80, 0x38]); // 8-bit, 2-line, 5x8
    delay.delay_millis(5);
    
    let _ = i2c.write(0x3E, &[0x80, 0x38]);
    delay.delay_millis(5);
    
    let _ = i2c.write(0x3E, &[0x80, 0x38]);
    delay.delay_millis(5);
    
    // Display control
    let _ = i2c.write(0x3E, &[0x80, 0x0C]); // Display on, cursor off
    delay.delay_millis(5);
    
    // Clear display
    let _ = i2c.write(0x3E, &[0x80, 0x01]);
    delay.delay_millis(5);
    
    // Entry mode
    let _ = i2c.write(0x3E, &[0x80, 0x06]); // Increment, no shift
    delay.delay_millis(5);
    
    println!("Writing text: 'LCD ONLY'");
    
    // Write characters
    for ch in b"LCD ONLY" {
        match i2c.write(0x3E, &[0x40, *ch]) {
            Ok(_) => print!("."),
            Err(_) => print!("X"),
        }
        delay.delay_millis(10);
    }
    
    println!("\n\nIf you see 'LCD ONLY' on the display:");
    println!("  → LCD works! RGB controller may be damaged.");
    println!("If display is blank:");
    println!("  → LCD controller may be damaged or needs 5V power.");
}

fn test_rgb_only(i2c: &mut I2c<'_, esp_hal::Blocking>, delay: &mut Delay, is_v5: bool) {
    println!("\n--- Testing RGB Controller Only ---\n");
    
    let addr = if is_v5 { 0x30 } else { 0x62 };
    println!("Using address: 0x{:02X} ({})", addr, if is_v5 { "v5" } else { "v4" });
    
    if is_v5 {
        // v5 initialization
        println!("Initializing v5 RGB controller...");
        let _ = i2c.write(addr, &[0x00, 0x07]); // Reset
        delay.delay_millis(200);
        let _ = i2c.write(addr, &[0x04, 0x15]); // Set PWM - all LEDs always on
        delay.delay_millis(10);
    } else {
        // v4 initialization
        println!("Initializing v4 RGB controller...");
        let _ = i2c.write(addr, &[0x00, 0x00]); // MODE1
        delay.delay_millis(10);
        let _ = i2c.write(addr, &[0x01, 0x00]); // MODE2
        delay.delay_millis(10);
        let _ = i2c.write(addr, &[0x08, 0xFF]); // Output
        delay.delay_millis(10);
    }
    
    println!("Cycling through colors...");
    
    let colors = [
        (255, 0, 0, "RED"),
        (0, 255, 0, "GREEN"),
        (0, 0, 255, "BLUE"),
        (255, 255, 0, "YELLOW"),
        (255, 0, 255, "MAGENTA"),
        (0, 255, 255, "CYAN"),
        (255, 255, 255, "WHITE"),
    ];
    
    for (r, g, b, name) in colors {
        println!("  Setting {}", name);
        
        if is_v5 {
            // v5 uses registers 0x06, 0x07, 0x08
            let _ = i2c.write(addr, &[0x06, r]); // Red
            let _ = i2c.write(addr, &[0x07, g]); // Green
            let _ = i2c.write(addr, &[0x08, b]); // Blue
        } else {
            // v4 uses registers 0x04, 0x03, 0x02
            let _ = i2c.write(addr, &[0x04, r]); // Red
            let _ = i2c.write(addr, &[0x03, g]); // Green
            let _ = i2c.write(addr, &[0x02, b]); // Blue
        }
        
        delay.delay_millis(1000);
    }
    
    println!("\nIf you saw the backlight change colors:");
    println!("  → RGB works! LCD controller may be damaged.");
    println!("If backlight stayed off:");
    println!("  → RGB controller may be damaged or need different init.");
}

fn test_full_display(i2c: &mut I2c<'_, esp_hal::Blocking>, delay: &mut Delay, is_v5: bool) {
    println!("\n--- Testing Full Display ---\n");
    
    // Initialize RGB
    let rgb_addr = if is_v5 { 0x30 } else { 0x62 };
    
    if is_v5 {
        let _ = i2c.write(rgb_addr, &[0x00, 0x07]);
        delay.delay_millis(200);
        let _ = i2c.write(rgb_addr, &[0x04, 0x15]);
    } else {
        let _ = i2c.write(rgb_addr, &[0x00, 0x00]);
        let _ = i2c.write(rgb_addr, &[0x01, 0x00]);
        let _ = i2c.write(rgb_addr, &[0x08, 0xFF]);
    }
    
    delay.delay_millis(10);
    
    // Set green backlight
    println!("Setting GREEN backlight...");
    if is_v5 {
        // v5 registers: 0x06=R, 0x07=G, 0x08=B
        let _ = i2c.write(rgb_addr, &[0x06, 0]);   // Red off
        let _ = i2c.write(rgb_addr, &[0x07, 255]); // Green on
        let _ = i2c.write(rgb_addr, &[0x08, 0]);   // Blue off
    } else {
        // v4 registers: 0x04=R, 0x03=G, 0x02=B
        let _ = i2c.write(rgb_addr, &[0x04, 0]);   // Red off
        let _ = i2c.write(rgb_addr, &[0x03, 255]); // Green on
        let _ = i2c.write(rgb_addr, &[0x02, 0]);   // Blue off
    }
    
    delay.delay_millis(100);
    
    // Initialize LCD
    println!("Initializing LCD...");
    let _ = i2c.write(0x3E, &[0x80, 0x38]);
    delay.delay_millis(5);
    let _ = i2c.write(0x3E, &[0x80, 0x0C]);
    delay.delay_millis(5);
    let _ = i2c.write(0x3E, &[0x80, 0x01]);
    delay.delay_millis(5);
    let _ = i2c.write(0x3E, &[0x80, 0x06]);
    delay.delay_millis(5);
    
    // Write text
    println!("Writing 'SUCCESS!'...");
    for ch in b"I LOVE YOU!" {//FIXME
        let _ = i2c.write(0x3E, &[0x40, *ch]);
        delay.delay_millis(10);
    }
    
    println!("\n✓ Test complete!");
    println!("You should see 'SUCCESS!' on a green background.");
}

fn attempt_recovery(i2c: &mut I2c<'_, esp_hal::Blocking>, delay: &mut Delay) {
    println!("\nAttempting recovery procedures...\n");
    
    // Try general call address
    println!("1. Trying general call address (0x00)...");
    let _ = i2c.write(0x00, &[0x06]); // Software reset
    delay.delay_millis(100);
    
    // Scan again
    println!("2. Re-scanning after reset...");
    let lcd = scan_for_device(i2c, delay, 0x3E, "LCD");
    let rgb_v4 = scan_for_device(i2c, delay, 0x62, "RGB v4");
    let rgb_v5 = scan_for_device(i2c, delay, 0x30, "RGB v5");
    
    if lcd || rgb_v4 || rgb_v5 {
        println!("\n✓ Found devices after recovery!");
    } else {
        println!("\n✗ Still no devices. Hardware issue likely.");
        println!("\nFinal checks:");
        println!("  • Measure voltage at LCD VCC (should be 3.3V or 5V)");
        println!("  • Check continuity of SDA/SCL wires");
        println!("  • Try powering LCD with external 5V supply");
        println!("  • Test LCD with a working Arduino setup");
    }
}
