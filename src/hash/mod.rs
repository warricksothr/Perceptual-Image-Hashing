// Copyright 2016 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate dft;
extern crate image;

mod ahash;
mod dhash;
mod phash;

use std::path::Path;
use std::f64;
use self::image::{Pixel, FilterType};
use cache::Cache;

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

// Structs/Enums //

/**
 * Prepared image that can be used to generate hashes
 */
pub struct PreparedImage<'a> {
    orig_path: &'a str,
    image: image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
    cache: &'a Cache<'a>,
}

/**
 * Wraps the various perceptual hashes
 */
pub struct PerceptualHashes<'a> {
    pub orig_path: &'a str,
    pub ahash: u64,
    pub dhash: u64,
    pub phash: u64,
}

/**
 * All the supported precision types
 *
 * Low aims for 32 bit precision
 * Medium aims for 64 bit precision
 * High aims for 128 bit precision
 */
#[allow(dead_code)]
pub enum Precision {
    Low,
    Medium,
    High,
}

// Get the size of the required image
//
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
pub enum HashType {
    AHash,
    DHash,
    PHash,
}

// Traits //

pub trait PerceptualHash {
    fn get_hash(&self) -> u64;
}

// Functions //

/**
 * Resonsible for parsing a path, converting an image and package it to be
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
pub fn prepare_image<'a>(path: &'a Path,
                         hash_type: &HashType,
                         precision: &Precision,
                         cache: &'a Cache<'a>)
                         -> PreparedImage<'a> {
    let image_path = path.to_str().unwrap();
    let size: u32 = match *hash_type {
        HashType::PHash => precision.get_size() * 4,
        _ => precision.get_size(),
    };
    // Check if we have the already converted image in a cache and use that if possible.
    match cache.get_image_from_cache(&path, size) {
        Some(image) => {
            PreparedImage {
                orig_path: &*image_path,
                image: image,
                cache: &cache
            }
        }
        None => {
            // Otherwise let's do that work now and store it.
            let image = image::open(path).unwrap();
            let small_image = image.resize_exact(size, size, FilterType::Lanczos3);
            let grey_image = small_image.to_luma();
            match cache.put_image_in_cache(&path, size, &grey_image) {
                Ok(_) => {}
                Err(e) => println!("Unable to store image in cache. {}", e),
            };
            PreparedImage {
                orig_path: &*image_path,
                image: grey_image,
                cache: &cache,
            }
        }
    }
}

/**
 * Get a specific HashType hash
 */
pub fn get_perceptual_hash<'a>(path: &'a Path, precision: &Precision, hash_type: &HashType, cache: &Cache) -> u64 {
    match *hash_type {
        HashType::AHash => ahash::AHash::new(&path, &precision, &cache).get_hash(),
        HashType::DHash => dhash::DHash::new(&path, &precision, &cache).get_hash(),
        HashType::PHash => phash::PHash::new(&path, &precision, &cache).get_hash()
    }
}

/**
 * Get all perceptual hashes for an image
 */
pub fn get_perceptual_hashes<'a>(path: &'a Path, precision: &Precision, cache: &Cache) -> PerceptualHashes<'a> {
    let image_path = path.to_str().unwrap();
    let ahash = ahash::AHash::new(&path, &precision, &cache).get_hash();
    let dhash = dhash::DHash::new(&path, &precision, &cache).get_hash();
    let phash = phash::PHash::new(&path, &precision, &cache).get_hash();
    PerceptualHashes {
        orig_path: &*image_path,
        ahash: ahash,
        dhash: dhash,
        phash: phash,
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
    let bin_diff = hash1 ^ hash2;
    let bin_diff_str = format!("{:b}", bin_diff);
    let mut hamming = 0u64;
    for bit in bin_diff_str.chars() {
        match bit {
            '1' => hamming += 1,
            _ => continue,
        }
    }
    hamming
}