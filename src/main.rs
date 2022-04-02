// PACKAGES
extern crate chrono;
use chrono::Utc;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use sha2::{Sha256, Digest}; // SHA-516
// PACKAGES


#[derive(Debug)]
struct BaseMain 
{
	hash: String,
	prev_hash: String,
	time: String,
	num: i64
}

impl BaseMain {
	fn generate_block(&self) {
		let block = BaseMain {
			hash: "".to_string(),
			prev_hash: "".to_string(),
			time: Utc::now().format("%Y%m%d%H%M%S").to_string(),
			num: 0
		};

		let txt: String = "1hello".to_string();
		let hashed: String = block.hash_func(txt);
	}

	fn hash_func(&self, arg: String) -> String {
		let mut hasher = Sha256::new();
		hasher.update(arg);

		let result = hasher.finalize();
		let hashed = hex::encode(result);

		return hashed;
	}

	#[warn(dead_code)]
	fn test(&self) {
		match fs::create_dir("./data") {
			Err(why) => println!("! {:?}", why.kind()),
			Ok(_) => {},
		}
	}
}

fn zero_block() -> BaseMain {
	BaseMain {
		hash: "".to_string(),
		prev_hash: "".to_string(),
		time: Utc::now().format("%Y-%m-%d|%H:%M:%S").to_string(),
		num: 0
	}
}


fn input(rsp: &str) -> String {

	if rsp == "/start"
	{
		return String::from("true");
	}

	else if rsp == "/print"
	{
		return String::from("false");
	}

	else { return String::from("none"); }
}


fn main() {
	let mut block = zero_block();

	//println!("{}-{}-{}-{}", block.hash, block.prev_hash, block.time, block.num);

	//block.hash = block.hash_func(format!("{}-{}-{}-{}", block.hash, block.prev_hash, block.time, block.num));

	//println!("{:#?}", block);

	println!("Enter -> ");

	let mut resp = String::new();
	std::io::stdin()
		.read_line(&mut resp)
		.expect("Failes");

	let x = input(&resp[0..&resp.len() - 2]);

	println!("{:?}", x);
}


