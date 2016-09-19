extern crate ical;


fn main() {
	let calendar = ical::from_file("input.ics");

	if let Err(error) = calendar {
		println!("Error parsing file: {}", error);
	} else {
		println!("Calendar parsed successfully!");
	}
}
