//use rand::Rng;

extern crate chrono;
use chrono::{Datelike, Timelike, Utc};

fn main() {
	//println!("Hello, world!");

	let list_num: [i64; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let list_let = ["q","w","e","r","t","y","u","i","o","p","a","s","d","f","g","h","j","k","l","z","x","c","v","b","n","m"];

	println!("{:?}", list_num);

	let now = Utc::now();

	//println!("{:?}", now);

	//let (is_common_era, year) = now.year_ce();
  println!("The current UTC date is {}-{:02}-{:02}", now.year(), now.month(), now.day());
  println!("{}-{}-{}", now.hour(), now.minute(), now.second());
  println!("UTC now in a custom format is: {}", now.format("%Y %m %d - %H:%M:%S%.3f"));

	//println!("{:?}", list_let);
	//println!("{:?}", chrono::offset::Utc::now());
}
