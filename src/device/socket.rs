use super::SmartDeviceControl;
use super::error::Result;
use rand::Rng;

pub struct Socket {
    name: String,
    amperes: f32,
    volts: f32,
    enabled: bool,
}

impl Socket {
    pub fn new(name: &str) -> Self {
        let mut rng = rand::thread_rng();
        Socket {
            name: name.to_string(),
            amperes: rng.gen_range(4.0..6.0),
            volts: rng.gen_range(210.0..230.0),
            enabled: true,
        }
    }
}

impl SmartDeviceControl for Socket {
    fn get_value(&self) -> f32 {
        self.amperes * self.volts
    }

    fn turn_on(&mut self) -> Result<()> {
        let mut rng = rand::thread_rng();
        self.enabled = true;
        self.amperes = rng.gen_range(4.0..6.0);
        self.volts = rng.gen_range(210.0..230.0);
        Ok(())
    }

    fn turn_off(&mut self) -> Result<()> {
        self.enabled = false;
        self.amperes = 0.0;
        self.volts = 0.0;
        Ok(())
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Реализация типажа Display для Thermometer для вывода вывода отчета.
impl core::fmt::Display for Socket {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(
            fmt,
            "Pозетка (name: {}, value: {:.2}A, {:.2}V, {:.2}W)",
            self.name,
            self.amperes,
            self.volts,
            self.get_value()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_socket_ok() {
        let name_fx = "NewSocket";

        let socket = Socket::new(name_fx);

        assert_eq!(socket.name, name_fx.to_string());
        assert!(socket.enabled);
        assert!(
            socket.amperes >= 4.0 && socket.amperes <= 6.0,
            "Значение Ампер должно быть в диапазоне 4.0..6.0, но равно {}",
            socket.amperes
        );
        assert!(
            socket.volts >= 210.0 && socket.volts <= 230.0,
            "Значение Вольт должно быть в диапазоне 210.0..230.0, но равно {}",
            socket.amperes
        );
    }

    #[test]
    fn socket_get_value_ok() -> Result<()> {
        let mut socket = Socket {
            name: "Test Socket".to_string(),
            enabled: false,
            amperes: 5.0, // Фиксированное значение
            volts: 220.0, // Фиксированное значение
        };

        let power = socket.get_value();
        assert_eq!(power, 1100.00);
        socket.turn_off();
        let power = socket.get_value();
        assert_eq!(power, 0.00);
        Ok(())
    }

    #[test]
    fn socket_display_ok() -> Result<()> {
        let socket = Socket {
            name: "NewSocket".to_string(),
            enabled: true,
            amperes: 5.0,
            volts: 220.0,
        };

        let output = format!("{}", socket);
        let expected_output = "Pозетка (name: NewSocket, value: 5.00A, 220.00V, 1100.00W)";
        assert_eq!(output, expected_output);
        Ok(())
    }
}
