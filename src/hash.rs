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
use self::image::imageops;

pub struct PreparedImage<'a> {
    orig_path: &'a str,
    image: image::ImageBuffer<image::Luma<u8>,Vec<u8>>
}

pub fn prepare_image(path: &Path, size: u32) -> PreparedImage {
    let image_path = path.to_str().unwrap();
    let image = image::open(path).unwrap();
    let small_image = image.resize_exact(size, size, image::FilterType::Lanczos3);
    let grey_image = small_image.to_luma();
    PreparedImage { orig_path: &*image_path, image: grey_image }
}

/*
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
 * Returns a u64 representing the value of the hash
 */
pub fn get_ahash(prepared_image: PreparedImage) -> u64 {
    let img = prepared_image.image;
    let (width, height) = img.dimensions();

    // calculating the average pixel value
    let mut total = 0u64;
    for pixel in img.pixels() {
        let channels = pixel.channels();
        //println!("Pixel is: {}", channels[0]);
        total += channels[0] as u64;
    }
    let mean = total / (width*height) as u64;
    //println!("Mean for {} is {}", prepared_image.orig_path, mean);

    // Calculating a hash based on the mean
    let mut hash = 0u64;
    for pixel in img.pixels() {
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
pub fn get_dhash(prepared_image: PreparedImage) -> u64 {

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
pub fn get_phash(prepared_image: PreparedImage) -> u64 {

}
