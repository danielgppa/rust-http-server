use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: Method,
        }

        impl Request {
            fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
        }

        impl TryFrom<&[u8]> for Request {
            type Error = ParseError;

            fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
                unimplemented!()
            }
        }

        pub enum ParseError {
            InvalidRequest,
            InvalidEncoding,
            InvalidProtocol,
            InvalidMethod,
        }

        impl ParseError {
            fn description(&self) -> &str {
                match self {
                    Self::InvalidRequest => "Invalid Request",
                    Self::InvalidEncoding => "Invalid Encoding",
                    Self::InvalidProtocol => "Invalid Protocol",
                    Self::InvalidMethod => "Invalid Method",
                }
            }
        }

        impl Display for ParseError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.description())
            }
        }

        impl Debug for ParseError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.description())
            }
        }

        impl Error for ParseError {}

        trait Encrypt {
            fn encrypt(&self) -> Self {
                unimplemented!()
            }
        }

        impl Encrypt for &[u8] {
            fn encrypt(&self) -> Self {
                unimplemented!()
            }
        }