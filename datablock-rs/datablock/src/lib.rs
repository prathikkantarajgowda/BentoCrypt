#![no_std]

#[macro_use]
extern crate datablock_derive;

pub mod datablock;

pub use datablock::{DataBlock, Error};

pub use datablock_derive::*;
