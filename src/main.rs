use std::fs;

use hsapi::project_from_json;

fn main() {
	let json = fs::read_to_string("test.hopscotch").expect("Read file");
	let project  = project_from_json(&json).expect("parse");
	println!("{:?}", project)
}