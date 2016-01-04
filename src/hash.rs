// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

// Pull in the image processing crate
extern crate image;
extern crate dft;
extern crate complex;

use std::path::Path;
use std::f64;
use self::image::{GenericImage, Pixel, FilterType};
use self::dft::Transform;
use cache;

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

/**
 * Prepared image that can be used to generate hashes
 */
pub struct PreparedImage<'a> {
    orig_path: &'a str,
    image: image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
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
    Ahash,
    Dhash,
    Phash,
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
pub fn prepare_image<'a>(path: &'a Path,
                         hash_type: &HashType,
                         precision: &Precision)
                         -> PreparedImage<'a> {
    let image_path = path.to_str().unwrap();
    let size: u32 = match *hash_type {
        HashType::Phash => precision.get_size() * 4,
        _ => precision.get_size(),
    };
    // Check if we have the already converted image in a cache and use that if possible.
    match cache::get_image_from_cache(&path, size) {
        Some(image) => {
            PreparedImage {
                orig_path: &*image_path,
                image: image,
            }
        }
        None => {
            // Otherwise let's do that work now and store it.
            let image = image::open(path).unwrap();
            let small_image = image.resize_exact(size, size, FilterType::Lanczos3);
            let grey_image = small_image.to_luma();
            cache::put_image_in_cache(&path, size, &grey_image);
            PreparedImage {
                orig_path: &*image_path,
                image: grey_image,
            }
        }
    }
}

/**
 * Get all perceptual hashes for an image
 */
pub fn get_perceptual_hashes<'a>(path: &'a Path, precision: &Precision) -> PerceptualHashes<'a> {
    let image_path = path.to_str().unwrap();
    let ahash = AHash::new(&path, &precision).get_hash();
    let dhash = DHash::new(&path, &precision).get_hash();
    let phash = PHash::new(&path, &precision).get_hash();
    PerceptualHashes {
        orig_path: &*image_path,
        ahash: ahash,
        dhash: dhash,
        phash: phash,
    }
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
            '1' => hamming += 1,
            _ => continue,
        }
    }
    hamming
}

pub trait PerceptualHash {
    fn get_hash(&self) -> u64;
}

pub struct AHash<'a> {
    prepared_image: Box<PreparedImage<'a>>,
}

impl<'a> AHash<'a> {
    pub fn new(path: &'a Path, precision: &Precision) -> Self {
        AHash { prepared_image: Box::new(prepare_image(&path, &HashType::Ahash, &precision)) }
    }
}

impl<'a> PerceptualHash for AHash<'a> {
    /**
    * Calculate the ahash of the provided prepared image.
    *
    * # Returns
    *
    * A u64 representing the value of the hash
    */
    fn get_hash(&self) -> u64 {
        let (width, height) = self.prepared_image.image.dimensions();

        // calculating the average pixel value
        let mut total = 0u64;
        for pixel in self.prepared_image.image.pixels() {
            let channels = pixel.channels();
            // println!("Pixel is: {}", channels[0]);
            total += channels[0] as u64;
        }
        let mean = total / (width * height) as u64;
        // println!("Mean for {} is {}", prepared_image.orig_path, mean);

        // Calculating a hash based on the mean
        let mut hash = 0u64;
        for pixel in self.prepared_image.image.pixels() {
            let channels = pixel.channels();
            let pixel_sum = channels[0] as u64;
            if pixel_sum >= mean {
                hash |= 1;
                // println!("Pixel {} is >= {} therefore {:b}", pixel_sum, mean, hash);
            } else {
                hash |= 0;
                // println!("Pixel {} is < {} therefore {:b}", pixel_sum, mean, hash);
            }
            hash <<= 1;
        }
        // println!("Hash for {} is {}", prepared_image.orig_path, hash);

        hash
    }
}

pub struct DHash<'a> {
    prepared_image: Box<PreparedImage<'a>>,
}

impl<'a> DHash<'a> {
    pub fn new(path: &'a Path, precision: &Precision) -> Self {
        DHash { prepared_image: Box::new(prepare_image(&path, &HashType::Dhash, &precision)) }
    }
}

