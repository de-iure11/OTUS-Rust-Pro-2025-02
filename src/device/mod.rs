pub mod error;
pub mod socket;
pub mod thermo;

pub use socket::Socket;
pub use thermo::Thermo;

use error::Result;

pub trait SmartDeviceControl {
    fn get_value(&self) -> f32;
    fn turn_on(&mut self) -> Result<()>;
    fn turn_off(&mut self) -> Result<()>;
    fn is_enabled(&self) -> bool;
}

pub enum SmartDevice {
    Socket(Socket),
    Thermo(Thermo),
}

// Преобразование Socket -> SmartDevice
impl From<Socket> for SmartDevice {
    fn from(s: Socket) -> Self {
        SmartDevice::Socket(s)
    }
}

// Преобразование Thermometer -> SmartDevice
impl From<Thermo> for SmartDevice {
    fn from(t: Thermo) -> Self {
        SmartDevice::Thermo(t)
    }
}

impl core::fmt::Display for SmartDevice {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            SmartDevice::Socket(s) => write!(fmt, "{}", s),
            SmartDevice::Thermo(t) => write!(fmt, "{}", t),
        }
    }
}
