#![allow(unused_variables)]

mod adc;
#[cfg(feature = "embedded-svc")]
mod connection;
mod error;
mod input;
mod output;
mod pwm;
mod spi;
mod time;
mod timer;
mod uart;
mod watchdog;

pub use adc::*;
#[cfg(feature = "embedded-svc")]
pub use connection::*;
pub use error::*;
pub use input::*;
pub use output::*;
pub use pwm::*;
pub use spi::*;
pub use time::*;
pub use timer::*;
pub use uart::*;
pub use watchdog::*;
