// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.


mod hash;

pub fn hello(mut result: String) -> String {
    let helloworld = "Hello, World!\n";
    result.push_str(helloworld);
    let n = 1u8;
    let ns = format!("1: {:b}\n", n);
    let n2 = 2u8;
    let n2s = format!("2: {:b}\n", n2);
    result.push_str(&ns);
    result.push_str(&n2s);
    let mut endian = "Big Endian\n";
    if cfg!(target_endian = "big") {
        result.push_str(endian);
    } else {
        endian = "Little Endian\n";
        result.push_str(endian);
    }
    result
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
    fn test_image_set_ahash(
        large_path: &str,
        medium_path: &str,
        small_path: &str,
        expected_large_hash: u64,
        expected_medium_hash: u64,
        expected_small_hash: u64,
        expected_large_medium_hamming: u64,
        expected_large_small_hamming: u64,
        expected_medium_small_hamming: u64) {
        
        let large_prepared_image = hash::prepare_image(path::Path::new(large_path), 8u32);
        let medium_prepared_image = hash::prepare_image(path::Path::new(medium_path), 8u32);
        let small_prepared_image = hash::prepare_image(path::Path::new(small_path), 8u32);
        
        let large_ahash = hash::get_ahash(large_prepared_image);
        let medium_ahash = hash::get_ahash(medium_prepared_image);
        let small_ahash = hash::get_ahash(small_prepared_image);

        println!("./test_images/sample_01_large.jpg: {}", large_ahash);
        println!("./test_images/sample_01_medium.jpg: {}", medium_ahash);
        println!("./test_images/sample_01_small.jpg: {}", small_ahash);
        
        assert!(large_ahash == expected_large_hash);
        assert!(medium_ahash == expected_medium_hash);
        assert!(small_ahash == expected_small_hash);

        let large_medium_hamming = hash::calculate_hamming_distance(large_ahash, medium_ahash);
        let large_small_hamming = hash::calculate_hamming_distance(large_ahash, small_ahash);
        let medium_small_hamming = hash::calculate_hamming_distance(medium_ahash, small_ahash);

        println!("Large-Medium Hamming Distance: {}", large_medium_hamming);
        println!("Large-Small Hamming Distance: {}", large_small_hamming);
        println!("Medium-Small Hamming Distance: {}", medium_small_hamming);

        assert!(large_medium_hamming == expected_large_medium_hamming);
        assert!(large_small_hamming == expected_large_small_hamming);
        assert!(medium_small_hamming == expected_medium_small_hamming);

    }

    #[test]
    fn confirm_ahash_results() {
        // Sample_01 tests
        test_image_set_ahash(
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
        test_image_set_ahash(
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
        test_image_set_ahash(
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
        test_image_set_ahash(
            "./test_images/sample_04_large.jpg",
            "./test_images/sample_04_medium.jpg",
            "./test_images/sample_04_small.jpg",
            18446460933225054208,
            18446460933225054208,
            18446460933225054208,
            0u64,
            0u64,
            0u64
        );
    }

}
