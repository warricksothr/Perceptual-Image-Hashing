// Copyright 2016 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate dft;
extern crate image;

use std::f64;
use std::fmt;
use std::fmt::{Error, Formatter};
use std::path::Path;

use serde::export::fmt::Debug;

use cache::Cache;

use self::image::FilterType;

mod ahash;
mod dhash;
mod phash;

// Constants //

// Used to get ranges for the precision of rounding floats
// Can round to 1 significant factor of precision
const FLOAT_PRECISION_MAX_1: f64 = f64::MAX / 10_f64;
const FLOAT_PRECISION_MIN_1: f64 = f64::MIN / 10_f64;
// Can round to 2 significant factors of precision
const FLOAT_PRECISION_MAX_2: f64 = f64::MAX / 100_f64;
const FLOAT_PRECISION_MIN_2: f64 = f64::MIN / 100_f64;
// Can round to 3 significant factors of precision
const FLOAT_PRECISION_MAX_3: f64 = f64::MAX / 1000_f64;
const FLOAT_PRECISION_MIN_3: f64 = f64::MIN / 1000_f64;
// Can round to 4 significant factors of precision
const FLOAT_PRECISION_MAX_4: f64 = f64::MAX / 10000_f64;
const FLOAT_PRECISION_MIN_4: f64 = f64::MIN / 10000_f64;
// Can round to 5 significant factors of precision
const FLOAT_PRECISION_MAX_5: f64 = f64::MAX / 100000_f64;
const FLOAT_PRECISION_MIN_5: f64 = f64::MIN / 100000_f64;
// Hamming Distance Similarity Limit //
const HAMMING_DISTANCE_SIMILARITY_LIMIT: u64 = 5u64;

// Structs/Enums //

/**
 * Prepared image that can be used to generate hashes
 */
pub struct PreparedImage {
    orig_path: String,
    image: Option<image::DynamicImage>,
}

/**
 * Wraps the various perceptual hashes
 */
#[derive(Debug)]
pub struct PerceptualHashes {
    pub orig_path: String,
    pub ahash: u64,
    pub dhash: u64,
    pub phash: u64
}

impl PartialEq for PerceptualHashes {
    fn eq(&self, other: &Self) -> bool {
        return self.ahash == other.ahash
            && self.dhash == other.dhash
            && self.phash == other.phash;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.ahash != other.ahash
            || self.dhash != other.dhash
            || self.phash != other.phash;
    }
}

impl PerceptualHashes {
    pub fn similar(&self, other: &PerceptualHashes) -> bool {
        if self.orig_path != other.orig_path
            && calculate_hamming_distance(self.ahash, other.ahash)
            <= HAMMING_DISTANCE_SIMILARITY_LIMIT
            && calculate_hamming_distance(self.dhash, other.dhash)
            <= HAMMING_DISTANCE_SIMILARITY_LIMIT
            && calculate_hamming_distance(self.phash, other.phash)
            <= HAMMING_DISTANCE_SIMILARITY_LIMIT
        {
            true
        } else {
            false
        }
    }
}

/**
 * All the supported precision types
 *
 * Low aims for 32 bit precision
 * Medium aims for 64 bit precision
 * High aims for 128 bit precision
 */
#[derive(Copy, Clone)]
pub enum Precision {
    Low,
    Medium,
    High,
}

// Get the size of the required image
impl Precision {
    fn get_size(&self) -> u32 {
        match *self {
            Precision::Low => 4,
            Precision::Medium => 8,
            Precision::High => 16,
        }
    }
}

/**
 * Types of hashes supported
 */
#[derive(Copy, Clone)]
pub enum HashType {
    AHash,
    DHash,
    PHash,
}

impl fmt::Display for HashType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HashType::AHash => write!(f, "AHash"),
            HashType::DHash => write!(f, "DHash"),
            HashType::PHash => write!(f, "PHash"),
        }
    }
}

// Traits //

pub trait PerceptualHash {
    fn get_hash(&self, cache: &Option<Cache>) -> u64;
}

// Functions //

