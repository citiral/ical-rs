use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ParserError {
    InvalidContentLine(&'static str),
    Generic(&'static str)
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            ParserError::InvalidContentLine(ref message) => write!(f, "Parse error: {}", message),
            ParserError::Generic(ref message) => write!(f, "Parse error: {}", message),
        }
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        match *self {
            ParserError::InvalidContentLine(ref message) => message,
            ParserError::Generic(ref message) => message
        }
    }

    fn cause(&self) -> Option<&Error> { None }  
}

#[derive(Debug)]
pub enum CalendarError {
	Io(io::Error),
	Parser(ParserError),
}

impl From<io::Error> for CalendarError {
	fn from(err: io::Error) -> CalendarError {
		CalendarError::Io(err)
	}
}

impl From<ParserError> for CalendarError {
	fn from(err: ParserError) -> CalendarError {
		CalendarError::Parser(err)
	}
}

impl fmt::Display for CalendarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            CalendarError::Io(ref err) => write!(f, "Calendar error: {}", err),
            CalendarError::Parser(ref err) => write!(f, "Calendar error: {}", err),
        }
    }
}

impl Error for CalendarError {
    fn description(&self) -> &str {
    	match *self {
    		CalendarError::Io(ref err) => err.description(),
    		CalendarError::Parser(ref err) => err.description()
    	}
    }

    fn cause(&self) -> Option<&Error> { None }	
}
