use core::convert::Infallible;
use std::time::SystemTime;

use embedded_svc::wifi::Wifi;
use esp_idf_svc::sntp::*;
use time::{OffsetDateTime, UtcOffset};

use crate::peripherals::time::real_time::RealTimeClock;

pub struct RealTime<WifiDriver: Wifi>(UtcOffset, Option<WifiDriver>, Option<EspSntp<'static>>);

impl<WifiDriver: Wifi> RealTime<WifiDriver>
{
	pub fn new(utc_offset: UtcOffset, wifi_driver: Option<WifiDriver>, sntp: Option<EspSntp<'static>>) -> Self
	{
		Self(utc_offset, wifi_driver, sntp)
	}
}

impl<WifiDriver: Wifi> RealTimeClock for RealTime<WifiDriver>
{
	type Error = Infallible;

	fn now(&self) -> Result<(time::Date, time::Time), Self::Error>
	{
		let now: OffsetDateTime = SystemTime::now().into();
		let now = now.to_offset(self.0);
		Ok((now.date(), now.time()))
	}
}
