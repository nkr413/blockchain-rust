pub mod main {
	use sha2::{Sha256, Sha512, Digest};

	pub fn hash_func(txt: String) -> String {
		let mut hasher = Sha512::new();
		hasher.update(txt);

		let result = hasher.finalize();
		let hashed = hex::encode(result);

		return hashed;
	}
}