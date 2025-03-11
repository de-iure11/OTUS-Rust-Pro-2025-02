use crate::device::SmartDevice;

pub struct Room {
    pub name: String,
    pub devices: Vec<SmartDevice>,
}

impl Room {
    /// Создает новый объект Room с указанным именем и устройствами.
    ///
    /// # Пример
    ///
    /// ```
    /// let devices = vec![crate::smart_house::device::Socket::new("розетка №1").into()];
    /// let room = crate::smart_house::house::Room::new("Гостинная", devices);
    /// assert_eq!(room.name, "Гостинная");
    /// assert_eq!(room.devices.len(), 1);
    /// ```
    pub fn new(name: &str, devices: Vec<SmartDevice>) -> Self {
        Room {
            name: name.to_string(),
            devices,
        }
    }

    /// Получение ссылки на устройство по индексу
    pub fn get_device(&self, index: usize) -> Option<&SmartDevice> {
        self.devices.get(index)
    }

    /// Получение мутабельной ссылки на устройство по индексу
    pub fn get_mut_device(&mut self, index: usize) -> Option<&mut SmartDevice> {
        self.devices.get_mut(index)
    }

    /// Вывод в стандартный вывод отчёт о всех устройствах в комнате.
    pub fn print_devices(&self) {
        for (idx, device) in self.devices.iter().enumerate() {
            print!("{}) ", idx);
            print!("{}", device);
        }
    }
}
