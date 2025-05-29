pub mod socket;
pub mod thermo;

pub use crate::error::Result;
use crate::smart_house::Report;
pub use socket::Socket;
pub use thermo::Thermo;

pub trait ControllableDevice {
    fn get_name(&self) -> &String;
    fn get_value(&self) -> f32;
    fn turn_on(&mut self) -> Result<()>;
    fn turn_off(&mut self) -> Result<()>;
    fn is_on(&self) -> bool;
}

#[derive(Debug)]
pub enum SmartDevices {
    Socket(Socket),
    Thermo(Thermo),
}

// Преобразование Socket -> SmartDevice
impl From<Socket> for SmartDevices {
    fn from(s: Socket) -> Self {
        SmartDevices::Socket(s)
    }
}

// Преобразование Thermometer -> SmartDevice
impl From<Thermo> for SmartDevices {
    fn from(t: Thermo) -> Self {
        SmartDevices::Thermo(t)
    }
}

impl SmartDevices {
    pub fn get_name(&self) -> &String {
        match self {
            SmartDevices::Socket(socket) => socket.get_name(),
            SmartDevices::Thermo(thermo) => thermo.get_name(),
        }
    }
}

impl Report for SmartDevices {
    fn get_report(&self) -> String {
        match self {
            SmartDevices::Socket(socket) => socket.get_report(),
            SmartDevices::Thermo(thermo) => thermo.get_report(),
        }
    }
}
