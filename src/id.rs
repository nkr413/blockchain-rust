pub mod id_create {
	use std::collections::hash_map::DefaultHasher;
	use std::hash::{Hash, Hasher};

	#[derive(Hash)]
	struct User {
		id: i64,
		name: String
	}

	pub fn hashing() {
		let user1 = User {
			id: 123456789,
			name: "Kamronbek".to_string()
		};

		let user2 = User {
			id: 298328782,
			name: "Putin".to_string()
		};

		//assert!(calculate_hash(&user1) != calculate_hash(&user2));
		assert_eq!(calculate_hash(&user1), calculate_hash(&user2));

		fn calculate_hash<T: Hash>(t: &T) {
			let mut s = DefaultHasher::new();
			t.hash(&mut s);
			s.finish();

			println!("{:?}", s);
		}
	}
}