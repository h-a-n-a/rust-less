#![feature(test)]
#[macro_use]
extern crate strum;
extern crate core;
extern crate test as unit_test;

pub mod extend;
pub mod new_less;
#[cfg(test)]
mod test;
#[cfg(test)]
mod example;
