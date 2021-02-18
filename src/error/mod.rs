use std::error;
use std::fmt;
use std::io;
use std::result;

/// A type alias for `Result<T, dpt::Error>`.
///
/// This result type embeds the error type in this crate.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur when using `dpt`.
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// A crate private constructor for `Error`.
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }

    /// Returns the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    /// Unwraps this error into its underlying type.
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

/// The specific type of an error.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Can occur when reading or writing to a file.
    Io(io::Error),
    /// Can occur when walking directory entries.
    WalkDir(walkdir::Error),
    /// Occurs when concatenating files failed.
    Concat(fcc::Error),
    /// Can occur when executing some browser action.
    CmdError(fantoccini::error::CmdError),
    /// Cannot establish a session for a new browser client.
    NewSessionError(fantoccini::error::NewSessionError),
    /// Failed to decode Chinese character sets (GBK, GB18030)
    Decode(String),
    /// `config.toml` is invalid or incomplete.
    Config(String),
    /// Parsing string to another type failed.
    FromStr(String, &'static str),
    /// Encountering malformed data.
    MalformedData(String, usize),
    /// The given file refused to access or is not found.
    Access(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::WalkDir(ref err) => err.fmt(f),
            ErrorKind::CmdError(ref err) => err.fmt(f),
            ErrorKind::NewSessionError(ref err) => err.fmt(f),
            ErrorKind::Decode(ref err) => write!(f, "decode error: failed to decode `{}`", err),
            ErrorKind::Config(ref err) => {
                write!(f, "config error: `{}` is invalid or not found", err)
            }
            ErrorKind::FromStr(ref src, ref ty) => {
                write!(f, "parse error: failed to parse `{}` as `{}`", src, ty)
            }
            ErrorKind::MalformedData(ref msg, ref num) => {
                write!(f, "{} (line: {})", msg, num)
            }
            ErrorKind::Access(ref path) => {
                write!(f, "access error: failed to access `{}`", path)
            }
            ErrorKind::Concat(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

macro_rules! impl_from_error {
    ($err:ty, $ident:ident) => {
        impl From<$err> for Error {
            fn from(err: $err) -> Error {
                Error::new(ErrorKind::$ident(err))
            }
        }
    };
}

impl_from_error!(io::Error, Io);
impl_from_error!(walkdir::Error, WalkDir);
impl_from_error!(fcc::Error, Concat);
impl_from_error!(fantoccini::error::CmdError, CmdError);
impl_from_error!(fantoccini::error::NewSessionError, NewSessionError);
