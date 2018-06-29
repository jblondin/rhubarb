use std::error::Error;
use std::fmt;

use serde_json;
use handlebars::{TemplateFileError, RenderError};
use agnes::error::AgnesError;

use event::Event;

/// Generate Rhubarb error enum.
#[derive(Debug)]
pub enum RhubarbError {
    /// Dataframe-based error
    DataFrame(AgnesError),
    /// Serde JSON error
    Json(serde_json::Error),
    /// Template (Handlebars) Error
    Template(Box<Error>),
    /// Missing key in the component registry
    ComponentRegistry(String),
    /// Invalid call adding a component to a layout
    InvalidLayout(String),
    /// Invalid event for specified component
    InvalidEvent {
        event: Event,
        component_type: String,
    }
}

/// Wrapper for Rhubarb-based results.
pub type Result<T> = ::std::result::Result<T, RhubarbError>;

impl fmt::Display for RhubarbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RhubarbError::DataFrame(ref err) => write!(f, "DataFrame error: {}", err),
            RhubarbError::Json(ref err) => write!(f, "JSON error: {}", err),
            RhubarbError::Template(ref err) => write!(f, "Templating error: {}", err),
            RhubarbError::ComponentRegistry(ref s) => write!(f,
                "Missing component name in component registry: {}", s),
            RhubarbError::InvalidLayout(ref s) => write!(f,
                "Invalid layout: {}", s),
            RhubarbError::InvalidEvent { ref event, ref component_type } => write!(f,
                "Invalid event '{}' for component '{}'", event.name(), component_type),
        }
    }
}

impl Error for RhubarbError {
    fn description(&self) -> &str {
        match *self {
            RhubarbError::DataFrame(ref err) => err.description(),
            RhubarbError::Json(ref err) => err.description(),
            RhubarbError::Template(ref err) => err.description(),
            RhubarbError::ComponentRegistry(_) => "missing component",
            RhubarbError::InvalidLayout(_) => "invalid layout",
            RhubarbError::InvalidEvent { .. } => "invalid event for component"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RhubarbError::DataFrame(ref err) => Some(err),
            RhubarbError::Json(ref err) => Some(err),
            RhubarbError::Template(ref err) => Some(err.as_ref()),
            RhubarbError::ComponentRegistry(_) => None,
            RhubarbError::InvalidLayout(_) => None,
            RhubarbError::InvalidEvent { .. } => None,
        }
    }
}

impl From<AgnesError> for RhubarbError {
    fn from(err: AgnesError) -> RhubarbError {
        RhubarbError::DataFrame(err)
    }
}
impl From<serde_json::Error> for RhubarbError {
    fn from(err: serde_json::Error) -> RhubarbError {
        RhubarbError::Json(err)
    }
}
impl From<TemplateFileError> for RhubarbError {
    fn from(err: TemplateFileError) -> RhubarbError {
        RhubarbError::Template(Box::new(err))
    }
}
impl From<RenderError> for RhubarbError {
    fn from(err: RenderError) -> RhubarbError {
        RhubarbError::Template(Box::new(err))
    }
}
