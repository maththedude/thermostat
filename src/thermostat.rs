use esp_hal::{gpio::Output, time::Instant};

use crate::{OFF, ON, helpers::set_relay_state};

pub struct Thermostat<'a> {
    pub heat: bool,
    pub cool: bool,
    pub fan: bool,
    pub fan_mode: FanMode,
    pub temp: i16,
    pub set_point_low: i16,
    pub set_point_high: i16,
    pub mode: Mode,
    pub backlight: bool,
    pub backlight_since: Instant,
    pub heat_pin: Output<'a>,
    pub cool_pin: Output<'a>,
    pub fan_pin: Output<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Off,
    Heat,
    Cool,
    Hold,
    Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FanMode {
    Off,
    On,
    Auto,
}

impl<'a> Thermostat<'a> {
    pub fn turn_heat_on(&mut self) {
        set_relay_state(&mut self.heat_pin, ON);
    }
}

impl<'a> Thermostat<'a> {
    pub fn turn_heat_off(&mut self) {
        set_relay_state(&mut self.heat_pin, OFF);
    }
}
