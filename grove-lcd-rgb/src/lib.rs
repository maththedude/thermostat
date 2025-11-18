//! Grove LCD RGB Backlight Driver
//!
//! A platform-agnostic driver for the Grove LCD RGB Backlight display using embedded-hal traits.
//! This driver supports the 16x2 character LCD with RGB backlight control via I2C.
//!
//! # Features
//! - Full LCD control (clear, cursor positioning, etc.)
//! - RGB backlight color control
//! - Custom character support
//! - Display scrolling
//! - Cursor and blink control
//!
//! # Example
//! ```no_run
//! use grove_lcd_rgb::GroveLcd;
//! 
//! let mut lcd = GroveLcd::new(i2c);
//! lcd.begin(16, 2).unwrap();
//! lcd.set_rgb(255, 0, 0).unwrap(); // Red backlight
//! lcd.print("Hello, World!").unwrap();
//! ```

#![no_std]

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

/// I2C address for the LCD text controller
pub const LCD_ADDRESS: u8 = 0x3E;

/// I2C address for the RGB backlight controller
pub const RGB_ADDRESS: u8 = 0x62;

/// Alternative RGB address for v5 hardware
pub const RGB_ADDRESS_V5: u8 = 0x30;

// LCD Commands
const LCD_CLEARDISPLAY: u8 = 0x01;
const LCD_RETURNHOME: u8 = 0x02;
const LCD_ENTRYMODESET: u8 = 0x04;
const LCD_DISPLAYCONTROL: u8 = 0x08;
const LCD_CURSORSHIFT: u8 = 0x10;
const LCD_FUNCTIONSET: u8 = 0x20;
const LCD_SETCGRAMADDR: u8 = 0x40;
const LCD_SETDDRAMADDR: u8 = 0x80;

// Entry Mode flags
const LCD_ENTRYRIGHT: u8 = 0x00;
const LCD_ENTRYLEFT: u8 = 0x02;
const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;
const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;

// Display Control flags
const LCD_DISPLAYON: u8 = 0x04;
const LCD_DISPLAYOFF: u8 = 0x00;
const LCD_CURSORON: u8 = 0x02;
const LCD_CURSOROFF: u8 = 0x00;
const LCD_BLINKON: u8 = 0x01;
const LCD_BLINKOFF: u8 = 0x00;

// Cursor Shift flags
const LCD_DISPLAYMOVE: u8 = 0x08;
const LCD_CURSORMOVE: u8 = 0x00;
const LCD_MOVERIGHT: u8 = 0x04;
const LCD_MOVELEFT: u8 = 0x00;

// Function Set flags
const LCD_8BITMODE: u8 = 0x10;
const LCD_4BITMODE: u8 = 0x00;
const LCD_2LINE: u8 = 0x08;
const LCD_1LINE: u8 = 0x00;
const LCD_5x10DOTS: u8 = 0x04;
const LCD_5x8DOTS: u8 = 0x00;

// RGB Registers
const REG_MODE1: u8 = 0x00;
const REG_MODE2: u8 = 0x01;
const REG_OUTPUT: u8 = 0x08;
const REG_RED: u8 = 0x04;
const REG_GREEN: u8 = 0x03;
const REG_BLUE: u8 = 0x02;

/// Dot size for LCD characters
#[derive(Debug, Clone, Copy)]
pub enum DotSize {
    /// 5x8 dots per character (default)
    Dots5x8,
    /// 5x10 dots per character (only for 1-line displays)
    Dots5x10,
}

/// Errors that can occur when using the LCD
#[derive(Debug, Clone, Copy)]
pub enum LcdError<E> {
    /// I2C communication error
    I2c(E),
    /// Invalid parameter provided
    InvalidParameter,
}

impl<E: core::fmt::Debug> core::fmt::Display for LcdError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LcdError::I2c(e) => write!(f, "I2C Error: {:?}", e),
            LcdError::InvalidParameter => write!(f, "Invalid parameter"),
        }
    }
}

impl<E> From<E> for LcdError<E> {
    fn from(error: E) -> Self {
        LcdError::I2c(error)
    }
}

/// Grove LCD RGB Backlight driver
pub struct GroveLcd<I2C, D> {
    i2c: I2C,
    delay: D,
    display_function: u8,
    display_control: u8,
    display_mode: u8,
    num_lines: u8,
    curr_line: u8,
    rgb_addr: u8,
}

