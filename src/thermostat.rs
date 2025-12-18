use esp_hal::{gpio::Output, time::Instant};

use crate::{OFF, ON, helpers::set_relay_state};

pub struct Thermostat<'a> {
    pub heat: bool,
    pub ac: bool,
    pub fan: bool,
    pub fan_mode: FanMode,
    pub temp: f32,
    pub _humidity: f32, // for future smart home upgrades, or a future ac with built-in humidifier
    pub set_point: i16,
    pub mode: Mode,
    pub backlight: bool,
    pub backlight_since: Instant,
    pub heat_pin: Output<'a>,
    pub ac_pin: Output<'a>,
    pub fan_pin: Output<'a>,
    pub hysteresis: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Off,
    Heat,
    Cool,
    Hold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FanMode {
    On,
    Auto,
}

impl<'a> Thermostat<'a> {
    fn turn_heat_on(&mut self) {
        set_relay_state(&mut self.heat_pin, ON);
    }

    fn turn_heat_off(&mut self) {
        set_relay_state(&mut self.heat_pin, OFF);
    }

    fn turn_ac_on(&mut self) {
        set_relay_state(&mut self.ac_pin, ON);
    }

    fn turn_ac_off(&mut self) {
        set_relay_state(&mut self.ac_pin, OFF);
    }

    fn turn_fan_on(&mut self) {
        set_relay_state(&mut self.fan_pin, ON);
    }

    fn turn_fan_off(&mut self) {
        set_relay_state(&mut self.fan_pin, OFF);
    }

    /// Determine what HVAC action is needed based on temperature and mode
    pub fn determine_hvac_action(&mut self) {
        match self.mode {
            Mode::Off => {
                self.heat = OFF;
                self.ac = OFF;
            }

            Mode::Heat => {
                let target = self.set_point as f32;

                if self.temp < (target - self.hysteresis) {
                    self.heat = ON;
                    self.ac = OFF;
                } else if self.temp > (target + self.hysteresis) {
                    self.heat = OFF;
                    self.ac = OFF;
                }
                // else maintain current state (within hysteresis band)
            }

            Mode::Cool => {
                let target = self.set_point as f32;

                if self.temp > (target + self.hysteresis) {
                    self.ac = ON;
                    self.heat = OFF;
                } else if self.temp < (target - self.hysteresis) {
                    self.ac = OFF;
                    self.heat = OFF;
                }
                // else maintain current state (within hysteresis band)
            }

            Mode::Hold => {
                // Hold a specific temperature
                // Use both heating and cooling to maintain exact temperature
                let target = self.set_point as f32;

                if self.temp < (target - self.hysteresis) {
                    self.heat = ON;
                    self.ac = OFF;
                } else if self.temp > (target + self.hysteresis) {
                    self.ac = ON;
                    self.heat = OFF;
                } else if self.temp >= (target - self.hysteresis)
                    && self.temp <= (target + self.hysteresis)
                {
                    self.heat = OFF;
                    self.ac = OFF;
                }
                // else maintain current state
            }
        }
    }

    /// Control fan based on mode and HVAC state
    pub fn control_fan(&mut self) {
        match self.fan_mode {
            FanMode::On => {
                // Fan always on regardless of heating/cooling
                self.fan = ON;
            }
            FanMode::Auto => {
                // Fan on only when heating or cooling
                if self.heat == ON || self.ac == ON {
                    self.fan = ON;
                } else {
                    self.fan = OFF;
                }
            }
        }
    }

    /// Apply hardware states with safety checks
    pub fn apply_hardware_states(&mut self) {
        // Safety Check 1: Never run furnace and AC simultaneously
        if self.heat == ON && self.ac == ON {
            // Error condition - turn both off and log error
            self.heat = OFF;
            self.ac = OFF;
            esp_println::println!("ERROR: Attempted to run furnace and AC simultaneously");
        }

        // Safety Check 2: Never run furnace or AC without the fan
        if (self.heat == ON || self.ac == ON) && self.fan == OFF {
            // Force fan on when HVAC is running
            self.fan = ON;
            esp_println::println!("WARNING: Fan was off while HVAC active - forcing fan on");
        }

        // Apply states to actual hardware
        if self.heat == ON {
            self.turn_heat_on();
        } else {
            self.turn_heat_off();
        }

        if self.ac == ON {
            self.turn_ac_on();
        } else {
            self.turn_ac_off();
        }

        if self.fan == ON {
            self.turn_fan_on();
        } else {
            self.turn_fan_off();
        }
    }
}
