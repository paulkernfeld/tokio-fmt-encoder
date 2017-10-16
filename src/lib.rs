//! Encode items that implement formatting traits like `Debug` and `Display`.
//!
//! To encode an item that implements `Debug`:
//!
//! ```
//! extern crate bytes;
//! extern crate tokio_fmt_encoder;
//! extern crate tokio_io;
//!
//! fn main() {
//!     use bytes::BytesMut;
//!     use std::fmt::Formatter;
//!     use tokio_fmt_encoder::DebugEncoder;
//!     use tokio_io::codec::Encoder;
//!
//!     let to_encode = Some(10);
//!     let mut buffer = BytesMut::with_capacity(64);
//!     let mut encoder: DebugEncoder<Option<usize>> = Default::default();
//!     encoder.encode(&to_encode, &mut buffer).unwrap();
//!     assert_eq!(&buffer.take(), &"Some(10)\n");
//! }
//! ```
//!
//! To encode an item that implements `Display`:
//!
//! ```
//! extern crate bytes;
//! extern crate tokio_fmt_encoder;
//! extern crate tokio_io;
//!
//! fn main() {
//!     use bytes::BytesMut;
//!     use std::fmt::Formatter;
//!     use tokio_fmt_encoder::DisplayEncoder;
//!     use tokio_io::codec::Encoder;
//!
//!     let to_encode = String::from("hello");
//!     let mut buffer = BytesMut::with_capacity(64);
//!     let mut encoder: DisplayEncoder<String> = Default::default();
//!     encoder.encode(&to_encode, &mut buffer).unwrap();
//!     assert_eq!(&buffer.take(), &"hello\n");
//! }
//! ```
//!
#![deny(warnings)]
#![allow(deprecated)]
extern crate bytes;
extern crate tokio_io;

use bytes::BytesMut;
use std::fmt::{Debug, Display, Write};
use std::marker::PhantomData;
use tokio_io::codec::Encoder;


#[derive(Debug)]
pub enum Error {
    Fmt(std::fmt::Error),
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

/// Encode items that implement `Debug`, separated by newlines.
pub struct DebugEncoder<'a, I: 'a> {
    _i: PhantomData<&'a I>, // TODO can this be removed?
}

impl<'a, I> Default for DebugEncoder<'a, I> {
    fn default() -> Self {
        Self { _i: PhantomData }
    }
}

impl<'a, I: Debug + 'a> Encoder for DebugEncoder<'a, I> {
    type Item = &'a I;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        writeln!(dst, "{:?}", item).map_err(Error::Fmt)
    }
}

/// Encode items that implement `Display`, separated by newlines.
pub struct DisplayEncoder<'a, I: 'a> {
    _i: PhantomData<&'a I>, // TODO can this be removed?
}

impl<'a, I> Default for DisplayEncoder<'a, I> {
    fn default() -> Self {
        Self { _i: PhantomData }
    }
}

impl<'a, I: Display + 'a> Encoder for DisplayEncoder<'a, I> {
    type Item = &'a I;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        writeln!(dst, "{}", item).map_err(Error::Fmt)
    }
}
