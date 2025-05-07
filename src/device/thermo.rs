use super::{SmartDeviceControl, error::Result};
use rand::Rng;

pub struct Thermo {
    name: String,
    value: f32,
    enabled: bool,
}

impl Thermo {
    pub fn new(name: &str) -> Self {
        let mut rng = rand::thread_rng();

        Thermo {
            name: name.to_string(),
            value: rng.gen_range(20.00..25.00),
            enabled: true,
        }
    }
}

impl SmartDeviceControl for Thermo {
    fn get_value(&self) -> f32 {
        self.value
    }

    fn turn_on(&mut self) -> Result<()> {
        let mut rng = rand::thread_rng();
        self.enabled = true;
        self.value = rng.gen_range(20.00..25.00);
        Ok(())
    }

    fn turn_off(&mut self) -> Result<()> {
        self.enabled = false;
        self.value = 0.0;
        Ok(())
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl core::fmt::Display for Thermo {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        let t = self.get_value();
        write!(fmt, "Tермометр (name: {}, value: {:.2}°C)", self.name, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_thermo_ok() -> Result<()> {
        let name_fx = "NewThermo";
        let thermo = Thermo::new(name_fx);
        assert_eq!(thermo.name, name_fx.to_string());
        assert!(thermo.enabled);
        Ok(())
    }

    #[test]
    fn thermo_get_value_ok() -> Result<()> {
        let mut thermo1 = Thermo {
            name: "NewThermometer".to_string(),
            value: 24.6,
            enabled: true,
        };

        assert_eq!(24.60, thermo1.get_value());
        thermo1.turn_off();
        assert_eq!(0.00, thermo1.get_value());
        Ok(())
    }

    #[test]
    fn thermo_display_ok() {
        let thermo = Thermo {
            name: "NewThermometer".to_string(),
            value: 24.6,
            enabled: true,
        };

        let output = format!("{}", thermo);
        let expected_output = "Tермометр (name: NewThermometer, value: 24.60°C)";

        assert_eq!(output, expected_output);
    }
}
