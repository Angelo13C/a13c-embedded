#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MockEmptyError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MockSpiError(embedded_hal::spi::ErrorKind);

impl embedded_hal::spi::Error for MockSpiError
{
	fn kind(&self) -> embedded_hal::spi::ErrorKind
	{
		self.0
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MockDigitalError(embedded_hal::digital::ErrorKind);

impl embedded_hal::digital::Error for MockDigitalError
{
	fn kind(&self) -> embedded_hal::digital::ErrorKind
	{
		self.0
	}
}

#[cfg(feature = "embedded-svc")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MockIoError(embedded_svc::io::ErrorKind);

#[cfg(feature = "embedded-svc")]
impl embedded_svc::io::Error for MockIoError
{
	fn kind(&self) -> embedded_svc::io::ErrorKind
	{
		self.0
	}
}
