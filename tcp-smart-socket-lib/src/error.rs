use std::io;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Внутренняя ошибка IO.
    Io(io::Error),
    /// Некорректная кодировка строки.
    BadEncoding,
    /// Ошибка подключения.
    ConnectError,
}

impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
