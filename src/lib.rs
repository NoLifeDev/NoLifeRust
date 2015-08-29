// Copyright © 2014, Peter Atashian
//! A high performance Rust library used to read [NX files](http://nxformat.github.io/) with
//! minimal memory usage.
#![warn(missing_docs)]

extern crate memmap;

pub use file::{Error, File};
pub use node::{GenericNode, Node, Type};

pub mod audio;
pub mod file;
pub mod node;
mod repr;

