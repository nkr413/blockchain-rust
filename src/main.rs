mod hashing;
use hashing::main::{hash_func};

use rand::Rng;
extern crate chrono;
use chrono::Utc;

fn main() {
	let now = Utc::now();

	let list_num: [i64; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let	list_let = ["q","w","e","r","t","y","u","i","o","p","a","s","d","f","g","h","j","k","l","z","x","c","v","b","n","m"];
	let	time = now.format("%Y%m%d%H%M%S").to_string();
	let id_lim: i64 = 40;

	let txt: String = "hello".to_string();
	let hashed: String = hash_func(txt); // mod hashing.rs

	println!("{:?}", hashed);
}


