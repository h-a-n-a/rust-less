#![feature(test)]
#[macro_use]
extern crate strum;
extern crate test;
extern crate core;

#[cfg(test)]
mod example;
#[cfg(test)]
mod unit_test;

pub mod extend;
pub mod new_less;
