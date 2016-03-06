// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate libc;
extern crate rustc_serialize;

mod hash;
mod cache;

use std::path::Path;
use std::ffi::CStr;
use cache::Cache;

static LIB_CACHE: Cache<'static> = Cache { cache_dir: cache::CACHE_DIR, use_cache: true };

/**
 * Prepare the library for work.
 *
 * Not performing this step may cause parts to fail.
 */
 #[no_mangle]
 pub extern "C" fn init() {
 	match LIB_CACHE.init() {
 		Ok(_) => {}
 		Err(e) => println!("Error: {}", e),
 	}
 }

/**
 * Teardown for the library
 */
 #[no_mangle]
 pub extern "C" fn teardown() {
 	match LIB_CACHE.clean() {
 		Ok(_) => {}
 		Err(e) => println!("Error: {}", e),
 	}
 }

 pub fn get_phashes(path: &Path) -> hash::PerceptualHashes {
 	hash::get_perceptual_hashes(path, &hash::Precision::Medium, &LIB_CACHE)
 }


 pub fn get_ahash(path: &Path) -> u64 {
 	hash::get_perceptual_hash(&path, &hash::Precision::Medium, &hash::HashType::AHash, &LIB_CACHE)
 }

 pub fn get_dhash(path: &Path) -> u64 {
 	hash::get_perceptual_hash(&path, &hash::Precision::Medium, &hash::HashType::DHash, &LIB_CACHE)
 }

 pub fn get_phash(path: &Path) -> u64 {
 	hash::get_perceptual_hash(&path, &hash::Precision::Medium, &hash::HashType::DHash, &LIB_CACHE)
 }

 pub fn get_hamming_distance(hash1: u64, hash2: u64) -> u64 {
 	hash::calculate_hamming_distance(hash1, hash2)
 }

 // External proxies for the get_*hash methods

 #[no_mangle]
 pub extern "C" fn ext_get_ahash(path_char: *const libc::c_char) -> libc::uint64_t {
 	unsafe {
 		let path_str = CStr::from_ptr(path_char);
 		let image_path = match path_str.to_str() {
 			Ok(result) => result,
 			Err(e) => {
 				println!("Error: {}. Unable to parse '{}'",
 					e,
 					to_hex_string(path_str.to_bytes()));
 				panic!("Unable to parse path")
 			}
 		};
 		let path = Path::new(&image_path);
 		get_ahash(&path)
 	}
 }

 #[no_mangle]
 pub extern "C" fn ext_get_dhash(path_char: *const libc::c_char) -> libc::uint64_t {
 	unsafe {
 		let path_str = CStr::from_ptr(path_char);
 		let image_path = match path_str.to_str() {
 			Ok(result) => result,
 			Err(e) => {
 				println!("Error: {}. Unable to parse '{}'",
 					e,
 					to_hex_string(path_str.to_bytes()));
 				panic!("Unable to parse path")
 			}
 		};
 		let path = Path::new(&image_path);
 		get_dhash(&path)
 	}
 }

 #[no_mangle]
 pub extern "C" fn ext_get_phash(path_char: *const libc::c_char) -> libc::uint64_t {
 	unsafe {
 		let path_str = CStr::from_ptr(path_char);
 		let image_path = match path_str.to_str() {
 			Ok(result) => result,
 			Err(e) => {
 				println!("Error: {}. Unable to parse '{}'",
 					e,
 					to_hex_string(path_str.to_bytes()));
 				panic!("Unable to parse path")
 			}
 		};
 		let path = Path::new(&image_path);
 		get_phash(&path)
 	}
 }

 fn to_hex_string(bytes: &[u8]) -> String {
 	println!("length: {}", bytes.len());
 	let mut strs: Vec<String> = Vec::new();
 	for byte in bytes {
 		// println!("{:02x}", byte);
 		strs.push(format!("{:02x}", byte));
 	}
 	strs.join("\\x")
 }

 // Module for the tests
 //
 #[cfg(test)]
 mod tests {

 	use super::*;
 	use std::fs;
 	use std::path::Path;
 	use hash;

 	#[test]
 	fn test_can_get_test_images() {
 		let paths = fs::read_dir(&Path::new("./test_images")).unwrap();
 		let mut num_paths = 0;
 		for path in paths {
 			let orig_path = path.unwrap().path();
 			let ext = Path::new(&orig_path).extension();
 			match ext {
 				Some(_) => {
 					if ext.unwrap() == "jpg" {
 						num_paths += 1;
 						println!("Is a image {}: {:?}", num_paths, orig_path) ;
 					}
 				}
 				_ => {
 					println!("Not an image: {:?}", orig_path) ;
 					continue;
 				}
 			}
 			// println!("Name: {}", path.unwrap().path().display())
 		}
 		// Currently 12 images in the test imaages directory
 		assert!(num_paths == 12);
 	}

		/**
		* Updated test function. Assumes 3 images to a set and no hamming distances.
		* We don't need to confirm that the hamming distance calculation works in these tests.
		*/
		fn test_imageset_hash(hash_type: hash::HashType,
			hash_precision: hash::Precision,
			image_paths: [&Path; 3],
			image_hashes: [u64; 3]) {
			for index in 0..image_paths.len() {
				let image_path = image_paths[index];
				let calculated_hash = hash::get_perceptual_hash(&image_path, &hash_precision, &hash_type, &super::LIB_CACHE);
				println!("Image hashes for '{}': expected: {} actual: {}",
					image_path.to_str().unwrap(),
					image_hashes[index],
					calculated_hash);
				assert!(calculated_hash == image_hashes[index]);
			}
		}

		#[test]
		fn test_confirm_ahash_results() {
			// Prep_Cache
			super::init();

			// Sample_01 tests
			let sample_01_images: [&Path; 3] = [&Path::new("./test_images/sample_01_large.jpg"),
												&Path::new("./test_images/sample_01_medium.jpg"),
												&Path::new("./test_images/sample_01_small.jpg")];
			let sample_01_hashes: [u64; 3] = [857051991849750,
												857051991849750,
												857051991849750];
			test_imageset_hash(hash::HashType::AHash, hash::Precision::Medium, sample_01_images, sample_01_hashes);

			// Sample_02 tests
			let sample_02_images: [&Path; 3] = [&Path::new("./test_images/sample_02_large.jpg"),
												&Path::new("./test_images/sample_02_medium.jpg"),
												&Path::new("./test_images/sample_02_small.jpg")];
			let sample_02_hashes: [u64; 3] = [18446744073441116160,
												18446744073441116160,
												18446744073441116160];
			test_imageset_hash(hash::HashType::AHash, hash::Precision::Medium, sample_02_images, sample_02_hashes);

			// Sample_03 tests
			let sample_03_images: [&Path; 3] = [&Path::new("./test_images/sample_03_large.jpg"),
												&Path::new("./test_images/sample_03_medium.jpg"),
												&Path::new("./test_images/sample_03_small.jpg")];
			let sample_03_hashes: [u64; 3] = [135670932300497406,
												135670932300497406,
												135670932300497406];
			test_imageset_hash(hash::HashType::AHash, hash::Precision::Medium, sample_03_images, sample_03_hashes);

			// Sample_04 tests
			let sample_04_images: [&Path; 3] = [&Path::new("./test_images/sample_04_large.jpg"),
												&Path::new("./test_images/sample_04_medium.jpg"),
												&Path::new("./test_images/sample_04_small.jpg")];
			let sample_04_hashes: [u64; 3] = [18446460933225054208,
												18446460933090836480,
												18446460933090836480];
			test_imageset_hash(hash::HashType::AHash, hash::Precision::Medium, sample_04_images, sample_04_hashes);

			// Clean_Cache
			// super::teardown();
		}

		#[test]
		fn test_confirm_dhash_results() {
			// Prep_Cache
			super::init();

			// Sample_01 tests
			let sample_01_images: [&Path; 3] = [&Path::new("./test_images/sample_01_large.jpg"),
												&Path::new("./test_images/sample_01_medium.jpg"),
												&Path::new("./test_images/sample_01_small.jpg")];
			let sample_01_hashes: [u64; 3] = [7937395827556495926,
												7937395827556495926,
												7939647627370181174];
			test_imageset_hash(hash::HashType::DHash, hash::Precision::Medium, sample_01_images, sample_01_hashes);

			// Sample_02 tests
			let sample_02_images: [&Path; 3] = [&Path::new("./test_images/sample_02_large.jpg"),
												&Path::new("./test_images/sample_02_medium.jpg"),
												&Path::new("./test_images/sample_02_small.jpg")];
			let sample_02_hashes: [u64; 3] = [11009829669713008949,
												11009829670249879861,
												11009829669713008949];
			test_imageset_hash(hash::HashType::DHash, hash::Precision::Medium, sample_02_images, sample_02_hashes);

			// Sample_03 tests
			let sample_03_images: [&Path; 3] = [&Path::new("./test_images/sample_03_large.jpg"),
												&Path::new("./test_images/sample_03_medium.jpg"),
												&Path::new("./test_images/sample_03_small.jpg")];
			let sample_03_hashes: [u64; 3] = [225528496439353286,
												225528496439353286,
												226654396346195908];
			test_imageset_hash(hash::HashType::DHash, hash::Precision::Medium, sample_03_images, sample_03_hashes);

			// Sample_04 tests
			let sample_04_images: [&Path; 3] = [&Path::new("./test_images/sample_04_large.jpg"),
												&Path::new("./test_images/sample_04_medium.jpg"),
												&Path::new("./test_images/sample_04_small.jpg")];
			let sample_04_hashes: [u64; 3] = [14620651386429567209,
												14620651386429567209,
												14620651386429567209];
			test_imageset_hash(hash::HashType::DHash, hash::Precision::Medium, sample_04_images, sample_04_hashes);

			// Clean_Cache
			// super::teardown();
		}

		#[test]
		fn test_confirm_phash_results() {
			// Prep_Cache
			super::init();

			// Sample_01 tests
			let sample_01_images: [&Path; 3] = [&Path::new("./test_images/sample_01_large.jpg"),
												&Path::new("./test_images/sample_01_medium.jpg"),
												&Path::new("./test_images/sample_01_small.jpg")];
			let sample_01_hashes: [u64; 3] = [72357778504597504,
												72357778504597504,
												72357778504597504];
			test_imageset_hash(hash::HashType::PHash, hash::Precision::Medium, sample_01_images, sample_01_hashes);

			// Sample_02 tests
			let sample_02_images: [&Path; 3] = [&Path::new("./test_images/sample_02_large.jpg"),
												&Path::new("./test_images/sample_02_medium.jpg"),
												&Path::new("./test_images/sample_02_small.jpg")];
			let sample_02_hashes: [u64; 3] = [5332332327550844928,
												5332332327550844928,
												5332332327550844928];
			test_imageset_hash(hash::HashType::PHash, hash::Precision::Medium, sample_02_images, sample_02_hashes);

			// Sample_03 tests
			let sample_03_images: [&Path; 3] = [&Path::new("./test_images/sample_03_large.jpg"),
												&Path::new("./test_images/sample_03_medium.jpg"),
												&Path::new("./test_images/sample_03_small.jpg")];
			let sample_03_hashes: [u64; 3] = [6917529027641081856,
												6917529027641081856,
												6917529027641081856];
			test_imageset_hash(hash::HashType::PHash, hash::Precision::Medium, sample_03_images, sample_03_hashes);

			// Sample_04 tests
			let sample_04_images: [&Path; 3] = [&Path::new("./test_images/sample_04_large.jpg"),
												&Path::new("./test_images/sample_04_medium.jpg"),
												&Path::new("./test_images/sample_04_small.jpg")];
			let sample_04_hashes: [u64; 3] = [10997931646002397184,
												10997931646002397184,
												11142046834078253056];
			test_imageset_hash(hash::HashType::PHash, hash::Precision::Medium, sample_04_images, sample_04_hashes);

			// Clean_Cache
			// super::teardown();
		}
	}
