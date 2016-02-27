// Copyright 2016 Serde YAML Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.extern crate serde;

use std::error;
use std::fmt;
use std::result;
use std::string::FromUtf8Error;

use yaml_rust::{emitter, scanner};

use serde::{ser, de};

/// This type represents all possible errors that can occur when serializing or
/// deserializing a value using YAML.
#[derive(Debug)]
pub enum Error {
    Custom(String),
    EndOfStream,

    Emit(emitter::EmitError),
    Scan(scanner::ScanError),
    FromUtf8(FromUtf8Error),

    AliasUnsupported,
    TooManyDocuments(usize),
    VariantMapWrongSize(String, usize),
    VariantNotAMap(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Custom(_) => "syntax error",
            Error::EndOfStream => "EOF while parsing a value",
            Error::Emit(_) => "emit error",
            Error::Scan(_) => "scan error",
            Error::FromUtf8(ref err) => err.description(),
            Error::AliasUnsupported => "YAML aliases are not supported",
            Error::TooManyDocuments(_) =>
                "expected a single YAML document but found multiple",
            Error::VariantMapWrongSize(..) =>
                "expected a YAML map of size 1 while parsing variant",
            Error::VariantNotAMap(_) =>
                "expected a YAML map while parsing variant",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Scan(ref err) => Some(err),
            Error::FromUtf8(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Custom(ref msg) =>
                write!(f, "{}", msg),
            Error::EndOfStream =>
                write!(f, "EOF while parsing a value"),
            Error::Emit(ref err) =>
                write!(f, "{:?}", err),
            Error::Scan(ref err) =>
                write!(f, "{:?}", err),
            Error::FromUtf8(ref err) =>
                write!(f, "{:?}", err),
            Error::AliasUnsupported =>
                write!(f, "YAML aliases are not supported"),
            Error::TooManyDocuments(n) =>
                write!(f, "Expected a single YAML document but found {}", n),
            Error::VariantMapWrongSize(ref variant, size) =>
                write!(f, "Expected a YAML map of size 1 while parsing variant {} but was size {}", variant, size),
            Error::VariantNotAMap(ref variant) =>
                write!(f, "Expected a YAML map while parsing variant {}", variant),
        }
    }
}

impl From<emitter::EmitError> for Error {
    fn from(err: emitter::EmitError) -> Error {
        Error::Emit(err)
    }
}

impl From<scanner::ScanError> for Error {
    fn from(err: scanner::ScanError) -> Error {
        Error::Scan(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::FromUtf8(err)
    }
}

impl ser::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Custom(msg.into())
    }
}

impl de::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Custom(msg.into())
    }

    fn end_of_stream() -> Self {
        Error::EndOfStream
    }
}

/// Helper alias for `Result` objects that return a YAML `Error`.
pub type Result<T> = result::Result<T, Error>;