impl<'a> PerceptualHash for DHash<'a> {
    /**
     * Calculate the dhash of the provided prepared image
     *
     * # Return
     *
     * Returns a u64 representing the value of the hash
     */
    fn get_hash(&self) -> u64 {
        // Stored for later
        let first_pixel_val = self.prepared_image.image.pixels().nth(0).unwrap().channels()[0];
        let last_pixel_val = self.prepared_image.image.pixels().last().unwrap().channels()[0];

        // Calculate the dhash
        let mut previous_pixel_val = 0u64;
        let mut hash = 0u64;
        for (index, pixel) in self.prepared_image.image.pixels().enumerate() {
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

        hash
    }
}

pub struct PHash<'a> {
    prepared_image: Box<PreparedImage<'a>>,
}

impl<'a> PHash<'a> {
    pub fn new(path: &'a Path, precision: &Precision) -> Self {
        PHash { prepared_image: Box::new(prepare_image(&path, &HashType::Phash, &precision)) }
    }
}

impl<'a> PerceptualHash for PHash<'a> {
    /**
     * Calculate the phash of the provided prepared image
     *
     * # Return
     *
     * Returns a u64 representing the value of the hash
     */
    fn get_hash(&self) -> u64 {
        // Get the image data into a vector to perform the DFT on.
        let width = self.prepared_image.image.width() as usize;
        let height = self.prepared_image.image.height() as usize;

        // Get 2d data to 2d FFT/DFT
        let mut data_matrix: Vec<Vec<f64>> = Vec::new();
        for x in 0..width {
            data_matrix.push(Vec::new());
            for y in 0..height {
                let pos_x = x as u32;
                let pos_y = y as u32;
                data_matrix[x]
                    .push(self.prepared_image.image.get_pixel(pos_x, pos_y).channels()[0] as f64);
            }
        }

        // Perform the 2D DFT operation on our matrix
        calculate_2d_dft(&mut data_matrix);
        // Store this DFT in the cache
        cache::put_matrix_in_cache(&Path::new(self.prepared_image.orig_path),
                                   width as u32,
                                   &"dft",
                                   &data_matrix);

        // Only need the top left quadrant
        let target_width = (width / 4) as usize;
        let target_height = (height / 4) as usize;
        let dft_width = (width / 4) as f64;
        let dft_height = (height / 4) as f64;

        // Calculate the mean
        let mut total = 0f64;
        for x in 0..target_width {
            for y in 0..target_height {
                total += data_matrix[x][y];
            }
        }
        let mean = total / (dft_width * dft_height);

        // Calculating a hash based on the mean
        let mut hash = 0u64;
        for x in 0..target_width {
            // println!("Mean: {} Values: {:?}",mean,data_matrix[x]);
            for y in 0..target_height {
                if data_matrix[x][y] >= mean {
                    hash |= 1;
                    // println!("Pixel {} is >= {} therefore {:b}", pixel_sum, mean, hash);
                } else {
                    hash |= 0;
                    // println!("Pixel {} is < {} therefore {:b}", pixel_sum, mean, hash);
                }
                hash <<= 1;
            }
        }
        // println!("Hash for {} is {}", prepared_image.orig_path, hash);
        hash
    }
}

