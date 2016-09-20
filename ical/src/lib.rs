use std::io;
use std::io::prelude::*;
use std::fs::File;
use error::CalendarError;

mod parser;
mod error;

#[derive(Debug)]
enum CalendarScale {
	Gregorian,
}

#[derive(Debug)]
enum CalendarVersion {
	Version_2_0,
}

#[derive(Debug)]
pub struct CalendarProperties {
	pub prodid: String,
	pub version: String,
	pub calscale: CalendarScale,
	pub method: Option<String>,
}

#[derive(Debug)]
pub struct Calendar {
	pub properties: CalendarProperties
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