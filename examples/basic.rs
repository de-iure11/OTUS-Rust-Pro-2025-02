//! Пример использования библиотеки для реализации умного дома

use smart_house::device::{SmartDevice, SmartDeviceControl, Socket, Thermo};
use smart_house::house::{Room, SmartHouse};

fn main() {
    let mut my_smart_house = SmartHouse::new(
        "Дом в Ромашково",
        vec![
            Room::new(
                "Гостинная",
                vec![
                    Socket::new("Розетка-1").into(),
                    Thermo::new("Термометр-1").into(),
                ],
            ),
            Room::new(
                "Детская",
                vec![
                    Socket::new("Розетка-2").into(),
                    Thermo::new("Термометр-2").into(),
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
