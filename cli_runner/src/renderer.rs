use lyon_path::Path;
use std::{io, fmt};
use crate::renderer::RenderingError::{SvgParsingError, StringParsingError, IOError};

pub trait Renderer{
    fn update_display(&mut self) -> Result<Path, RenderingError>;
}

pub enum RenderingError{
    ArgumentError,
    DummyError,
    XmlParsingError,
    SvgParsingError,
    StringParsingError,
    IOError(std::io::Error),
}
impl From<lyon::lyon_svg::path_utils::ParseError> for RenderingError {
    fn from(_: lyon::lyon_svg::path_utils::ParseError) -> Self {
        SvgParsingError
    }
}

impl From<std::io::Error> for RenderingError {
    fn from(error: std::io::Error) -> Self {
        IOError(error)
    }
}

impl From<std::num::ParseFloatError> for RenderingError {
    fn from(error: std::num::ParseFloatError) -> Self {
        StringParsingError
    }
}

impl fmt::Display for RenderingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderingError::ArgumentError =>
                write!(f, "Couldn't parse given arguments"),
            RenderingError::SvgParsingError =>
                write!(f, "Couldn't parse given svg file. Possibly wrong format."),
            RenderingError::IOError(ref e) =>
                e.fmt(f),
            _ =>
                write!(f, "Message not implemented for this error code"),
        }
    }
}

