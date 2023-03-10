enum MyError {
	Io(std::io::Error),
	Num(std::num::ParseIntError),
}

use std::fmt;

impl fmt::Display for MyError {
	fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
		match self {
			MyError::Io(e) => write!(f, "IO error: {}", e),
			MyError::Num(e) => write!(f, "Parse error: {}", e),
		}
	}
}

impl From<std::io::Error> for MyError {
	fn from(cause: std::io::Error) -> Self {
		MyError::Io(cause)
	}
}

fn get_int_from_file () -> Result<i32, MyError> {
	let path = "number.txt";
	let num_str = std::fs::read_to_string(path).map_err(MyError::from)?;

	num_str.trim()
		.parse::<i32>()
		.map(|t| t * 2)
		.map_err(|e| MyError::Num(e))
}

fn main() {
	match get_int_from_file() {
		Ok(x) => println!("{}", x),
		Err(e) => match e {
			MyError::Io(e) => println!("IO error: {}", e),
			MyError::Num(e) => println!("Parse error: {}", e),
		},
	}
}