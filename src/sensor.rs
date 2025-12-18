use crate::thermostat;
use embedded_sht3x::Sht3x;

const MAX_RETRIES: u8 = 3;

/// Reads temperature and humidity from the SHT3x sensor with retry logic.
/// Updates the thermostat with the readings and displays them on the LCD.
///
/// # Arguments
/// * `sensor` - Mutable reference to the SHT3x sensor
/// * `thermostat` - Mutable reference to the Thermostat
/// * `lcd` - Mutable reference to the Grove LCD display
///
/// # Returns
/// * `Ok(())` if reading was successful within MAX_RETRIES attempts
/// * `Err(())` if all retry attempts failed
pub fn read_and_update_sensor<I2C, D>(
    sensor: &mut Sht3x<I2C, D>,
    thermostat: &mut thermostat::Thermostat,
) -> Result<(), ()>
where
    I2C: embedded_hal::i2c::I2c,
    D: embedded_hal::delay::DelayNs,
{
    for attempt in 1..=MAX_RETRIES {
        match sensor.single_measurement() {
            Ok(measurement) => {
                // Display temperature and humidity to serial
                esp_println::println!(
                    "  Temperature: {:.2} °C ({:.2} °F)",
                    measurement.temperature,
                    measurement.temperature * 9.0 / 5.0 + 32.0
                );
                esp_println::println!("  Humidity:    {:.2} %\n", measurement.humidity);

                // Convert to Fahrenheit
                let temp_f = measurement.temperature * 9.0 / 5.0 + 32.0;
                let humidity = measurement.humidity;

                // Update thermostat state
                thermostat.temp = temp_f;
                thermostat._humidity = humidity;

                return Ok(());
            }
            Err(e) => {
                esp_println::println!(
                    "Error reading sensor (attempt {}/{}): {:?}",
                    attempt,
                    MAX_RETRIES,
                    e
                );

                // If this wasn't the last attempt, continue to retry
                if attempt < MAX_RETRIES {
                    esp_println::println!("Retrying...\n");
                }
            }
        }
    }

    // All retries exhausted
    esp_println::println!("Failed to read sensor after {} attempts\n", MAX_RETRIES);
    Err(())
}
