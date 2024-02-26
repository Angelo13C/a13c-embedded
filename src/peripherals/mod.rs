//! Module that contains various submodules representing different peripherals commonly
//! found in embedded systems, such as ADC, timers, UART, SPI, etc.
//!
//! Each submodule contains functionality and abstractions specific to the corresponding peripheral.
//! Consult individual submodule documentation for more details.

pub mod adc;
pub mod interrupt;
pub mod pwm;
pub mod system_time;
pub mod timer;