/**
 * Responsible for parsing a path, converting an image and package it to be
 * hashed.
 *
 * # Arguments
 *
 * * 'path' - The path to the image requested to be hashed
 * * 'size' - The size that the image should be resize to, in the form of size x size
 *
 * # Returns
 *
 * A PreparedImage struct with the required information for performing hashing
 *
 */
pub fn prepare_image(
    path: &Path,
    hash_type: &HashType,
    precision: &Precision,
    cache: &Option<Cache>,
) -> PreparedImage {
    let image_path = path.to_str().unwrap();
    let size: u32 = match *hash_type {
        HashType::PHash => precision.get_size() * 4,
        _ => precision.get_size(),
    };
    // Check if we have the already converted image in a cache and use that if possible.
    match *cache {
        Some(ref cache) => {
            match cache.get_image_from_cache(&path, size) {
                Some(image) => PreparedImage {
                    orig_path: String::from(&*image_path),
                    image: Some(image),
                },
                None => {
                    let processed_image = process_image(&image_path, size);
                    // Oh, and save it in a cache
                    match processed_image.image {
                        Some(ref image) => {
                            match cache.put_image_in_cache(&path, size, &image) {
                                Ok(_) => {}
                                Err(e) => println!("Unable to store image in cache. {}", e),
                            };
                        }
                        None => {}
                    };
                    processed_image
                }
            }
        }
        None => process_image(&image_path, size),
    }
}

/**
 * Turn the image into something we can work with
 */
fn process_image(image_path: &str, size: u32) -> PreparedImage {
    // Otherwise let's do that work now and store it.
    // println!("Path: {}", image_path);
    let image = match image::open(Path::new(image_path)) {
        Ok(image) => {
            let small_image = image.resize_exact(size, size, FilterType::Lanczos3);
            Some(small_image.grayscale())
        }
        Err(e) => {
            println!("Error Processing Image [{}]: {} ", image_path, e);
            None
        }
    };
    PreparedImage {
        orig_path: String::from(&*image_path),
        image,
    }
}

/**
 * Get a specific HashType hash
 */
pub fn get_perceptual_hash(
    path: &Path,
    precision: &Precision,
    hash_type: &HashType,
    cache: &Option<Cache>,
) -> u64 {
    match *hash_type {
        HashType::AHash => ahash::AHash::new(&path, &precision, &cache).get_hash(&cache),
        HashType::DHash => dhash::DHash::new(&path, &precision, &cache).get_hash(&cache),
        HashType::PHash => phash::PHash::new(&path, &precision, &cache).get_hash(&cache),
    }
}

/**
 * Get all perceptual hashes for an image
 */
pub fn get_perceptual_hashes(
    path: &Path,
    precision: &Precision,
    cache: &Option<Cache>,
) -> PerceptualHashes {
    let image_path = path.to_str().unwrap();
    let ahash = ahash::AHash::new(&path, &precision, &cache).get_hash(&cache);
    let dhash = dhash::DHash::new(&path, &precision, &cache).get_hash(&cache);
    let phash = phash::PHash::new(&path, &precision, &cache).get_hash(&cache);
    PerceptualHashes {
        orig_path: String::from(&*image_path),
        ahash,
        dhash,
        phash,
    }
}

/**
 * Calculate the number of bits different between two hashes
 * Add to the PerceptualHashTrait
 */
pub fn calculate_hamming_distance(hash1: u64, hash2: u64) -> u64 {
    // The binary xor of the two hashes should give us a number representing
    // the differences between the two hashes. All that's left is to count
    // the number of 1's in the difference to determine the hamming distance
    (hash1 ^ hash2).count_ones() as u64
}

#[cfg(test)]
mod tests {
    use hash::calculate_hamming_distance;

    #[test]
    fn test_no_hamming_distance() {
        let hamming_distance = calculate_hamming_distance(0, 0);
        assert_eq!(hamming_distance, 0);
    }

    #[test]
    fn test_one_hamming_distance() {
        let hamming_distance = calculate_hamming_distance(0, 1);
        assert_eq!(hamming_distance, 1);
    }

    #[test]
    fn test_two_hamming_distance() {
        let hamming_distance = calculate_hamming_distance(0, 3);
        assert_eq!(hamming_distance, 2);
    }
}
