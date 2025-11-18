//! Minimal Grove LCD RGB Backlight driver (pure embedded-hal version)
//!
//! Compatible with any HAL that implements:
//!   - embedded_hal::i2c::I2c
//!   - embedded_hal::delay::DelayNs

use defmt::info;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

pub struct GroveLcdRgb<I2C, DELAY> {
    i2c: I2C,
    delay: DELAY,
}

impl<I2C, DELAY, E> GroveLcdRgb<I2C, DELAY>
where
    I2C: I2c<Error = E>,
    DELAY: DelayNs,
{
    pub fn new(i2c: I2C, delay: DELAY) -> Result<Self, E> {
        let mut lcd = Self { i2c, delay };
        lcd.init()?;
        Ok(lcd)
    }

    fn init(&mut self) -> Result<(), E> {
        // --- RGB controller initialization ---
        self.rgb_write(0x00, 0x00)?; // mode1
        self.rgb_write(0x01, 0x00)?; // mode2
        self.rgb_write(0x08, 0xAA)?; // LED output on

        // --- LCD initialization ---
        self.lcd_command(0x38)?; // function set: 2-line mode
        self.lcd_command(0x39)?; // extended instruction set
        self.lcd_command(0x14)?; // internal OSC frequency
        self.lcd_command(0x70)?; // contrast low bits
        self.lcd_command(0x56)?; // power/icon/contrast high bits
        self.lcd_command(0x6C)?; // follower control

        self.delay.delay_ms(200);

        self.lcd_command(0x38)?; // normal instruction set
        self.lcd_command(0x0C)?; // display on
        self.clear()?;

        Ok(())
    }

    // ----------------- LCD low-level -----------------

    fn lcd_command(&mut self, cmd: u8) -> Result<(), E> {
        self.i2c.write(0x3E, &[0x80, cmd])
    }

    fn lcd_write_char(&mut self, ch: u8) -> Result<(), E> {
        self.i2c.write(0x3E, &[0x40, ch])
    }

    // ----------------- RGB low-level -----------------

    fn rgb_write(&mut self, reg: u8, value: u8) -> Result<(), E> {
        info!("reg: {}, value: {}", reg, value);
        self.i2c.write(0x62, &[reg, value])
    }

    // ----------------- High-level API -----------------

    pub fn clear(&mut self) -> Result<(), E> {
        self.lcd_command(0x01)?;
        self.delay.delay_ms(2);
        Ok(())
    }

    pub fn set_cursor(&mut self, col: u8, row: u8) -> Result<(), E> {
        let row_offsets = [0x00, 0x40];
        let offset = col + row_offsets[row as usize];
        self.lcd_command(0x80 | offset)
    }

    pub fn print(&mut self, text: &str) -> Result<(), E> {
        let mut col = 0u8;
        let mut row = 0u8;

        for b in text.bytes() {
            if b == b'\n' {
                row = (row + 1).min(1);
                col = 0;
                self.set_cursor(col, row)?;
                continue;
            }

            self.lcd_write_char(b)?;
            col += 1;

            if col >= 16 {
                row = (row + 1).min(1);
                col = 0;
                self.set_cursor(col, row)?;
            }
        }

        Ok(())
    }

    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) -> Result<(), E> {
        self.rgb_write(0x04, r)?;
        self.rgb_write(0x03, g)?;
        self.rgb_write(0x02, b)
    }

    pub fn display_on(&mut self) -> Result<(), E> {
        self.lcd_command(0x0C)
    }

    pub fn display_off(&mut self) -> Result<(), E> {
        self.lcd_command(0x08)
    }

    pub fn free(self) -> (I2C, DELAY) {
        (self.i2c, self.delay)
    }
}