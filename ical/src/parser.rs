use super::Calendar;
use std::result;
use std::str;
use std::iter;
use std::error::Error;
use std::fmt;

pub type Result<T> = result::Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
	EndOfStream,
	Generic(&'static str)
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            ParserError::EndOfStream => write!(f, "IO error: {}", "The stream has terminated before parsing was finished."),
            ParserError::Generic(ref message) => write!(f, "Parse error: {}", message),
        }
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
    	match *self {
    		ParserError::EndOfStream => "The stream has terminated before parsing was finished.",
    		ParserError::Generic(ref message) => message
    	}
    }

    fn cause(&self) -> Option<&Error> { None }	
}

struct CalendarParser<'a> {
	iterator: iter::Peekable<str::Chars<'a>>,
}

impl<'a> CalendarParser<'a> {

	pub fn from_string(content: &'a str) -> CalendarParser<'a> {
		// create a parser and parse a single icalobject
		CalendarParser {
			iterator: content.chars().peekable()
		}
	}

	pub fn expect_crlf(&mut self) -> Result<()> {
		try!(self.expect_char('\u{000D}'));
		try!(self.expect_char('\u{000A}'));
		Ok(())
	}

	pub fn expect_iana_token(&mut self, token: &str) -> Result<()> {
		let val = try!(self.parse_iana_token());

		if val == token {
			Ok(())
		} else {
			println!("found {} expected {}", val, token);
			Err(ParserError::Generic("Found the wrong token."))
		}
	}

	pub fn expect_char(&mut self, expected: char) -> Result<()> {
		let val = self.iterator.next();
		val.map_or(Err(ParserError::EndOfStream), |c| {
			if c == expected {
				Ok(())
			} else {
				Err(ParserError::Generic("Found the wrong char"))
			}
		})
	}

	pub fn parse_iana_token(&mut self) -> Result<String> {
		let mut token = String::new();
		
		// we need at least one token, so check if there is one
		if self.iterator.peek().is_none() {
			return Err(ParserError::EndOfStream);
		}

		// keep reading characters until end of stream or an invalid character is encountered
		loop {
			if let Some(&c) = self.iterator.peek() {
				if c.is_alphanumeric() || c == '-' {
					token.push(c);
					self.iterator.next();
				} else {
					break
				}
			} else {
				break
			}
		}

		Ok(token)
	}

	// actual parsing functions
	pub fn parse_icalobject(&mut self) -> Result<Calendar> {
		try!(self.expect_iana_token("BEGIN"));
		try!(self.expect_char(':'));
		try!(self.expect_iana_token("VCALENDAR"));
		try!(self.expect_crlf());

		// parse the icalbody

		try!(self.expect_iana_token("END"));
		try!(self.expect_char(':'));
		try!(self.expect_iana_token("VCALENDAR"));
		// a file _should_ end with a crlf but we will led it slide to prevent annoying parser errors
		//try!(self.expect_crlf());


		Ok(Calendar {})
	}	
}

pub fn from_string(content: &str) -> Result<Calendar> {
	let mut parser = CalendarParser::from_string(content);	
	parser.parse_icalobject()
}