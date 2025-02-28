use embedded_hal_async::spi::{self};

#[derive(Debug)]
pub enum As8510Error {
    Spi(spi::ErrorKind),
    SpiOther,
    IllegalAddress(u8),
    NotReady,
}

impl core::fmt::Display for As8510Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            As8510Error::Spi(kind) => write!(f, "SPI error {:?} ", kind),
            As8510Error::SpiOther => write!(f, "Invalid SPIOther data "),
            As8510Error::NotReady => write!(f, "Reading not yet available"),
            As8510Error::IllegalAddress(l) => write!(f, "Length error {:?}", l),
        }
    }
}

impl<E> From<E> for As8510Error
where
    E: embedded_hal::spi::Error + core::fmt::Debug,
{
    fn from(e: E) -> Self {
        Self::Spi(e.kind())
    }
}

enum GpioError {
    Error(embedded_hal::digital::ErrorKind),
}
impl core::fmt::Display for GpioError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GpioError::Error(kind) => write!(f, "SPI error {:?} ", kind),
        }
    }
}

impl<E> From<E> for GpioError
where
    E: embedded_hal::digital::Error + core::fmt::Debug,
{
    fn from(e: E) -> Self {
        Self::Error(e.kind())
    }
}
