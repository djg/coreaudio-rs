#![feature(box_syntax, core)]

pub extern crate "coreaudio-sys" as bindings;
extern crate libc;

pub mod audio_unit;
pub mod error;

