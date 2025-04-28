//! Пример использования библиотеки для реализации умного дома

use smart_house::device::{SmartDevice, Socket, TemperatureScale, Thermometer};
use smart_house::house::{Room, SmartHouse};

fn main() {
    let mut my_smart_house = SmartHouse::new(
        "Дом в Ромашково",
        vec![
            Room::new(
                "Гостинная",
                vec![
                    Socket::new("розетка №1").into(),
                    Thermometer::new("термометр №1", None).into(),
                ],
            ),
            Room::new(
                "Детская",
                vec![
                    Socket::new("розетка №2").into(),
                    Thermometer::new("термометр №2", Some(TemperatureScale::Fahrenheit)).into(),
                ],
            ),
        ],
    );

    my_smart_house.print_rooms();

    if let Some(SmartDevice::Socket(socket)) = my_smart_house
        .get_mut_room(1)
        .and_then(|room| room.get_mut_device(0))
    {
        socket.turn_off();
        println!("{} - выключена\n", socket);
    }

    println!("Обновленный отчет.");
    my_smart_house.print_rooms();
}
