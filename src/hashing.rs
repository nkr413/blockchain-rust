pub mod main {
	pub use sha2::{Sha512, Digest};

	//mod main_struct;
	//use main_struct::struct_mod::BaseMain;

	pub fn hash_func(arg: String) -> String {
		let mut hasher = Sha512::new();
		hasher.update(arg);

		let result = hasher.finalize();
		let hashed = hex::encode(result);

		return hashed;
	}

	// pub fn test(arg: Struct) {
	// 	println!("{:?}", arg);
	// }
}