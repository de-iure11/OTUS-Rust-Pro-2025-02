use super::error::{Error, Result};
use rand::Rng;

/// Шкалы температур для котрых можно получить данные от умного термометра.
#[derive(Clone)]
pub enum TemperatureScale {
    Kelvin,
    Celsius,
    Fahrenheit,
}

impl core::fmt::Display for TemperatureScale {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        let scale = match &self {
            TemperatureScale::Celsius => "°C",
            TemperatureScale::Fahrenheit => "°F",
            TemperatureScale::Kelvin => "°K",
        };
        write!(fmt, "{}", scale)
    }
}

/// Структура с  типа умного теммометра
pub struct Thermometer {
    name: String,
    scale: TemperatureScale,
    temperature: f32,
    enabled: bool,
}

impl Thermometer {
    /// Кнструктор умного термометра (принимает опциональный параметр шкалы измерений - TemperatureScale, по умолчанию шкала измерений в Цельсиях )
    pub fn new(name: &str, scale: Option<TemperatureScale>) -> Self {
        let scale = scale.unwrap_or(TemperatureScale::Celsius);

        let mut thermometer = Thermometer {
            name: name.to_string(),
            scale,
            enabled: true,
            temperature: 0.0,
        };
        let _ = thermometer.run_measurement();
        thermometer
    }

    /// Получение значения текущей температуры (принимает опциональный параметр TemperatureScale)
    pub fn get_temperature(&self, to_scale: Option<TemperatureScale>) -> Result<(f32, String)> {
        if !self.enabled {
            return Err(Error::FailConnectToDevice(
                "Ошибка подключения к устройству, возможно устройство выключено.".to_string(),
            ));
        }
        let to_scale = to_scale.unwrap_or(TemperatureScale::Celsius);
        let (temp, scale) = self._transform(to_scale);
        Ok((temp, scale.to_string()))
    }

    /// Измерение значения температуры (генерируется новое значение температуры)
    pub fn run_measurement(&mut self) -> Result<()> {
        let mut rng = rand::thread_rng();

        if !self.enabled {
            return Err(Error::FailConnectToDevice(
                "Ошибка подключения к устройству, возможно устройство выключено.".to_string(),
            ));
        }

        let temperature: f32 = match self.scale {
            TemperatureScale::Celsius => rng.gen_range(0.0..37.0),
            TemperatureScale::Fahrenheit => rng.gen_range(32.0..98.6),
            TemperatureScale::Kelvin => rng.gen_range(273.15..310.15),
        };

        self.temperature = temperature;

        Ok(())
    }

    /// Преобразование значения температуры для разных температурных шкал.
    fn _transform(&self, to_scale: TemperatureScale) -> (f32, String) {
        // 1️⃣ Преобразуем текущую температуру в градусы Цельсия
        let celsius = match self.scale {
            TemperatureScale::Celsius => self.temperature,
            TemperatureScale::Fahrenheit => (self.temperature - 32.0) * 5.0 / 9.0,
            TemperatureScale::Kelvin => self.temperature - 273.15,
        };

        // 2️⃣ Преобразуем из Цельсия в нужную шкалу
        let temperature = match to_scale {
            TemperatureScale::Celsius => celsius,
            TemperatureScale::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
            TemperatureScale::Kelvin => celsius + 273.15,
        };

        (temperature, to_scale.to_string())
    }
}

/// Реализация типажа Display для Thermometer для вывода вывода отчета.
impl core::fmt::Display for Thermometer {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        let (temp_c, scale_c) = self._transform(TemperatureScale::Celsius);
        let (temp_f, scale_f) = self._transform(TemperatureScale::Fahrenheit);
        let (temp_k, scale_k) = self._transform(TemperatureScale::Kelvin);
        write!(
            fmt,
            "Умный термометр ({}, текущая температура: {:.2}{}, {:.2}{}, {:.2}{})",
            self.name, temp_c, scale_c, temp_f, scale_f, temp_k, scale_k
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_default_thermometer_ok() -> Result<()> {
        let name_fx = "Default Sensor";
        let scale_fx = TemperatureScale::Celsius;

        let thermo = Thermometer::new(name_fx, None);

        assert_eq!(thermo.name, name_fx.to_string());
        assert_eq!(thermo.scale.to_string(), scale_fx.to_string()); // Должен быть Celsius по умолчанию
        assert!(thermo.enabled); // Должен быть включен

        Ok(())
    }

    #[test]
    fn test_new_specific_thermometer_ok() -> Result<()> {
        let name_fx = "Fahrenheit Sensor";
        let scale_fx = TemperatureScale::Fahrenheit;

        let thermo = Thermometer::new(name_fx, Some(scale_fx.clone()));

        assert_eq!(thermo.name, name_fx.to_string());
        assert_eq!(thermo.scale.to_string(), scale_fx.to_string());
        assert!(thermo.enabled);

        Ok(())
    }

    #[test]
    fn test_get_temperature_ok() -> Result<()> {
        let thermo = Thermometer {
            name: "Test Thermometer".to_string(),
            scale: TemperatureScale::Celsius,
            enabled: true,
            temperature: 25.0, // Фиксируем температуру для предсказуемости (25.00°C = 77.00°F = 298.15°K)
        };

        if let Ok((t, scale)) = thermo.get_temperature(None) {
            assert_eq!(t, 25.00);
            assert_eq!(scale, "°C");
        } else {
            panic!("Ошибка при получении температуры в °C");
        };

        if let Ok((t, scale)) = thermo.get_temperature(Some(TemperatureScale::Fahrenheit)) {
            assert_eq!(t, 77.00);
            assert_eq!(scale, "°F");
        } else {
            panic!("Ошибка при получении температуры в °F");
        };

        if let Ok((t, scale)) = thermo.get_temperature(Some(TemperatureScale::Kelvin)) {
            assert_eq!(t, 298.15);
            assert_eq!(scale, "°K");
        } else {
            panic!("Ошибка при получении температуры °K");
        };

        Ok(())
    }

    #[test]
    fn test_run_measurement() -> Result<()> {
        let mut thermo = Thermometer {
            name: "Thermometer".to_string(),
            scale: TemperatureScale::Celsius,
            enabled: true,
            temperature: 0.0, // Начальная температура
        };

        // Запускаем измерение
        thermo.run_measurement()?;

        let (t, _) = thermo.get_temperature(None).unwrap();
        assert!(
            t >= 0.0 && t <= 37.0,
            "Температура (°C) должна быть в диапазоне 0.0..37.0, но была {}",
            thermo.temperature
        );

        let (t, _) = thermo
            .get_temperature(Some(TemperatureScale::Fahrenheit))
            .unwrap();

        assert!(
            t >= 32.0 && t <= 98.6,
            "Температура (°F) должна быть в диапазоне 32.0..98.6, но была {}",
            thermo.temperature
        );

        let (t, _) = thermo
            .get_temperature(Some(TemperatureScale::Kelvin))
            .unwrap();

        assert!(
            t >= 273.15 && t <= 310.15,
            "Температура (°K) должна быть в диапазоне 273.15..310.15, но была {}",
            thermo.temperature
        );

        Ok(())
    }

    #[test]
    fn test_thermometer_display_ok() {
        let thermo = Thermometer {
            name: "Test Thermometer".to_string(),
            scale: TemperatureScale::Celsius,
            enabled: true,
            temperature: 25.0, // Фиксируем температуру для предсказуемости
        };

        let output = format!("{}", thermo);

        let expected_output =
            "Умный термометр (Test Thermometer, текущая температура: 25.00°C, 77.00°F, 298.15°K)";

        assert_eq!(output, expected_output);
    }
}
