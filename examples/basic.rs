use smart_house::smart_devices::{Socket, Thermo};
use smart_house::smart_house::SmartHouse;
use smart_house::utils::print_report;
use smart_house::{create_room, room::Room};

fn main() {
    let mut my_smart_house = SmartHouse::new("Дом в Ромашково");

    let living_room = create_room!(
        "LivingRoom",
        ("socket1", Socket::new("socket1")),
        ("therm1", Thermo::new("therm1"))
    );

    let childrens_room = create_room!(
        "ChildrensRoom",
        ("socket2", Socket::new("socket2")),
        ("therm2", Thermo::new("therm2"))
    );

    let _ = my_smart_house.add_room(living_room);
    let _ = my_smart_house.add_room(childrens_room);

    println!("Общий отчет:");
    print_report(&my_smart_house);

    let some_device1 = my_smart_house.get_smart_device("LivingRoom", "socket1");

    print!("Отчет об устройстве: ");
    match some_device1 {
        Ok(device) => print_report(device),
        Err(err) => println!("{}", err),
    }

    let some_device2 = my_smart_house.get_smart_device("LivingRoom", "socket3");
    print!("Ошибка получения устройства: ");
    match some_device2 {
        Ok(device) => print_report(device),
        Err(err) => println!("{}", err),
    }
}
