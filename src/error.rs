//! Contains helper structs for error handling

use std::convert::From;
use std::error::Error;
use std::fmt;
use std::io;
use std::num::{ParseFloatError, ParseIntError};

/// A type for results generated by `load_obj` and `load_mtl` where the `Err` type is hard-wired to
/// `ObjError`
///
/// This typedef is generally used to avoid writing out `ObjError` directly and is otherwise a
/// direct mapping to `std::result::Result`.
pub type ObjResult<T> = Result<T, ObjError>;

/// The error type for loading of the `obj` file.
#[derive(Debug)]
pub enum ObjError {
    /// IO error has been occurred during opening the `obj` file.
    Io(io::Error),
    /// Tried to parse integer frome the `obj` file, but failed.
    ParseInt(ParseIntError),
    /// Tried to parse floating point number frome the `obj` file, but failed.
    ParseFloat(ParseFloatError),
    /// `LoadError` has been occurred during parseing the `obj` file.
    Load(LoadError),
}

macro_rules! implmnt {
    ($name:ident, $error:path) => {
        impl From<$error> for ObjError {
            fn from(err: $error) -> Self {
                ObjError::$name(err)
            }
        }
    };
}

impl fmt::Display for ObjError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjError::Io(ref e) => e.fmt(f),
            ObjError::ParseInt(ref e) => e.fmt(f),
            ObjError::ParseFloat(ref e) => e.fmt(f),
            ObjError::Load(ref e) => e.fmt(f),
        }
    }
}

impl Error for ObjError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ObjError::Io(ref err) => Some(err),
            ObjError::ParseInt(ref err) => Some(err),
            ObjError::ParseFloat(ref err) => Some(err),
            ObjError::Load(ref err) => Some(err),
        }
    }
}

implmnt!(Io, io::Error);
implmnt!(ParseInt, ParseIntError);
implmnt!(ParseFloat, ParseFloatError);
implmnt!(Load, LoadError);

/// The error type for parse operations of the `Obj` struct.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LoadError {
    kind: LoadErrorKind,
    message: &'static str,
}

/// A list specifying general categories of load error.
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum LoadErrorKind {
    /// Met unexpected statement.
    UnexpectedStatement,
    /// Received wrong number of arguments.
    WrongNumberOfArguments,
    /// Received unexpected type of arguments.
    WrongTypeOfArguments,
    /// Model should be triangulated first to be loaded properly.
    UntriangulatedModel,
    /// Model cannot be transformed into requested form.
    InsufficientData,
}

impl LoadError {
    /// Creates a new custom error from a specified kind and message.
    pub fn new(kind: LoadErrorKind, message: &'static str) -> Self {
        LoadError { kind, message }
    }
}

impl Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self.kind {
            LoadErrorKind::UnexpectedStatement => "Met unexpected statement",
            LoadErrorKind::WrongNumberOfArguments => "Received wrong number of arguments",
            LoadErrorKind::WrongTypeOfArguments => "Received unexpected type of arguments",
            LoadErrorKind::UntriangulatedModel => {
                "Model should be triangulated first to be loaded properly"
            }
            LoadErrorKind::InsufficientData => "Model cannot be transformed into requested form",
        };

        write!(fmt, "{}: {}", msg, self.message)
    }
}

macro_rules! make_error {
    ($kind:ident, $message:expr) => {
        return Err(::std::convert::From::from($crate::error::LoadError::new(
            $crate::error::LoadErrorKind::$kind,
            $message,
        )));
    };
}
