// PACKAGES
extern crate chrono;
use chrono::Utc;

// IMPORT
mod hashing; 
use hashing::main::{hash_func};

mod main_struct;
use main_struct::struct_mod::BaseMain;


fn main() {
	let base = BaseMain {
		time: Utc::now().format("%Y%m%d%H%M%S").to_string(),
		num: 9
	};

	let txt: String = "1hello".to_string();
	let hashed: String = hash_func(txt); // mod hashing.rs

	//test(base);

	println!("{:?}", base);
}


