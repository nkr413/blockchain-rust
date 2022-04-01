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
		let mut hasher = Sha512::new();
		hasher.update(arg);

		let result = hasher.finalize();
		let hashed = hex::encode(result);

		return hashed;
	}

	fn test(&self) {
		match fs::create_dir("./data/base.json") {
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


fn main() {
	let mut block = zero_block();

	block.hash = block.hash_func(format!("{}-{}-{}-{}", block.hash, block.prev_hash, block.time, block.num));

	println!("{:#?}", block);
}


