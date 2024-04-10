pub use embedded_hal::digital::OutputPin;

use crate::{
	peripherals::pwm::PwmPin,
	utils::{math::Percentage, physical_quantities::rotational_direction::RotationalDirection},
};

pub struct L298NMotor<EN: PwmPin, IN1: OutputPin, IN2: OutputPin>
{
	pin_en: EN,
	pin_in1: IN1,
	pin_in2: IN2,
}

impl<EN: PwmPin, IN1: OutputPin, IN2: OutputPin> L298NMotor<EN, IN1, IN2>
{
	pub fn new(pin_en: EN, pin_in1: IN1, pin_in2: IN2) -> Self
	{
		Self {
			pin_en,
			pin_in1,
			pin_in2,
		}
	}

	pub fn turn_off(&mut self) -> Result<(), Error<IN1, IN2>>
	{
		self.pin_in1.set_low().map_err(Error::IN1)?;
		self.pin_in2.set_low().map_err(Error::IN2)?;

		Ok(())
	}

	pub fn set_direction(&mut self, direction: RotationalDirection) -> Result<(), Error<IN1, IN2>>
	{
		match direction
		{
			RotationalDirection::CW => self.set_forward_direction(),
			RotationalDirection::CCW => self.set_backward_direction(),
		}
	}

	pub fn set_forward_direction(&mut self) -> Result<(), Error<IN1, IN2>>
	{
		self.pin_in1.set_high().map_err(Error::IN1)?;
		self.pin_in2.set_low().map_err(Error::IN2)?;

		Ok(())
	}

	pub fn set_backward_direction(&mut self) -> Result<(), Error<IN1, IN2>>
	{
		self.pin_in1.set_low().map_err(Error::IN1)?;
		self.pin_in2.set_high().map_err(Error::IN2)?;

		Ok(())
	}

	pub fn set_speed(&mut self, speed: Percentage) -> Result<(), EN::Error>
	{
		self.pin_en.set_duty_cycle(speed)?;

		Ok(())
	}
}

pub struct L298N<ENA: PwmPin, ENB: PwmPin, IN1: OutputPin, IN2: OutputPin, IN3: OutputPin, IN4: OutputPin>
{
	motor_a: L298NMotor<ENA, IN1, IN2>,
	motor_b: L298NMotor<ENB, IN3, IN4>,
}

impl<ENA: PwmPin, ENB: PwmPin, IN1: OutputPin, IN2: OutputPin, IN3: OutputPin, IN4: OutputPin>
	L298N<ENA, ENB, IN1, IN2, IN3, IN4>
{
	pub fn new(pin_en_a: ENA, pin_in1: IN1, pin_in2: IN2, pin_en_b: ENB, pin_in3: IN3, pin_in4: IN4) -> Self
	{
		Self {
			motor_a: L298NMotor::new(pin_en_a, pin_in1, pin_in2),
			motor_b: L298NMotor::new(pin_en_b, pin_in3, pin_in4),
		}
	}

	pub fn a(&mut self) -> &mut L298NMotor<ENA, IN1, IN2>
	{
		&mut self.motor_a
	}
	pub fn b(&mut self) -> &mut L298NMotor<ENB, IN3, IN4>
	{
		&mut self.motor_b
	}
}

pub enum Error<IN1: OutputPin, IN2: OutputPin>
{
	IN1(IN1::Error),
	IN2(IN2::Error),
}
