#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(register_tool)]
#![register_tool(c2rust)]


extern crate libc;
pub mod src {
pub mod buffer;
pub mod csv;
pub mod encoding;
pub mod fec;
pub mod memory;
pub mod urlopen;
pub mod writer;
} // mod src
