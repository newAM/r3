//! R3 PortKit
#![feature(const_generics)]
#![feature(const_fn)]
#![feature(const_panic)]
#![feature(core_panic)]
#![feature(decl_macro)]
#![feature(external_doc)]
#![no_std]

#[macro_use]
pub mod utils;

pub mod num;
pub mod pptext;
pub mod tickful;
pub mod tickless;
