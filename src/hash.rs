// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

// Pull in the image processing crate
extern crate image;

use std::path::Path;
use self::image::{
    GenericImage,
    Pixel,
    FilterType
};

/**
 * Prepared image that can be used to generate hashes
 */
pub struct PreparedImage<'a> {
    orig_path: &'a str,
    image: image::ImageBuffer<image::Luma<u8>,Vec<u8>>
}

/**
 * Wraps the various perceptual hashes
 */
pub struct PerceptualHashes<'a> {
    orig_path: &'a str,
    ahash: u64,
    dhash: u64,
    phash: u64
}

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
pub fn prepare_image(path: &Path, size: u32) -> PreparedImage {
    let image_path = path.to_str().unwrap();
    let image = image::open(path).unwrap();
    let small_image = image.resize_exact(size, size, FilterType::Lanczos3);
    let grey_image = small_image.to_luma();
    PreparedImage { orig_path: &*image_path, image: grey_image }
}

/**
 * Get all perceptual hashes for an image
 */
pub fn get_perceptual_hashes(path: &Path, size: u32, phash_size: u32) -> PerceptualHashes {
    let image_path = path.to_str().unwrap();
    let prepared_image = prepare_image(path, size);
    let phash_prepared_image = prepare_image(path, phash_size);
    let ahash = get_ahash(&prepared_image);
    let dhash = get_dhash(&prepared_image);
    let phash = get_phash(&phash_prepared_image);
    PerceptualHashes { orig_path: &*image_path, ahash: ahash, dhash: dhash, phash: phash }
}

/**
 * Calculate the number of bits different between two hashes
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
            '1' => hamming+=1,
            _ => continue
        }
    }
    hamming
}

/**
 * Calculate the ahash of the provided prepared image.
 *
 * # Arguments
 *
 * * 'prepared_image' - The already prepared image for perceptual processing.
 *
 * # Returns 
 *
 * A u64 representing the value of the hash
 */
pub fn get_ahash(prepared_image: &PreparedImage) -> u64 {
    let (width, height) = prepared_image.image.dimensions();

    // calculating the average pixel value
    let mut total = 0u64;
    for pixel in prepared_image.image.pixels() {
        let channels = pixel.channels();
        //println!("Pixel is: {}", channels[0]);
        total += channels[0] as u64;
    }
    let mean = total / (width*height) as u64;
    //println!("Mean for {} is {}", prepared_image.orig_path, mean);

    // Calculating a hash based on the mean
    let mut hash = 0u64;
    for pixel in prepared_image.image.pixels() {
        let channels = pixel.channels();
        let pixel_sum = channels[0] as u64;
        if pixel_sum >= mean {
            hash |= 1;
            //println!("Pixel {} is >= {} therefore {:b}", pixel_sum, mean, hash);
        } else {
            hash |= 0;
            //println!("Pixel {} is < {} therefore {:b}", pixel_sum, mean, hash);
        }
        hash <<= 1;
    }
    //println!("Hash for {} is {}", prepared_image.orig_path, hash);

    return hash;
}

/**
 * Calculate the dhash of the provided prepared image
 *
 * # Arguments
 *
 * * 'prepared_image' - The already prepared image for perceptual processing
 *
 * # Return
 *
 * Returns a u64 representing the value of the hash
 */
pub fn get_dhash(prepared_image: &PreparedImage) -> u64 {
    // Stored for later
    let first_pixel_val = prepared_image.image.pixels().nth(0).unwrap().channels()[0];
    let last_pixel_val = prepared_image.image.pixels().last().unwrap().channels()[0];

    // Calculate the dhash
    let mut previous_pixel_val = 0u64;
    let mut hash = 0u64;
    for (index, pixel) in prepared_image.image.pixels().enumerate() {
        if index == 0 {
            previous_pixel_val = pixel.channels()[0] as u64;
            continue;
        }
        let channels = pixel.channels();
        let pixel_val = channels[0] as u64;
        if pixel_val >= previous_pixel_val {
            hash |= 1;
        } else {
            hash |= 0;
        }
        hash <<= 1;
        previous_pixel_val = channels[0] as u64;
    }

    if first_pixel_val >= last_pixel_val {
        hash |= 1;
    } else {
        hash |= 0;
    }   

    return hash;
}

/**
 * Calculate the phash of the provided prepared image
 *
 * # Arguments
 *
 * * 'prepared_image' - The already prepared image for perceptual processing
 *
 * # Return
 *
 * Returns a u64 representing the value of the hash
 */
pub fn get_phash(prepared_image: &PreparedImage) -> u64 {
    0u64
}
