// PACKAGES
extern crate chrono;
use chrono::Utc;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use sha2::{Sha512, Digest};
// PACKAGES


#[derive(Debug)]
struct BaseMain 
{
	time: String,
	num: i64
}

impl BaseMain {
	fn hash_func(&self, arg: String) -> String {
		let mut hasher = Sha512::new();
		hasher.update(arg);

		let result = hasher.finalize();
		let hashed = hex::encode(result);

		return hashed;
	}

	fn test(&self) {
		match fs::create_dir("./data") {
			Err(why) => println!("! {:?}", why.kind()),
			Ok(_) => {},
		}
	}
}


fn main() {
	let base = BaseMain {
		time: Utc::now().format("%Y%m%d%H%M%S").to_string(),
		num: 9
	};

	let txt: String = "1hello".to_string();
	let hashed: String = base.hash_func(txt); // mod hashing.rs


	println!("{:?}", hashed);
}


