extern crate bs58;
extern crate tiny_keccak;

use tiny_keccak::Keccak;

const ITERATIONS: usize = 16_384;

/// ```rust
/// extern crate my_password;
///
/// use my_password::password;
///
/// fn main() {
/// 	assert_eq!(password("Master Password", "test@gmail.com", "facebook"), "J3Penz9p9bfgmKUB5AUwyyAfCtpm96yz9HfGL8SUf9fp".to_owned());
/// }
/// ```
pub fn password(master_password: &str, email: &str, service: &str) -> String {
	let mut sha3_result = [0u8; 32];
	let mut sha3 = Keccak::new_sha3_256();
	sha3.update(master_password.as_bytes());
	sha3.update(email.as_bytes());
	sha3.update(service.as_bytes());
	sha3.finalize(&mut sha3_result);
	for _ in 0..ITERATIONS {
		let mut sha3 = Keccak::new_sha3_256();
		sha3.update(&sha3_result);
		sha3.finalize(&mut sha3_result);
	}
	bs58::encode(sha3_result).into_string()
}
