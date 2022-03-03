#![feature(in_band_lifetimes)]
#![feature(test)]
#[macro_use]
extern crate strum;
extern crate derivative;
extern crate test;

#[cfg(test)]
mod example;
#[cfg(test)]
mod unit_test;

pub mod extend;
pub mod new_less;
