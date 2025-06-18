use crate::error::{Error, Result};
use crate::smart_devices::SmartDevices;
use crate::smart_house::Report;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub devices: HashMap<String, SmartDevices>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Room {
            name: name.to_string(),
            devices: HashMap::new(),
        }
    }

    /// Получение имени комнаты
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Добавление устройства
    pub fn add_device(&mut self, key: Option<&str>, device: SmartDevices) -> Result<()> {
        let target_key = key.map_or(device.get_name().to_owned(), |k| k.to_owned());
        if self.devices.contains_key(&target_key) {
            return Err(Error::DeviceAlreadyExists(target_key));
        };
        self.devices.insert(target_key, device);
        Ok(())
    }

    /// Удаление устройства
    pub fn remove_device(&mut self, name: &str) -> Result<()> {
        if !self.devices.contains_key(name) {
            return Err(Error::DeviceDoesNotExist(name.to_owned()));
        };
        self.devices.remove(name);
        Ok(())
    }

    /// Получение ссылки на устройство
    pub fn get_device(&self, name: &str) -> Option<&SmartDevices> {
        self.devices.get(name)
    }

    /// Получение мутабельной ссылки на устройство
    pub fn get_mut_device(&mut self, name: &str) -> Option<&mut SmartDevices> {
        self.devices.get_mut(name)
    }
}

impl Report for Room {
    fn get_report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("Room (name: {})\n", self.name));
        for device in self.devices.values() {
            report.push_str(&format!("  {}\n", device.get_report()));
        }
        report
    }
}

// Макрос для упрощенного создания комнаты
#[macro_export]
macro_rules! create_room {
    ($name:expr, $(($key:expr, $device:expr)),*) => {
        {
            let mut room = Room::new($name);
            $(
                if let Err(e) = room.add_device(Some($key), $device.into()) {
                    eprintln!("Error adding device with key '{}': {:?}", $key, e);
                }
            )*
            room
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use crate::smart_devices::{Socket, Thermo};

    #[test]
    fn add_device_ok() {
        let name_fx = "NewSocket";

        assert_eq!(
            Ok(()),
            Room::new("LivingRoom").add_device(None, Socket::new(name_fx).into())
        )
    }

    #[test]
    fn add_device_err() {
        let mut room = Room::new("LivingRoom");
        let name_fx = "NewSocket";
        let _ = room.add_device(None, Socket::new(name_fx).into());

        assert_eq!(
            room.add_device(None, Socket::new(name_fx).into()),
            Err(Error::DeviceAlreadyExists(name_fx.to_owned()))
        );
    }

    #[test]
    fn remove_device_ok() {
        let mut room = Room::new("LivingRoom");
        let name_fx = "NewSocket";
        let _ = room.add_device(None, Socket::new(name_fx).into());

        assert_eq!(Ok(()), room.remove_device(name_fx));
    }

    #[test]
    fn remove_device_err() {
        let name_fx = "NewSocket";

        assert_eq!(
            Room::new("LivingRoom").remove_device(name_fx),
            Err(Error::DeviceDoesNotExist(name_fx.to_owned()))
        );
    }

    #[test]
    fn test_create_room() {
        let socket = Socket::new("socket1");
        let thermometer = Thermo::new("therm1");

        let room = create_room!("LivingRoom", ("socket1", socket), ("therm1", thermometer));

        assert_eq!(room.name, "LivingRoom");
        assert_eq!(room.devices.len(), 2);

        if let Some(socket) = room.get_device("socket1") {
            println!("Socket found: {:?}", socket);
        } else {
            panic!("Socket not found!");
        }

        if let Some(thermometer) = room.get_device("therm1") {
            println!("Thermometer found: {:?}", thermometer);
        } else {
            panic!("Thermometer not found!");
        }
    }
}
