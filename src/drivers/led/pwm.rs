use crate::{peripherals::pwm::PwmPin, utils::math::Percentage};

/// A [`light-emitting diode`] connected to the microcontroller through the `P` PWM pin, which allows to control it.
/// The difference between this struct and [`Led`], is that this one has PWM support, which allows you to control the
/// light intensity (from 0% to 100%) of the LED.
///
/// [`Led`]: super::Led
/// [`light-emitting diode`]: <https://en.wikipedia.org/wiki/Light-emitting_diode>
pub struct LedPwm<P: PwmPin>
{
	pin: P,
}

impl<P: PwmPin> LedPwm<P>
{
	/// Returns a [`Led`] that can control its on/off state through the provided `pin`.
	pub fn new(pin: P) -> Self
	{
		Self { pin }
	}

	/// Turns on the led at `100%` (setting the pin high).
	///
	/// Returns an error if there was an error controlling the output state of the pin.
	pub fn turn_on(&mut self) -> Result<(), <P as PwmPin>::Error>
	{
		self.pin.set_duty_cycle(Percentage::FULL)
	}
	/// Turns off the led (setting the pin low).
	///
	/// Returns an error if there was an error controlling the output state of the pin.
	pub fn turn_off(&mut self) -> Result<(), <P as PwmPin>::Error>
	{
		self.pin.set_duty_cycle(Percentage::ZERO)
	}

	/// Differently from [`Self::turn_on`] and [`Self::turn_off`], this method can control the "percentage of ON state"
	/// of the LED (varying the duty cycle of the PWM signal), varying the amount of light being emitted.
	///
	/// Returns an error if there was an error controlling the output state of the pin.
	///
	/// # Examples
	/// ```
	/// # /*
	/// let mut pwm_led = PwmLed::new(...);
	///
	/// // These 2 lines are equivalent
	/// pwm_led.turn_off();
	/// pwm_led.set_intensity(Percentage::ZERO);
	///
	/// // These 2 lines are equivalent
	/// pwm_led.turn_on();
	/// pwm_led.set_intensity(Percentage::FULL);
	///
	/// // But you can also set an intermediate percentage with this method
	/// pwm_led.set_intensity(Percentage::from_0_to_100(25.).unwrap());
	/// # */
	/// ```
	pub fn set_intensity(&mut self, intensity: Percentage) -> Result<(), <P as PwmPin>::Error>
	{
		self.pin.set_duty_cycle(intensity)
	}

	/// Returns the light intensity value you previously set calling [`Self::set_intensity`].
	///
	/// [`Self::turn_on`] sets a `Percentage::FULL` intensity, while [`Self::turn_off`] sets a `Percentage::ZERO` intensity.
	pub fn get_intensity(&self) -> Percentage
	{
		self.pin.get_duty_cycle()
	}
}
