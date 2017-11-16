//! Encode items that implement formatting traits like `Debug` and `Display`.
//!
//! This might be useful for debugging streams. Note that there is no corresponding `Decoder`.
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
//!     let mut buffer: BytesMut = Default::default();
//!     let mut encoder: DebugEncoder<Option<usize>> = Default::default();
//!     encoder.encode(to_encode, &mut buffer).unwrap();
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
//!     let mut buffer: BytesMut = Default::default();
//!     let mut encoder: DisplayEncoder<String> = Default::default();
//!     encoder.encode(to_encode, &mut buffer).unwrap();
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
pub struct DebugEncoder<I> {
    _i: PhantomData<I>, // TODO can this be removed?
}

impl<I> Default for DebugEncoder<I> {
    fn default() -> Self {
        Self { _i: PhantomData }
    }
}

impl<I: Debug> Encoder for DebugEncoder<I> {
    type Item = I;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        writeln!(dst, "{:?}", item).map_err(Error::Fmt)
    }
}

/// Encode items that implement `Display`, separated by newlines.
pub struct DisplayEncoder<I> {
    _i: PhantomData<I>, // TODO can this be removed?
}

impl<I> Default for DisplayEncoder<I> {
    fn default() -> Self {
        Self { _i: PhantomData }
    }
}

impl<I: Display> Encoder for DisplayEncoder<I> {
    type Item = I;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        writeln!(dst, "{}", item).map_err(Error::Fmt)
    }
}
