use core::time::Duration;

use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_sys::EspError;

use crate::peripherals::system_time::SystemTime as SystemTimeTrait;

#[derive(Clone)]
pub struct SystemTime(EspTaskTimerService);

impl SystemTime
{
	pub fn new() -> Result<Self, EspError>
	{
		Ok(Self(EspTaskTimerService::new()?))
	}
}

impl SystemTimeTrait for SystemTime
{
	fn now(&self) -> Duration
	{
		self.0.now()
	}
}
