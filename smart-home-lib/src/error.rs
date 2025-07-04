pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DeviceAlreadyExists(String),
    DeviceDoesNotExist(String),
    RoomAlreadyExists(String),
    RoomDoesNotExist(String),
    FailToConnectDevice(String),
    FailToSendRequest(String),
    UnknownCommand(String),
    Other(String),
}

impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<connection_lib::error::Error> for crate::error::Error {
    fn from(e: connection_lib::error::Error) -> Self {
        crate::error::Error::Other(e.to_string()) // или оберни внутрь своего enum
    }
}
