use rand::Rng;

pub struct Socket {
    name: String,
    enabled: bool,
    amperes: f32,
    volts: f32,
}

impl Socket {
    pub fn new(name: &str) -> Self {
        let mut rng = rand::thread_rng();
        Socket {
            name: name.to_string(),
            enabled: true,
            amperes: rng.gen_range(4.0..6.0),
            volts: rng.gen_range(210.0..230.0),
        }
    }

    pub fn turn_on(&mut self) {
        self.enabled = true;
    }

    pub fn turn_off(&mut self) {
        self.enabled = false;
        self.amperes = 0.0;
        self.volts = 0.0;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_power(&self) -> f32 {
        self.amperes * self.volts
    }
}

/// Реализация типажа Display для Thermometer для вывода вывода отчета.
impl core::fmt::Display for Socket {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        let state = if self.enabled {
            "работает"
        } else {
            "не работет"
        };
        write!(
            fmt,
            "Умная розетка ({}, {}, {:.2}A, {:.2}V, {:.2}W)",
            self.name,
            state,
            self.amperes,
            self.volts,
            self.get_power()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_socket_ok() {
        let name_fx = "Default Socket";

        let socket = Socket::new(name_fx);

        assert_eq!(socket.name, name_fx.to_string());
        assert!(socket.enabled);
        assert!(
            socket.amperes >= 4.0 && socket.amperes <= 6.0,
            "Значение Ампер должно быть в диапазоне 4.0..6.0, но была {}",
            socket.amperes
        );
        assert!(
            socket.volts >= 210.0 && socket.volts <= 230.0,
            "Значение Вольт должно быть в диапазоне 4.0..6.0, но была {}",
            socket.amperes
        );
    }

    #[test]
    fn test_socket_enabled_get_power_ok() {
        // Создаём сокет с фиксированными параметрами
        let socket = Socket {
            name: "Test Socket".to_string(),
            enabled: true,
            amperes: 5.0, // Фиксированное значение ампер
            volts: 220.0, // Фиксированное значение вольт
        };

        // Вычисляем мощность
        let power = socket.get_power();

        // Проверяем, что мощность равна 5.0 * 220.0 = 1100.0
        assert_eq!(power, 1100.0);
    }

    #[test]
    fn test_socket_disabled_get_power_ok() {
        // Создаём сокет с фиксированными параметрами и выключенным состоянием
        let socket = Socket {
            name: "Test Socket".to_string(),
            enabled: false,
            amperes: 0.0,
            volts: 0.0,
        };

        // Проверяем, что мощность будет 0.0, так как сокет выключен
        let power = socket.get_power();

        assert_eq!(power, 0.0);
    }

    #[test]
    fn test_socket_turn_off_ok() {
        // Создаём сокет с фиксированными параметрами и выключенным состоянием
        let mut socket = Socket {
            name: "Test Socket".to_string(),
            enabled: true,
            amperes: 5.0,
            volts: 220.0,
        };

        let _ = socket.turn_off();

        assert!(!socket.enabled);
    }

    #[test]
    fn test_socket_turn_on_ok() {
        // Создаём сокет с фиксированными параметрами и выключенным состоянием
        let mut socket = Socket {
            name: "Test Socket".to_string(),
            enabled: false,
            amperes: 5.0,
            volts: 220.0,
        };

        let _ = socket.turn_on();

        assert!(socket.enabled);
    }

    #[test]
    fn test_socket_display_ok() {
        let socket = Socket {
            name: "Test Socket".to_string(),
            enabled: true,
            amperes: 5.0,
            volts: 220.0,
        };

        let output = format!("{}", socket);

        let expected_output = "Умная розетка (Test Socket, работает, 5.00A, 220.00V, 1100.00W)";

        assert_eq!(output, expected_output);
    }
}
