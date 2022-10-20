#![cfg_attr(not(feature = "std"), no_std)]
#![feature(alloc_error_handler)]

extern crate beam_bvm_interface;
extern crate alloc;

pub mod common;
pub mod app;
pub mod contract;