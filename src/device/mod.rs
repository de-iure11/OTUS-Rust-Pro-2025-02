pub mod error;
pub mod socket;
pub mod thermometer;

pub use socket::Socket;
pub use thermometer::TemperatureScale;
pub use thermometer::Thermometer;

pub enum SmartDevice {
    Socket(Socket),
    Thermometer(Thermometer),
}

// Преобразование Socket -> SmartDevice
impl From<Socket> for SmartDevice {
    fn from(socket: Socket) -> Self {
        SmartDevice::Socket(socket)
    }
}

// Преобразование Thermometer -> SmartDevice
impl From<Thermometer> for SmartDevice {
    fn from(thermometer: Thermometer) -> Self {
        SmartDevice::Thermometer(thermometer)
    }
}

/// Реализация типажа Display для SmartDevice.
impl core::fmt::Display for SmartDevice {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            SmartDevice::Socket(socket) => write!(fmt, "{}", socket),
            SmartDevice::Thermometer(thermometer) => write!(fmt, "{}", thermometer),
        }
    }
}
