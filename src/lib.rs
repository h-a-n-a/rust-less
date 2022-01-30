#![feature(in_band_lifetimes)]
#![feature(test)]
#[macro_use]
extern crate strum;
extern crate test;

#[cfg(test)]
mod unit_test;
#[cfg(test)]
mod bench;

pub mod extend;
pub mod new_less;