impl<I2C, D, E> GroveLcd<I2C, D>
where
    I2C: I2c<Error = E>,
    D: DelayNs,
{
    /// Create a new Grove LCD instance
    pub fn new(i2c: I2C, delay: D) -> Self {
        Self {
            i2c,
            delay,
            display_function: 0,
            display_control: 0,
            display_mode: 0,
            num_lines: 0,
            curr_line: 0,
            rgb_addr: RGB_ADDRESS,
        }
    }

    /// Initialize the LCD with the specified columns and rows
    ///
    /// # Arguments
    /// * `cols` - Number of columns (typically 16)
    /// * `rows` - Number of rows (typically 2)
    pub fn begin(&mut self, cols: u8, rows: u8) -> Result<(), LcdError<E>> {
        self.begin_with_dotsize(cols, rows, DotSize::Dots5x8)
    }

    /// Initialize the LCD with specified columns, rows, and dot size
    pub fn begin_with_dotsize(
        &mut self,
        _cols: u8,
        lines: u8,
        dotsize: DotSize,
    ) -> Result<(), LcdError<E>> {
        if lines > 1 {
            self.display_function |= LCD_2LINE;
        }
        self.num_lines = lines;
        self.curr_line = 0;

        // For some 1-line displays you can select a 10-pixel high font
        if matches!(dotsize, DotSize::Dots5x10) && lines == 1 {
            self.display_function |= LCD_5x10DOTS;
        }

        // Wait for LCD to power up
        self.delay.delay_ms(50);

        // Initialize the display following HD44780 datasheet procedure
        self.command(LCD_FUNCTIONSET | self.display_function)?;
        self.delay.delay_us(4500);

        self.command(LCD_FUNCTIONSET | self.display_function)?;
        self.delay.delay_us(150);

        self.command(LCD_FUNCTIONSET | self.display_function)?;
        self.command(LCD_FUNCTIONSET | self.display_function)?;

        // Turn display on with no cursor or blinking
        self.display_control = LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF;
        self.display()?;

        // Clear display
        self.clear()?;

        // Set text direction (left to right)
        self.display_mode = LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT;
        self.command(LCD_ENTRYMODESET | self.display_mode)?;

        // Detect and initialize RGB backlight chip
        // Try v5 address first
        if self.i2c.write(RGB_ADDRESS_V5, &[]).is_ok() {
            self.rgb_addr = RGB_ADDRESS_V5;
            self.set_reg(0x00, 0x07)?; // Reset
            self.delay.delay_us(200);
            self.set_reg(0x04, 0x15)?; // Set all LEDs always on
        } else {
            self.rgb_addr = RGB_ADDRESS;
            self.set_reg(REG_MODE1, 0)?;
            self.set_reg(REG_OUTPUT, 0xFF)?;
            self.set_reg(REG_MODE2, 0x20)?;
        }

        // Set default white backlight
        self.set_rgb(255, 255, 255)?;

        Ok(())
    }

    /// Clear the display
    pub fn clear(&mut self) -> Result<(), LcdError<E>> {
        self.command(LCD_CLEARDISPLAY)?;
        self.delay.delay_ms(2);
        Ok(())
    }

    /// Return cursor to home position (0, 0)
    pub fn home(&mut self) -> Result<(), LcdError<E>> {
        self.command(LCD_RETURNHOME)?;
        self.delay.delay_ms(2);
        Ok(())
    }

    /// Set cursor position
    ///
    /// # Arguments
    /// * `col` - Column (0-15 for 16-column display)
    /// * `row` - Row (0-1 for 2-row display)
    pub fn set_cursor(&mut self, col: u8, row: u8) -> Result<(), LcdError<E>> {
        let row = row.min(self.num_lines.saturating_sub(1));
        let val = if row == 0 {
            col | 0x80
        } else {
            col | 0xc0
        };
        
        let data = [0x80, val];
        self.i2c.write(LCD_ADDRESS, &data)?;
        Ok(())
    }

    /// Turn display off (data remains in memory)
    pub fn no_display(&mut self) -> Result<(), LcdError<E>> {
        self.display_control &= !LCD_DISPLAYON;
        self.command(LCD_DISPLAYCONTROL | self.display_control)
    }

    /// Turn display on
    pub fn display(&mut self) -> Result<(), LcdError<E>> {
        self.display_control |= LCD_DISPLAYON;
        self.command(LCD_DISPLAYCONTROL | self.display_control)
    }

    /// Turn cursor off
    pub fn no_cursor(&mut self) -> Result<(), LcdError<E>> {
        self.display_control &= !LCD_CURSORON;
        self.command(LCD_DISPLAYCONTROL | self.display_control)
    }

    /// Turn cursor on
    pub fn cursor(&mut self) -> Result<(), LcdError<E>> {
        self.display_control |= LCD_CURSORON;
        self.command(LCD_DISPLAYCONTROL | self.display_control)
    }

    /// Turn cursor blinking off
    pub fn no_blink(&mut self) -> Result<(), LcdError<E>> {
        self.display_control &= !LCD_BLINKON;
        self.command(LCD_DISPLAYCONTROL | self.display_control)
    }

    /// Turn cursor blinking on
    pub fn blink(&mut self) -> Result<(), LcdError<E>> {
        self.display_control |= LCD_BLINKON;
        self.command(LCD_DISPLAYCONTROL | self.display_control)
    }

    /// Scroll display left without changing RAM
    pub fn scroll_display_left(&mut self) -> Result<(), LcdError<E>> {
        self.command(LCD_CURSORSHIFT | LCD_DISPLAYMOVE | LCD_MOVELEFT)
    }

    /// Scroll display right without changing RAM
    pub fn scroll_display_right(&mut self) -> Result<(), LcdError<E>> {
        self.command(LCD_CURSORSHIFT | LCD_DISPLAYMOVE | LCD_MOVERIGHT)
    }

    /// Set text flow direction left to right
    pub fn left_to_right(&mut self) -> Result<(), LcdError<E>> {
        self.display_mode |= LCD_ENTRYLEFT;
        self.command(LCD_ENTRYMODESET | self.display_mode)
    }

    /// Set text flow direction right to left
    pub fn right_to_left(&mut self) -> Result<(), LcdError<E>> {
        self.display_mode &= !LCD_ENTRYLEFT;
        self.command(LCD_ENTRYMODESET | self.display_mode)
    }

    /// Enable autoscroll (display shifts with each character)
    pub fn autoscroll(&mut self) -> Result<(), LcdError<E>> {
        self.display_mode |= LCD_ENTRYSHIFTINCREMENT;
        self.command(LCD_ENTRYMODESET | self.display_mode)
    }

    /// Disable autoscroll
    pub fn no_autoscroll(&mut self) -> Result<(), LcdError<E>> {
        self.display_mode &= !LCD_ENTRYSHIFTINCREMENT;
        self.command(LCD_ENTRYMODESET | self.display_mode)
    }

    /// Create a custom character
    ///
    /// # Arguments
    /// * `location` - Character location (0-7)
    /// * `charmap` - 8-byte array defining the character bitmap
    pub fn create_char(&mut self, location: u8, charmap: &[u8; 8]) -> Result<(), LcdError<E>> {
        let location = location & 0x7;
        self.command(LCD_SETCGRAMADDR | (location << 3))?;
        
        for &byte in charmap {
            self.write_data(byte)?;
        }
        
        Ok(())
    }

    /// Set RGB backlight color
    ///
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) -> Result<(), LcdError<E>> {
        self.set_reg(REG_RED, r)?;
        self.set_reg(REG_GREEN, g)?;
        self.set_reg(REG_BLUE, b)?;
        Ok(())
    }

    /// Turn off backlight
    pub fn backlight_off(&mut self) -> Result<(), LcdError<E>> {
        self.set_rgb(0, 0, 0)
    }

    /// Set backlight to white
    pub fn backlight_white(&mut self) -> Result<(), LcdError<E>> {
        self.set_rgb(255, 255, 255)
    }

    /// Print a string to the LCD
    pub fn print(&mut self, s: &str) -> Result<(), LcdError<E>> {
        for c in s.chars() {
            self.write_data(c as u8)?;
        }
        Ok(())
    }

    /// Print a single byte to the LCD
    pub fn write(&mut self, value: u8) -> Result<(), LcdError<E>> {
        self.write_data(value)
    }

    // Private helper methods

    fn command(&mut self, value: u8) -> Result<(), LcdError<E>> {
        let data = [0x80, value];
        self.i2c.write(LCD_ADDRESS, &data)?;
        Ok(())
    }

    fn write_data(&mut self, value: u8) -> Result<(), LcdError<E>> {
        let data = [0x40, value];
        self.i2c.write(LCD_ADDRESS, &data)?;
        Ok(())
    }

    fn set_reg(&mut self, reg: u8, value: u8) -> Result<(), LcdError<E>> {
        let data = [reg, value];
        self.i2c.write(self.rgb_addr, &data)?;
        Ok(())
    }

    /// Consume the driver and return the I2C peripheral
    pub fn release(self) -> (I2C, D) {
        (self.i2c, self.delay)
    }
}