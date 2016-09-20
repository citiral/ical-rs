use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::error::Error;

mod parser;

pub struct Calendar {

}

#[derive(Debug)]
pub enum CalendarError {
	Io(io::Error),
	Parser(parser::ParserError),
}

impl From<io::Error> for CalendarError {
	fn from(err: io::Error) -> CalendarError {
		CalendarError::Io(err)
	}
}

impl From<parser::ParserError> for CalendarError {
	fn from(err: parser::ParserError) -> CalendarError {
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

pub fn from_file(filename: &str) -> Result<Calendar, CalendarError>
{
	// read the file
	let mut file = try!(File::open(filename));
	let mut content = String::new();
    try!(file.read_to_string(&mut content));

	// and parse it
	parser::from_string(&content).map_err(CalendarError::Parser)
}