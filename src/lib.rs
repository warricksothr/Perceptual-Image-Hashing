// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

mod hash;
mod cache;

extern crate libc;

use std::path::Path;
use hash::PerceptualHash;
use std::ffi::CStr;

/**
 * Prepare the library for work.
 *
 * Not performing this step may cause parts to fail.
 */
#[no_mangle]
pub extern "C" fn init() {
    match cache::prep_cache() {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
}

/**
 * Teardown for the library
 */
#[no_mangle]
pub extern "C" fn teardown() {
    match cache::clear_cache() {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
}

pub fn get_phashes(path: &Path) -> hash::PerceptualHashes {
    hash::get_perceptual_hashes(path, &hash::Precision::Medium)
}


pub fn get_ahash(path: &Path) -> u64 {
    hash::AHash::new(&path, &hash::Precision::Medium).get_hash()
}

pub fn get_dhash(path: &Path) -> u64 {
    hash::DHash::new(&path, &hash::Precision::Medium).get_hash()
}

pub fn get_phash(path: &Path) -> u64 {
    hash::PHash::new(&path, &hash::Precision::Medium).get_hash()
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
    use std::path;
    use hash;

    #[test]
    fn test_can_get_test_images() {
        let paths = fs::read_dir(&path::Path::new("./test_images")).unwrap();
        let mut num_paths = 0;
        for path in paths {
            let orig_path = path.unwrap().path();
            let ext = path::Path::new(&orig_path).extension();
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

    // Simple function for the unit tests to succinctly test a set of images
    // that are organized in the fashion of large->medium->small
    fn test_imageset_hash(large_phash: &hash::PerceptualHash,
                          medium_phash: &hash::PerceptualHash,
                          small_phash: &hash::PerceptualHash,
                          expected_large_hash: u64,
                          expected_medium_hash: u64,
                          expected_small_hash: u64,
                          expected_large_medium_hamming: u64,
                          expected_large_small_hamming: u64,
                          expected_medium_small_hamming: u64) {

        let actual_large_hash = large_phash.get_hash();
        let actual_medium_hash = medium_phash.get_hash();
        let actual_small_hash = small_phash.get_hash();

        // println for the purpose of debugging
        println!("Large Image: expected: {} actual: {}",
                 expected_large_hash,
                 actual_large_hash);
        println!("Medium Image: expected: {} actual: {}",
                 expected_medium_hash,
                 actual_medium_hash);
        println!("Small Image: expected: {} actual: {}",
                 expected_small_hash,
                 actual_small_hash);

        let actual_large_medium_hamming = hash::calculate_hamming_distance(actual_large_hash,
                                                                           actual_medium_hash);
        let actual_large_small_hamming = hash::calculate_hamming_distance(actual_large_hash,
                                                                          actual_small_hash);
        let actual_medium_small_hamming = hash::calculate_hamming_distance(actual_medium_hash,
                                                                           actual_small_hash);

        println!("Large-Medium Hamming Distance: expected: {} actual: {}",
                 expected_large_medium_hamming,
                 actual_large_medium_hamming);
        println!("Large-Small Hamming Distance: expected: {} actual: {}",
                 expected_large_small_hamming,
                 actual_large_small_hamming);
        println!("Medium-Small Hamming Distance: expected: {} actual: {}",
                 expected_medium_small_hamming,
                 actual_medium_small_hamming);

        // Doing that asserts
        assert!(actual_large_hash == expected_large_hash);
        assert!(actual_medium_hash == expected_medium_hash);
        assert!(actual_small_hash == expected_small_hash);

        assert!(actual_large_medium_hamming == expected_large_medium_hamming);
        assert!(actual_large_small_hamming == expected_large_small_hamming);
        assert!(actual_medium_small_hamming == expected_medium_small_hamming);

    }

    #[test]
    fn test_confirm_ahash_results() {
        // Prep_Cache
        super::init();

        // Sample_01 tests
        test_imageset_hash(&hash::AHash::new(path::Path::new("./test_images/sample_01_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::AHash::new(path::Path::new("./test_images/sample_01_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::AHash::new(path::Path::new("./test_images/sample_01_small.jpg"),
                                             &hash::Precision::Medium),
                           857051991849750,
                           857051991849750,
                           857051991849750,
                           0u64,
                           0u64,
                           0u64);

        // Sample_02 tests
        test_imageset_hash(&hash::AHash::new(path::Path::new("./test_images/sample_02_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::AHash::new(path::Path::new("./test_images/sample_02_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::AHash::new(path::Path::new("./test_images/sample_02_small.jpg"),
                                             &hash::Precision::Medium),
                           18446744073441116160,
                           18446744073441116160,
                           18446744073441116160,
                           0u64,
                           0u64,
                           0u64);
        // Sample_03 tests
        test_imageset_hash(&hash::AHash::new(path::Path::new("./test_images/sample_03_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::AHash::new(path::Path::new("./test_images/sample_03_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::AHash::new(path::Path::new("./test_images/sample_03_small.jpg"),
                                             &hash::Precision::Medium),
                           135670932300497406,
                           135670932300497406,
                           135670932300497406,
                           0u64,
                           0u64,
                           0u64);

        // Sample_04 tests
        test_imageset_hash(&hash::AHash::new(path::Path::new("./test_images/sample_04_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::AHash::new(path::Path::new("./test_images/sample_04_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::AHash::new(path::Path::new("./test_images/sample_04_small.jpg"),
                                             &hash::Precision::Medium),
                           18446460933225054208,
                           18446460933090836480,
                           18446460933090836480,
                           1u64,
                           1u64,
                           0u64);

        // Clean_Cache
        // super::teardown();
    }

    #[test]
    fn test_confirm_dhash_results() {
        // Prep_Cache
        super::init();

        // Sample_01 tests
        test_imageset_hash(&hash::DHash::new(path::Path::new("./test_images/sample_01_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::DHash::new(path::Path::new("./test_images/sample_01_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::DHash::new(path::Path::new("./test_images/sample_01_small.jpg"),
                                             &hash::Precision::Medium),
                           7937395827556495926,
                           7937395827556495926,
                           7939647627370181174,
                           0u64,
                           1u64,
                           1u64);

        // Sample_02 tests
        test_imageset_hash(&hash::DHash::new(path::Path::new("./test_images/sample_02_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::DHash::new(path::Path::new("./test_images/sample_02_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::DHash::new(path::Path::new("./test_images/sample_02_small.jpg"),
                                             &hash::Precision::Medium),
                           11009829669713008949,
                           11009829670249879861,
                           11009829669713008949,
                           1u64,
                           0u64,
                           1u64);
        // Sample_03 tests
        test_imageset_hash(&hash::DHash::new(path::Path::new("./test_images/sample_03_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::DHash::new(path::Path::new("./test_images/sample_03_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::DHash::new(path::Path::new("./test_images/sample_03_small.jpg"),
                                             &hash::Precision::Medium),
                           225528496439353286,
                           225528496439353286,
                           226654396346195908,
                           0u64,
                           2u64,
                           2u64);

        // Sample_04 tests
        test_imageset_hash(&hash::DHash::new(path::Path::new("./test_images/sample_04_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::DHash::new(path::Path::new("./test_images/sample_04_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::DHash::new(path::Path::new("./test_images/sample_04_small.jpg"),
                                             &hash::Precision::Medium),
                           14620651386429567209,
                           14620651386429567209,
                           14620651386429567209,
                           0u64,
                           0u64,
                           0u64);

        // Clean_Cache
        // super::teardown();
    }

    #[test]
    fn test_confirm_phash_results() {
        // Prep_Cache
        super::init();

        // Sample_01 tests
        test_imageset_hash(&hash::PHash::new(path::Path::new("./test_images/sample_01_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::PHash::new(path::Path::new("./test_images/sample_01_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::PHash::new(path::Path::new("./test_images/sample_01_small.jpg"),
                                             &hash::Precision::Medium),
                           72357778504597504,
                           72357778504597504,
                           72357778504597504,
                           0u64,
                           0u64,
                           0u64);

        // Sample_02 tests
        test_imageset_hash(&hash::PHash::new(path::Path::new("./test_images/sample_02_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::PHash::new(path::Path::new("./test_images/sample_02_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::PHash::new(path::Path::new("./test_images/sample_02_small.jpg"),
                                             &hash::Precision::Medium),
                           5332332327550844928,
                           5332332327550844928,
                           5332332327550844928,
                           0u64,
                           0u64,
                           0u64);
        // Sample_03 tests
        test_imageset_hash(&hash::PHash::new(path::Path::new("./test_images/sample_03_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::PHash::new(path::Path::new("./test_images/sample_03_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::PHash::new(path::Path::new("./test_images/sample_03_small.jpg"),
                                             &hash::Precision::Medium),
                           6917529027641081856,
                           6917529027641081856,
                           6917529027641081856,
                           0u64,
                           0u64,
                           0u64);

        // Sample_04 tests
        test_imageset_hash(&hash::PHash::new(path::Path::new("./test_images/sample_04_large.jpg"),
                                             &hash::Precision::Medium),
                           &hash::PHash::new(path::Path::new("./test_images/sample_04_medium.\
                                                              jpg"),
                                             &hash::Precision::Medium),
                           &hash::PHash::new(path::Path::new("./test_images/sample_04_small.jpg"),
                                             &hash::Precision::Medium),
                           10997931646002397184,
                           10997931646002397184,
                           11142046834078253056,
                           0u64,
                           1u64,
                           1u64);

        // Clean_Cache
        // super::teardown();
    }
}
