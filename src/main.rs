use std::fs;

use hsapi::project_from_json;

fn main() {
	let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];
	let json = fs::read_to_string(file_path).expect("Read file");
	let mut project  = project_from_json(&json).expect("parse");
	// Convert all comment blocks to none blocks
	project.blocks_iter_mut().for_each(|block| {
		// Currently only ArbitraryID exists, in the future add KnownBlock(HSBlockType) or something
		match &block.hs_type {
			hsapi::BlockType::ArbitraryID(id) => {
				// `*id`? Is this the correct way to do this?
				if *id == 69.0 {
					block.hs_type = hsapi::BlockType::ArbitraryID(22.0);
				}
			}
		}
	});
	let new_json = project.jsonify().expect("jsonify");
	println!("{}", new_json)
}