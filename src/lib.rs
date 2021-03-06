// Copyright 2016 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

// Enable nightly features for extra testing behind the bench feature
#![cfg_attr(feature = "bench", feature(test))]

extern crate libc;
extern crate rustc_serialize;
extern crate serde;
#[cfg(feature = "bench")]
extern crate test;

use std::ffi::CStr;
use std::path::Path;

use cache::Cache;

pub mod cache;
pub mod hash;

#[repr(C)]
pub struct PIHash {
    cache: Option<Cache>,
}

impl PIHash {
    /**
     * Create a new pihash library, and initialize a cache of a path is passed.
     * If none is passed then no cache is initialized or used with the library
     */
    pub fn new(cache_path: Option<&str>) -> PIHash {
        match cache_path {
            Some(path) => {
                let cache = Cache {
                    cache_dir: String::from(path),
                    use_cache: true,
                };
                match cache.init() {
                    Ok(_) => PIHash { cache: Some(cache) },
                    Err(e) => {
                        println!("Error creating library with cache: {}", e);
                        PIHash { cache: None }
                    }
                }
            }
            None => PIHash { cache: None },
        }
    }

    pub fn get_perceptual_hash(
        &self,
        path: &Path,
        precision: &hash::Precision,
        hash_type: &hash::HashType,
    ) -> u64 {
        hash::get_perceptual_hash(&path, &precision, &hash_type, &self.cache)
    }

    pub fn get_pihashes(&self, path: &Path) -> hash::PerceptualHashes {
        hash::get_perceptual_hashes(&path, &hash::Precision::Medium, &self.cache)
    }

    pub fn get_ahash(&self, path: &Path) -> u64 {
        hash::get_perceptual_hash(
            &path,
            &hash::Precision::Medium,
            &hash::HashType::AHash,
            &self.cache,
        )
    }

    pub fn get_dhash(&self, path: &Path) -> u64 {
        hash::get_perceptual_hash(
            &path,
            &hash::Precision::Medium,
            &hash::HashType::DHash,
            &self.cache,
        )
    }

    pub fn get_phash(&self, path: &Path) -> u64 {
        hash::get_perceptual_hash(
            &path,
            &hash::Precision::Medium,
            &hash::HashType::PHash,
            &self.cache,
        )
    }
}

/**
 * Get the Hamming Distance between two hashes.
 * Represents the absolute difference between two numbers.
 */
pub fn get_hamming_distance(hash1: u64, hash2: u64) -> u64 {
    hash::calculate_hamming_distance(hash1, hash2)
}

// External proxies for the get_*hash methods //

#[no_mangle]
pub extern "C" fn ext_init(cache_path_char: *const libc::c_char) -> *const libc::c_void {
    unsafe {
        let path_cstr = CStr::from_ptr(cache_path_char);
        let path_str = match path_cstr.to_str() {
            Ok(path) => Some(path),
            Err(_) => None,
        };
        // println!("Created new lib, with cache at {}", path_str.unwrap());
        let lib = Box::new(PIHash::new(path_str));
        let ptr = Box::into_raw(lib) as *mut libc::c_void;
        ptr
    }
}

#[no_mangle]
pub extern "C" fn ext_free(raw_lib: *const libc::c_void) {
    unsafe {
        drop(Box::from_raw(raw_lib as *mut PIHash));
    }
}

#[no_mangle]
pub extern "C" fn ext_get_ahash(lib: &PIHash, path_char: *const libc::c_char) -> u64 {
    unsafe {
        let path_str = CStr::from_ptr(path_char);
        let image_path = get_str_from_cstr(path_str);
        let path = Path::new(&image_path);
        lib.get_ahash(path)
    }
}

#[no_mangle]
pub extern "C" fn ext_get_dhash(lib: &PIHash, path_char: *const libc::c_char) -> u64 {
    unsafe {
        let path_str = CStr::from_ptr(path_char);
        let image_path = get_str_from_cstr(path_str);
        let path = Path::new(&image_path);
        lib.get_dhash(path)
    }
}

