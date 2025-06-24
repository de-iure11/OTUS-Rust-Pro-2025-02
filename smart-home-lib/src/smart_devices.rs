pub mod socket;
pub mod thermo;

pub use crate::error::Result;
use crate::smart_house::Report;
pub use socket::Socket;
// pub use thermo::Thermo;

pub trait ControllableDevice {
    fn get_name(&self) -> &str;
    fn get_value(&self) -> Option<String>;
    fn get_status(&self) -> Option<String>;
    fn turn_on(&self) -> Result<String>;
    fn turn_off(&self) -> Result<String>;
}

#[derive(Debug)]
pub enum SmartDevices {
    Socket(Socket),
    // Thermo(Thermo),
}

// Преобразование Socket -> SmartDevice
impl From<Socket> for SmartDevices {
    fn from(s: Socket) -> Self {
        SmartDevices::Socket(s)
    }
}

// Преобразование Thermometer -> SmartDevice
// impl From<Thermo> for SmartDevices {
//     fn from(t: Thermo) -> Self {
//         SmartDevices::Thermo(t)
//     }
// }

impl SmartDevices {
    pub fn get_name(&self) -> &str {
        match self {
            SmartDevices::Socket(socket) => socket.get_name(),
            // SmartDevices::Thermo(thermo) => thermo.get_name(),
        }
    }
}

impl Report for SmartDevices {
    fn get_report(&self) -> String {
        match self {
            SmartDevices::Socket(socket) => socket.get_report(),
            // SmartDevices::Thermo(thermo) => thermo.get_report(),
        }
    }
}
