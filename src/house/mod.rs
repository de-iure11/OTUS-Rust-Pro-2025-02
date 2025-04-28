mod room;

pub use room::Room;

pub struct SmartHouse {
    pub name: String,
    pub rooms: Vec<Room>,
}

impl SmartHouse {
    /// Конструктор - принимает название дома и список комнат
    ///
    /// # Пример
    ///
    /// ```
    /// let devices = vec![crate::smart_house::device::Socket::new("розетка №1").into()];
    /// let rooms = vec![crate::smart_house::house::Room::new("Гостинная", devices)];
    /// let house = crate::smart_house::house::SmartHouse::new("Новый дом", rooms);
    /// assert_eq!(house.name, "Новый дом");
    /// assert_eq!(house.rooms.len(), 1);
    /// ```
    pub fn new(name: &str, rooms: Vec<Room>) -> Self {
        SmartHouse {
            name: name.to_string(),
            rooms,
        }
    }

    /// Получение ссылки на комнату по индексу
    pub fn get_room(&self, index: usize) -> Option<&Room> {
        self.rooms.get(index)
    }

    /// Получение мутабельной ссылки на комнату по индексу
    pub fn get_mut_room(&mut self, index: usize) -> Option<&mut Room> {
        self.rooms.get_mut(index)
    }

    /// Вывод в стандартный вывод отчёт о всех комнатах.
    pub fn print_rooms(&self) {
        println!("\nОтчёт комнатах в доме ({}):\n", self.name);

        for (idx, room) in self.rooms.iter().enumerate() {
            println!(
                "{:>5}. {} (устройств - {})",
                idx,
                room.name,
                room.devices.len()
            );
            for (i, device) in room.devices.iter().enumerate() {
                println!("{:>8}. {}", i, device);
            }
            println!();
        }
    }
}
