use crate::error::{Error, Result};
use crate::room::Room;
use crate::smart_devices::SmartDevices;
use std::collections::HashMap;

pub trait Report {
    fn get_report(&self) -> String;
}

#[derive(Debug)]
pub struct SmartHouse {
    pub name: String,
    pub rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        SmartHouse {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }

    /// Добавление устройства
    pub fn add_room(&mut self, room: Room) -> Result<()> {
        if self.rooms.contains_key(room.get_name()) {
            return Err(Error::RoomAlreadyExists(room.get_name().to_owned()));
        };
        self.rooms.insert(room.get_name().to_owned(), room);
        Ok(())
    }

    /// Удаление устройства
    pub fn remove_room(&mut self, name: &str) -> Result<()> {
        if !self.rooms.contains_key(name) {
            return Err(Error::RoomDoesNotExist(name.to_owned()));
        };
        self.rooms.remove(name);
        Ok(())
    }

    /// Получение ссылки на комнату по индексу
    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    /// Получение мутабельной ссылки на комнату по индексу
    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    /// Получение  ссылки на умное устройство
    pub fn get_smart_device(
        &mut self,
        name_room: &str,
        name_device: &str,
    ) -> Result<&SmartDevices> {
        let room = match self.get_room(name_room) {
            Some(room) => room,
            None => return Err(Error::RoomDoesNotExist(name_room.to_owned())),
        };

        let device = match room.get_device(name_device) {
            Some(device) => device,
            None => return Err(Error::DeviceDoesNotExist(name_device.to_owned())),
        };

        Ok(device)
    }

    /// Получение  ссылки на умное устройство
    pub fn get_mut_smart_device(
        &mut self,
        name_room: &str,
        name_device: &str,
    ) -> Result<&mut SmartDevices> {
        let room = match self.get_room_mut(name_room) {
            Some(room) => room,
            None => return Err(Error::RoomDoesNotExist(name_room.to_owned())),
        };

        let device = match room.get_mut_device(name_device) {
            Some(device) => device,
            None => return Err(Error::DeviceDoesNotExist(name_device.to_owned())),
        };

        Ok(device)
    }
}

impl Report for SmartHouse {
    fn get_report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("SmartHouse (name: {})\n", self.name));
        for room in self.rooms.values() {
            report.push_str(&format!("  {}\n", room.get_report()));
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use crate::room::Room;

    #[test]
    fn add_room_ok() {
        let name_fx = "NewRoom";

        assert_eq!(
            Ok(()),
            SmartHouse::new("House").add_room(Room::new(name_fx))
        )
    }

    #[test]
    fn add_room_err() {
        let mut house = SmartHouse::new("House");
        let name_fx = "NewRoom";
        let _ = house.add_room(Room::new(name_fx));

        assert_eq!(
            house.add_room(Room::new(name_fx)),
            Err(Error::RoomAlreadyExists(name_fx.to_owned()))
        );
    }

    #[test]
    fn remove_room_ok() {
        let mut house = SmartHouse::new("House");
        let name_fx = "NewRoom";
        let _ = house.add_room(Room::new(name_fx));

        assert_eq!(Ok(()), house.remove_room(name_fx));
    }

    #[test]
    fn remove_room_err() {
        let name_fx = "NewRoom";

        assert_eq!(
            SmartHouse::new("House").remove_room(name_fx),
            Err(Error::RoomDoesNotExist(name_fx.to_owned()))
        );
    }
}
