use super::Calendar;
use std::result;
use std::str;
use std::iter;
use std::error::Error;
use std::fmt;

pub type Result<T> = result::Result<T, ParserError>;

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
struct ContentLine {
	name: String,
	value: String,
	param: Vec<ContentParam>
}

#[derive(Debug)]
struct ContentParam {
	name: String,
	value: Vec<String>,
}

#[derive(Debug)]
struct CalendarRaw {
	lines: Vec<ContentLine>,
}

fn fetch_next_unfolded_line(iter: &mut iter::Peekable<str::Lines>) -> Option<String> {
	// get the next line
	iter.next().map(|line| {
		let mut result = line.to_string();

		// keep peeking the next line and appending it if it is a folded line
		while iter.peek().map_or(false, |l| l.find(char::is_whitespace) == Some(0)) {
			let (_, trimmed) = iter.next().unwrap().split_at(1);
			result.push_str(trimmed);
		};
		
		// return the unfolded line, or None if the iterator was empty
		result
	})
}

fn parse_content_param(line: &str) -> Result<ContentParam> {
	println!("TODO: parse parameter");
	Ok(ContentParam {
		name: "todo".to_string(),
		value: Vec::new(),
	})
}

fn parse_content_line(line: &str) -> Result<ContentLine> {
	// split at the first ':' or return an error if no ':' is found.
	let (name_and_param, value) = line.split_at(try!(line.find(':').ok_or(ParserError::InvalidContentLine("No ':' found in contentline."))));

	// parse the name_and_param
	let mut iter = name_and_param.split(';');
	let name = iter.next().unwrap(); // there is always at least one result
	let mut params = Vec::new();
	for param in iter {
		params.push(try!(parse_content_param(param)));
	}

	Ok(ContentLine {
		name: name.to_string(),
		value: value.to_string(),
		param: params,
	})
}

pub fn from_string(content: &str) -> Result<Calendar> {
	let mut raw = CalendarRaw {
		lines: Vec::new()
	};

	// first parse all content lines
	let mut iter = content.lines().peekable();
	let mut lines = Vec::<ContentLine>::new();

	while let Some(line) = fetch_next_unfolded_line(&mut iter) {
		lines.push(try!(parse_content_line(&line)));
	};

	Ok(Calendar{})
}