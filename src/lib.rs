// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use std::path::Path;

mod hash;

pub fn get_phashes(path: &Path) -> hash::PerceptualHashes {
    hash::get_perceptual_hashes(path, 8)
}

pub fn get_ahash(path: &Path) -> u64 {
    hash::get_ahash(&hash::prepare_image(path, 8))
}

pub fn get_dhash(path: &Path) -> u64 {
    hash::get_dhash(&hash::prepare_image(path, 8))
}

pub fn get_phash(path: &Path) -> u64 {
    hash::get_phash(&hash::prepare_image(path, 32))
}

pub fn get_hamming_distance(hash1: u64, hash2: u64) -> u64 {
    hash::calculate_hamming_distance(hash1, hash2)
}

/*
 * Module for the tests
 */
#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::path;
    use hash;

    #[test]
    fn can_get_test_images() {
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
                },
                _ => {
                    println!("Not an image: {:?}", orig_path) ;
                    continue
                }
            }
            //println!("Name: {}", path.unwrap().path().display())
        }
        // Currently 12 images in the test imaages directory
        assert!(num_paths == 12);
    }

    // Simple function for the unit tests to succinctly test a set of images
    // that are organized in the fashion of large->medium->small
    fn test_imageset_hash<F: Fn(&hash::PreparedImage) -> u64>(
        hash_func: F,
        resize_target: u32,
        large_path: &str,
        medium_path: &str,
        small_path: &str,
        expected_large_hash: u64,
        expected_medium_hash: u64,
        expected_small_hash: u64,
        expected_large_medium_hamming: u64,
        expected_large_small_hamming: u64,
        expected_medium_small_hamming: u64) {
        
        let large_prepared_image = hash::prepare_image(path::Path::new(large_path), resize_target);
        let medium_prepared_image = hash::prepare_image(path::Path::new(medium_path), resize_target);
        let small_prepared_image = hash::prepare_image(path::Path::new(small_path), resize_target);
        
        let actual_large_hash = hash_func(&large_prepared_image);
        let actual_medium_hash = hash_func(&medium_prepared_image);
        let actual_small_hash = hash_func(&small_prepared_image);

        // println for the purpose of debugging
        println!("{}: expected: {} actual: {}", large_path, expected_large_hash, actual_large_hash);
        println!("{}: expected: {} actual: {}", medium_path, expected_medium_hash, actual_medium_hash);
        println!("{}: expected: {} actual: {}", small_path, expected_small_hash, actual_small_hash);
        
        let actual_large_medium_hamming = hash::calculate_hamming_distance(actual_large_hash, actual_medium_hash);
        let actual_large_small_hamming = hash::calculate_hamming_distance(actual_large_hash, actual_small_hash);
        let actual_medium_small_hamming = hash::calculate_hamming_distance(actual_medium_hash, actual_small_hash);

        println!("Large-Medium Hamming Distance: expected: {} actual: {}", expected_large_medium_hamming, actual_large_medium_hamming);
        println!("Large-Small Hamming Distance: expected: {} actual: {}", expected_large_small_hamming, actual_large_small_hamming);
        println!("Medium-Small Hamming Distance: expected: {} actual: {}", expected_medium_small_hamming, actual_medium_small_hamming);
        
        // Doing that asserts
        assert!(actual_large_hash == expected_large_hash);
        assert!(actual_medium_hash == expected_medium_hash);
        assert!(actual_small_hash == expected_small_hash);

        assert!(actual_large_medium_hamming == expected_large_medium_hamming);
        assert!(actual_large_small_hamming == expected_large_small_hamming);
        assert!(actual_medium_small_hamming == expected_medium_small_hamming);

    }

    #[test]
    fn confirm_ahash_results() {
        // Sample_01 tests
        test_imageset_hash(
            hash::get_ahash,
            8u32,
            "./test_images/sample_01_large.jpg",
            "./test_images/sample_01_medium.jpg",
            "./test_images/sample_01_small.jpg",
            857051991849750,
            857051991849750,
            857051991849750,
            0u64,
            0u64,
            0u64
        );
        
        // Sample_02 tests
        test_imageset_hash(
            hash::get_ahash,
            8u32,
            "./test_images/sample_02_large.jpg",
            "./test_images/sample_02_medium.jpg",
            "./test_images/sample_02_small.jpg",
            18446744073441116160,
            18446744073441116160,
            18446744073441116160,
            0u64,
            0u64,
            0u64
        );
        // Sample_03 tests
        test_imageset_hash(
            hash::get_ahash,
            8u32,
            "./test_images/sample_03_large.jpg",
            "./test_images/sample_03_medium.jpg",
            "./test_images/sample_03_small.jpg",
            135670932300497406,
            135670932300497406,
            135670932300497406,
            0u64,
            0u64,
            0u64
        );
        
        // Sample_04 tests
        test_imageset_hash(
            hash::get_ahash,
            8u32,
            "./test_images/sample_04_large.jpg",
            "./test_images/sample_04_medium.jpg",
            "./test_images/sample_04_small.jpg",
            18446460933225054208,
            18446460933090836480,
            18446460933090836480,
            1u64,
            1u64,
            0u64
        );
    }

    #[test]
    fn confirm_dhash_results() {
        // Sample_01 tests
        test_imageset_hash(
            hash::get_dhash,
            8u32,
            "./test_images/sample_01_large.jpg",
            "./test_images/sample_01_medium.jpg",
            "./test_images/sample_01_small.jpg",
            7937395827556495926,
            7937395827556495926,
            7939647627370181174,
            0u64,
            1u64,
            1u64
        );
        
        // Sample_02 tests
        test_imageset_hash(
            hash::get_dhash,
            8u32,
            "./test_images/sample_02_large.jpg",
            "./test_images/sample_02_medium.jpg",
            "./test_images/sample_02_small.jpg",
            11009829669713008949,
            11009829670249879861,
            11009829669713008949,
            1u64,
            0u64,
            1u64
        );
        // Sample_03 tests
        test_imageset_hash(
            hash::get_dhash,
            8u32,
            "./test_images/sample_03_large.jpg",
            "./test_images/sample_03_medium.jpg",
            "./test_images/sample_03_small.jpg",
            225528496439353286,
            225528496439353286,
            226654396346195908,
            0u64,
            2u64,
            2u64
        );
        
        // Sample_04 tests
        test_imageset_hash(
            hash::get_dhash,
            8u32,
            "./test_images/sample_04_large.jpg",
            "./test_images/sample_04_medium.jpg",
            "./test_images/sample_04_small.jpg",
            14620651386429567209,
            14620651386429567209,
            14620651386429567209,
            0u64,
            0u64,
            0u64
        );
    }
    
    #[test]
    fn confirm_phash_results() {
        // Sample_01 tests
        test_imageset_hash(
            hash::get_phash,
            32u32,
            "./test_images/sample_01_large.jpg",
            "./test_images/sample_01_medium.jpg",
            "./test_images/sample_01_small.jpg",
            72357778504597504,
            72357778504597504,
            72357778504597504,
            0u64,
            0u64,
            0u64
        );
        
        // Sample_02 tests
        test_imageset_hash(
            hash::get_phash,
            32u32,
            "./test_images/sample_02_large.jpg",
            "./test_images/sample_02_medium.jpg",
            "./test_images/sample_02_small.jpg",
            5332332327550844928,
            5332332327550844928,
            5332332327550844928,
            0u64,
            0u64,
            0u64
        );
        // Sample_03 tests
        test_imageset_hash(
            hash::get_phash,
            32u32,
            "./test_images/sample_03_large.jpg",
            "./test_images/sample_03_medium.jpg",
            "./test_images/sample_03_small.jpg",
            6917529027641081856,
            6917529027641081856,
            6917529027641081856,
            0u64,
            0u64,
            0u64
        );
        
        // Sample_04 tests
        test_imageset_hash(
            hash::get_phash,
            32u32,
            "./test_images/sample_04_large.jpg",
            "./test_images/sample_04_medium.jpg",
            "./test_images/sample_04_small.jpg",
            10997931646002397184,
            10997931646002397184,
            11142046834078253056,
            0u64,
            1u64,
            1u64
        );
    }
}