#[no_mangle]
pub extern "C" fn ext_get_phash(lib: &PIHash, path_char: *const libc::c_char) -> u64 {
    unsafe {
        let path_str = CStr::from_ptr(path_char);
        let image_path = get_str_from_cstr(path_str);
        let path = Path::new(&image_path);
        lib.get_phash(path)
    }
}

#[repr(C)]
pub struct PIHashes {
    ahash: u64,
    dhash: u64,
    phash: u64,
}

#[no_mangle]
pub extern "C" fn ext_get_pihashes(lib: &PIHash, path_char: *const libc::c_char) -> *mut PIHashes {
    unsafe {
        let path_str = CStr::from_ptr(path_char);
        let image_path = get_str_from_cstr(path_str);
        let path = Path::new(&image_path);
        let pihashes = lib.get_pihashes(path);
        Box::into_raw(Box::new(PIHashes {
            ahash: pihashes.ahash,
            dhash: pihashes.dhash,
            phash: pihashes.phash,
        }))
    }
}

#[no_mangle]
pub extern "C" fn ext_free_pihashes(raw_pihashes: *const libc::c_void) {
    unsafe {
        drop(Box::from_raw(raw_pihashes as *mut PIHashes));
    }
}

fn get_str_from_cstr(path_str: &CStr) -> &str {
    match path_str.to_str() {
        Ok(result) => result,
        Err(e) => {
            println!(
                "Error: {}. Unable to parse '{}'",
                e,
                to_hex_string(path_str.to_bytes())
            );
            panic!("Unable to parse path")
        }
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
    use std::fs;
    use std::path::Path;

    use cache;
    use hash;
    use hash::{PerceptualHash, PerceptualHashes};

    use super::PIHash;
    #[cfg(feature = "bench")]
    use super::test::Bencher;

    thread_local!(static LIB: PIHash = PIHash::new(Some(cache::DEFAULT_CACHE_DIR)));
    thread_local!(static NO_CACHE_LIB: PIHash = PIHash::new(None));

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
                        println!("Is a image {}: {:?}", num_paths, orig_path);
                    }
                }
                _ => {
                    println!("Not an image: {:?}", orig_path);
                    continue;
                }
            }
            // println!("Name: {}", path.unwrap().path().display())
        }
        // Currently 12 images in the test images directory
        assert_eq!(num_paths, 12);
    }

    /**
     * Updated test function. Assumes 3 images to a set and no hamming distances.
     * We don't need to confirm that the hamming distance calculation works in these tests.
     */
    fn test_image_set_hash(
        hash_type: hash::HashType,
        hash_precision: hash::Precision,
        max_hamming_distance: u64,
        image_paths: [&Path; 3],
        image_hashes: [u64; 3],
        lib: &PIHash,
    ) {
        let mut hashes: [u64; 3] = [0; 3];
        for index in 0..image_paths.len() {
            //            println!("{}, {:?}", index, image_paths[index]);
            let image_path = image_paths[index];
            let calculated_hash = lib.get_perceptual_hash(&image_path, &hash_precision, &hash_type);
            println!(
                "[{}] Image hashes for [{}] expected: [{}] actual: [{}]",
                hash_type,
                image_path.to_str().unwrap(),
                image_hashes[index],
                calculated_hash
            );
            hashes[index] = calculated_hash;
        }
        assert_eq!(hashes, image_hashes);

        for index in 0..hashes.len() {
            for index2 in 0..hashes.len() {
                if index == index2 {
                    continue;
                } else {
                    let distance = hash::calculate_hamming_distance(hashes[index], hashes[index2]);
                    println!("Hashes [{}] and [{}] have a hamming distance of [{}] of a max allowed distance of [{}]",
                             hashes[index],
                             hashes[index2],
                             distance,
                             max_hamming_distance);
                    assert!(distance <= max_hamming_distance);
                }
            }
        }
    }

    /**
     * Test image set with and without caching
     */
    fn test_image_set(
        hash_type: hash::HashType,
        hash_precision: hash::Precision,
        max_hamming_distance: u64,
        image_paths: [&Path; 3],
        image_hashes: [u64; 3],
    ) {
        LIB.with(|lib| {
            test_image_set_hash(
                hash_type,
                hash_precision,
                max_hamming_distance,
                image_paths,
                image_hashes,
                lib,
            );
        });
        NO_CACHE_LIB.with(|lib| {
            test_image_set_hash(
                hash_type,
                hash_precision,
                max_hamming_distance,
                image_paths,
                image_hashes,
                lib,
            );
        });
    }

    /**
     * Updated test function. Assumes 3 images to a set and no hamming distances.
     * We don't need to confirm that the hamming distance calculation works in these tests.
     */
    fn test_images_hashes(
        image_hashes: &[PerceptualHashes],
        lib: &PIHash,
    ) {
        let mut hashes = vec![];
        for index in 0..image_hashes.len() {
//            println!("{}, {:?}", index, image_paths[index]);
            let image_path = Path::new(&image_hashes[index].orig_path);
            let calculated_hash = lib.get_pihashes(&image_path);
            println!(
                "Image hashes expected: [{:?}] actual: [{:?}]",
                image_hashes[index],
                calculated_hash
            );
            hashes.push(calculated_hash);
        }
        for index in 0..image_hashes.len() {
            assert_eq!(hashes[index], image_hashes[index]);
        }
//
//        for index in 0..hashes.len() {
//            for index2 in 0..hashes.len() {
//                if index == index2 {
//                    continue;
//                } else {
//                    let distance = hash::calculate_hamming_distance(hashes[index], hashes[index2]);
//                    println!("Hashes [{}] and [{}] have a hamming distance of [{}] of a max allowed distance of [{}]",
//                             hashes[index],
//                             hashes[index2],
//                             distance,
//                             max_hamming_distance);
//                    assert!(distance <= max_hamming_distance);
//                }
//            }
//        }
    }

    /**
     * Test images with and without caching
     */
    fn test_images(image_hashes: &[PerceptualHashes]) {
        LIB.with(|lib| {
            test_images_hashes(
                &image_hashes,
                lib,
            );
        });
        NO_CACHE_LIB.with(|lib| {
            test_images_hashes(
                &image_hashes,
                lib,
            );
        });
    }

    #[test]
    fn test_confirm_ahash_results_sample_01() {
        let sample_01_images: [&Path; 3] = [
            &Path::new("./test_images/sample_01_large.jpg"),
            &Path::new("./test_images/sample_01_medium.jpg"),
            &Path::new("./test_images/sample_01_small.jpg"),
        ];
        let sample_01_hashes: [u64; 3] = [857051991849750, 857051991849750, 857051991849750];
        test_image_set(
            hash::HashType::AHash,
            hash::Precision::Medium,
            0u64,
            sample_01_images,
            sample_01_hashes,
        );
    }

    #[test]
    fn test_confirm_ahash_results_sample_02() {
        let sample_02_images: [&Path; 3] = [
            &Path::new("./test_images/sample_02_large.jpg"),
            &Path::new("./test_images/sample_02_medium.jpg"),
            &Path::new("./test_images/sample_02_small.jpg"),
        ];
        let sample_02_hashes: [u64; 3] = [
            18446744073441116160,
            18446744073441116160,
            18446744073441116160,
        ];
        test_image_set(
            hash::HashType::AHash,
            hash::Precision::Medium,
            0u64,
            sample_02_images,
            sample_02_hashes,
        );
    }

    #[test]
    fn test_confirm_ahash_results_sample_03() {
        let sample_03_images: [&Path; 3] = [
            &Path::new("./test_images/sample_03_large.jpg"),
            &Path::new("./test_images/sample_03_medium.jpg"),
            &Path::new("./test_images/sample_03_small.jpg"),
        ];
        let sample_03_hashes: [u64; 3] =
            [135670932300497406, 135670932300497406, 135670932300497406];
        test_image_set(
            hash::HashType::AHash,
            hash::Precision::Medium,
            0u64,
            sample_03_images,
            sample_03_hashes,
        );
    }

    #[test]
    fn test_confirm_ahash_results_sample_04() {
        let sample_04_images: [&Path; 3] = [
            &Path::new("./test_images/sample_04_large.jpg"),
            &Path::new("./test_images/sample_04_medium.jpg"),
            &Path::new("./test_images/sample_04_small.jpg"),
        ];
        let sample_04_hashes: [u64; 3] = [
            18446460933225054208,
            18446460933225054208,
            18446460933225054208,
        ];
        LIB.with(|lib| {
            test_image_set_hash(
                hash::HashType::AHash,
                hash::Precision::Medium,
                0u64,
                sample_04_images,
                sample_04_hashes,
                lib,
            );
        });
    }

    #[test]
    fn test_confirm_dhash_results_sample_01() {
        let sample_01_images: [&Path; 3] = [
            &Path::new("./test_images/sample_01_large.jpg"),
            &Path::new("./test_images/sample_01_medium.jpg"),
            &Path::new("./test_images/sample_01_small.jpg"),
        ];
        let sample_01_hashes: [u64; 3] = [
            3404580580803739582,
            3404580580803739582,
            3404580580803739582,
        ];
        LIB.with(|lib| {
            test_image_set_hash(
                hash::HashType::DHash,
                hash::Precision::Medium,
                0u64,
                sample_01_images,
                sample_01_hashes,
                lib,
            );
        });
    }

    #[test]
    fn test_confirm_dhash_results_sample_02() {
        let sample_02_images: [&Path; 3] = [
            &Path::new("./test_images/sample_02_large.jpg"),
            &Path::new("./test_images/sample_02_medium.jpg"),
            &Path::new("./test_images/sample_02_small.jpg"),
        ];
        let sample_02_hashes: [u64; 3] = [
            14726771606135242753,
            14726771606135242753,
            14726771606135242753,
        ];
        LIB.with(|lib| {
            test_image_set_hash(
                hash::HashType::DHash,
                hash::Precision::Medium,
                0u64,
                sample_02_images,
                sample_02_hashes,
                lib,
            );
        });
    }

    #[test]
    fn test_confirm_dhash_results_sample_03() {
        let sample_03_images: [&Path; 3] = [
            &Path::new("./test_images/sample_03_large.jpg"),
            &Path::new("./test_images/sample_03_medium.jpg"),
            &Path::new("./test_images/sample_03_small.jpg"),
        ];
        let sample_03_hashes: [u64; 3] =
            [144115181601817086, 144115181601817086, 144115181601817086];
        LIB.with(|lib| {
            test_image_set_hash(
                hash::HashType::DHash,
                hash::Precision::Medium,
                0u64,
                sample_03_images,
                sample_03_hashes,
                lib,
            );
        });
    }

    #[test]
    fn test_confirm_dhash_results_sample_04() {
        let sample_04_images: [&Path; 3] = [
            &Path::new("./test_images/sample_04_large.jpg"),
            &Path::new("./test_images/sample_04_medium.jpg"),
            &Path::new("./test_images/sample_04_small.jpg"),
        ];
        let sample_04_hashes: [u64; 3] = [
            18374262188442386433,
            18374262188442386433,
            18374262188442386433,
        ];
        LIB.with(|lib| {
            test_image_set_hash(
                hash::HashType::DHash,
                hash::Precision::Medium,
                0u64,
                sample_04_images,
                sample_04_hashes,
                lib,
            );
        });
    }

    #[test]
    fn test_confirm_phash_results_sample_01() {
        let sample_01_images: [&Path; 3] = [
            &Path::new("./test_images/sample_01_large.jpg"),
            &Path::new("./test_images/sample_01_medium.jpg"),
            &Path::new("./test_images/sample_01_small.jpg"),
        ];
        let sample_01_hashes: [u64; 3] = [72357778504597504, 72357778504597504, 72357778504597504];
        test_image_set(
            hash::HashType::PHash,
            hash::Precision::Medium,
            0u64,
            sample_01_images,
            sample_01_hashes,
        );
    }

    #[test]
    fn test_confirm_phash_results_sample_02() {
        let sample_02_images: [&Path; 3] = [
            &Path::new("./test_images/sample_02_large.jpg"),
            &Path::new("./test_images/sample_02_medium.jpg"),
            &Path::new("./test_images/sample_02_small.jpg"),
        ];
        let sample_02_hashes: [u64; 3] = [
            5332332327550844928,
            5332332327550844928,
            5332332327550844928,
        ];
        test_image_set(
            hash::HashType::PHash,
            hash::Precision::Medium,
            0u64,
            sample_02_images,
            sample_02_hashes,
        );
    }

    #[test]
    fn test_confirm_phash_results_sample_03() {
        let sample_03_images: [&Path; 3] = [
            &Path::new("./test_images/sample_03_large.jpg"),
            &Path::new("./test_images/sample_03_medium.jpg"),
            &Path::new("./test_images/sample_03_small.jpg"),
        ];
        let sample_03_hashes: [u64; 3] = [
            6917529027641081856,
            6917529027641081856,
            6917529027641081856,
        ];
        test_image_set(
            hash::HashType::PHash,
            hash::Precision::Medium,
            0u64,
            sample_03_images,
            sample_03_hashes,
        );
    }

    #[test]
    fn test_confirm_phash_results_sample_04() {
        let sample_04_images: [&Path; 3] = [
            &Path::new("./test_images/sample_04_large.jpg"),
            &Path::new("./test_images/sample_04_medium.jpg"),
            &Path::new("./test_images/sample_04_small.jpg"),
        ];
        let sample_04_hashes: [u64; 3] = [
            10997931646002397184,
            10997931646002397184,
            10997931646002397184,
        ];
        test_image_set(
            hash::HashType::PHash,
            hash::Precision::Medium,
            0u64,
            sample_04_images,
            sample_04_hashes,
        );
    }

    #[test]
    fn test_confirm_pihash_results() {
        let sample_hashes: [PerceptualHashes; 4] = [
            PerceptualHashes {
                orig_path: "./test_images/sample_01_large.jpg".to_string(),
                ahash: 857051991849750,
                dhash: 3404580580803739582,
                phash: 72357778504597504,
            },
            PerceptualHashes {
                orig_path: "./test_images/sample_02_large.jpg".to_string(),
                ahash: 18446744073441116160,
                dhash: 14726771606135242753,
                phash: 5332332327550844928,
            },
            PerceptualHashes {
                orig_path: "./test_images/sample_03_large.jpg".to_string(),
                ahash: 135670932300497406,
                dhash: 144115181601817086,
                phash: 6917529027641081856,
            },
            PerceptualHashes {
                orig_path: "./test_images/sample_04_large.jpg".to_string(),
                ahash: 18446460933225054208,
                dhash: 18374262188442386433,
                phash: 10997931646002397184,
            }
        ];
        test_images(&sample_hashes);
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_with_cache(bench: &mut Bencher) -> () {
        // Prep_library
        let lib = PIHash::new(Some(cache::DEFAULT_CACHE_DIR));

        // Setup the caches to make sure we're good to properly bench
        // All pihashes so that the matrices are pulled from cache as well
        lib.get_perceptual_hash(
            &Path::new("./test_images/sample_01_large.jpg"),
            &hash::Precision::Medium,
            &hash::HashType::PHash,
        );

        bench.iter(|| {
            lib.get_perceptual_hash(
                &Path::new("./test_images/sample_01_large.jpg"),
                &hash::Precision::Medium,
                &hash::HashType::PHash,
            );
        })
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_without_cache(bench: &mut Bencher) -> () {
        // Prep_library
        let lib = PIHash::new(None);

        bench.iter(|| {
            lib.get_perceptual_hash(
                &Path::new("./test_images/sample_01_large.jpg"),
                &hash::Precision::Medium,
                &hash::HashType::PHash,
            );
        })
    }
}
