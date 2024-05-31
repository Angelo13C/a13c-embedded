use embedded_hal::delay::DelayNs;
use esp_idf_hal::delay::Delay as EspDelay;

pub struct Delay;

impl DelayNs for Delay
{
	fn delay_ns(&mut self, ns: u32)
	{
		// Delay at least 1us even if the duration is only 10ns
		EspDelay::new_default().delay_us((ns / 1_000).max(1))
	}
}
