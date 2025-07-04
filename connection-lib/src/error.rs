// use std::io;
// use thiserror::Error;

// pub type Result<T> = core::result::Result<T, Error>;

// #[derive(Debug)]
// pub enum Error {
//     /// Внутренняя ошибка IO.
//     Io(io::Error),
//     /// Некорректная кодировка строки.
//     BadEncoding,
//     /// Ошибка подключения.
//     ConnectError,
//     /// Несоответствие формата конфига.
//     ConfigFormatError(String),
// }

// impl std::error::Error for Error {}

// impl core::fmt::Display for Error {
//     fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
//         write!(fmt, "{self:?}")
//     }
// }

// impl From<std::io::Error> for Error {
//     fn from(err: std::io::Error) -> Self {
//         Error::Io(err)
//     }
// }
//

use std::io;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Внутренняя ошибка IO: {0}")]
    Io(#[from] io::Error),

    #[error("Некорректная кодировка строки")]
    BadEncoding,

    #[error("Ошибка подключения")]
    ConnectError,

    #[error("Ошибка отправки данных")]
    FailedToSendData,

    #[error("Несоответствие формата конфига: {0}")]
    ConfigFormatError(String),

    #[error("Ошибка разрешения адреса")]
    AddrResolutionFailed,
}