// Use a 1D DFT to cacluate the 2D DFT.
//
// This is achieved by calculating the DFT for each row, then calculating the
// DFT for each column of DFT row data. This means that a 32x32 image with have
// 1024 1D DFT operations performed on it. (Slightly caclulation intensive)
//
// This operation is in place on the data in the provided vector
//
// Inspired by:
// http://www.inf.ufsc.br/~visao/khoros/html-dip/c5/s2/front-page.html
//
// Checked with:
// http://calculator.vhex.net/post/calculator-result/2d-discrete-fourier-transform
//
fn calculate_2d_dft(data_matrix: &mut Vec<Vec<f64>>) {
    // println!("{:?}", data_matrix);
    let width = data_matrix.len();
    let height = data_matrix[0].len();

    let mut complex_data_matrix = Vec::with_capacity(width);

    // Perform DCT on the columns of data
    for x in 0..width {
        let mut column: Vec<f64> = Vec::with_capacity(height);
        for y in 0..height {
            column.push(data_matrix[x][y]);
        }

        // Perform the DCT on this column
        // println!("column[{}] before: {:?}", x, column);
        let forward_plan = dft::Plan::new(dft::Operation::Forward, column.len());
        column.transform(&forward_plan);
        let complex_column = dft::unpack(&column);
        // println!("column[{}] after: {:?}", x, complex_column);
        complex_data_matrix.push(complex_column);
    }

    // Perform DCT on the rows of data
    for y in 0..height {
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            row.push(complex_data_matrix[x][y]);
        }
        // Perform DCT on the row
        // println!("row[{}] before: {:?}", y, row);
        let forward_plan = dft::Plan::new(dft::Operation::Forward, row.len());
        row.transform(&forward_plan);
        // println!("row[{}] after: {:?}", y, row);

        // Put the row values back
        for x in 0..width {
            data_matrix[x][y] = round_float(row[x].re);
        }
    }
}

fn round_float(f: f64) -> f64 {
    if f >= FLOAT_PRECISION_MAX_1 || f <= FLOAT_PRECISION_MIN_1 {
        f
    } else if f >= FLOAT_PRECISION_MAX_2 || f <= FLOAT_PRECISION_MIN_2 {
        (f * 10_f64).round() / 10_f64
    } else if f >= FLOAT_PRECISION_MAX_3 || f <= FLOAT_PRECISION_MIN_3 {
        (f * 100_f64).round() / 100_f64
    } else if f >= FLOAT_PRECISION_MAX_4 || f <= FLOAT_PRECISION_MIN_4 {
        (f * 1000_f64).round() / 1000_f64
    } else if f >= FLOAT_PRECISION_MAX_5 || f <= FLOAT_PRECISION_MIN_5 {
        (f * 10000_f64).round() / 10000_f64
    } else {
        (f * 100000_f64).round() / 100000_f64
    }
}

#[test]
fn test_2d_dft() {
    let mut test_matrix: Vec<Vec<f64>> = Vec::new();
    test_matrix.push(vec![1f64, 1f64, 1f64, 3f64]);
    test_matrix.push(vec![1f64, 2f64, 2f64, 1f64]);
    test_matrix.push(vec![1f64, 2f64, 2f64, 1f64]);
    test_matrix.push(vec![3f64, 1f64, 1f64, 1f64]);

    println!("{:?}", test_matrix[0]);
    println!("{:?}", test_matrix[1]);
    println!("{:?}", test_matrix[2]);
    println!("{:?}", test_matrix[3]);

    println!("Performing 2d DFT");
    calculate_2d_dft(&mut test_matrix);

    println!("{:?}", test_matrix[0]);
    println!("{:?}", test_matrix[1]);
    println!("{:?}", test_matrix[2]);
    println!("{:?}", test_matrix[3]);

    assert!(test_matrix[0][0] == 24_f64);
    assert!(test_matrix[0][1] == 0_f64);
    assert!(test_matrix[0][2] == 0_f64);
    assert!(test_matrix[0][3] == 0_f64);

    assert!(test_matrix[1][0] == 0_f64);
    assert!(test_matrix[1][1] == 0_f64);
    assert!(test_matrix[1][2] == -2_f64);
    assert!(test_matrix[1][3] == 2_f64);

    assert!(test_matrix[2][0] == 0_f64);
    assert!(test_matrix[2][1] == -2_f64);
    assert!(test_matrix[2][2] == -4_f64);
    assert!(test_matrix[2][3] == -2_f64);

    assert!(test_matrix[3][0] == 0_f64);
    assert!(test_matrix[3][1] == 2_f64);
    assert!(test_matrix[3][2] == -2_f64);
    assert!(test_matrix[3][3] == 0_f64);
}
