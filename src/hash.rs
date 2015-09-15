// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

// Pull in the image processing crate
extern crate image;
extern crate dft;
extern crate complex;

use std::path::Path;
use self::image::{
    GenericImage,
    Pixel,
    FilterType
};
use self::dft::real;
use self::complex::*;

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
pub fn get_perceptual_hashes(path: &Path, size: u32) -> PerceptualHashes {
    let image_path = path.to_str().unwrap();
    let prepared_image = prepare_image(path, size);
    // phash uses a DFT, so it needs an image 4 times larger to work with for
    // the same precision of hash. That said, this hash is much more accurate.
    let phash_prepared_image = prepare_image(path, size*4);
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

    hash
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

    hash
}

/*
 * Use a 1D DFT to cacluate the 2D DFT.
 *
 * This is achieved by calculating the DFT for each row, then calculating the
 * DFT for each column of DFT row data. This means that a 32x32 image with have
 * 1024 1D DFT operations performed on it. (Slightly caclulation intensive)
 *
 * This operation is in place on the data in the provided vector
 *
 * Inspired by:
 * http://www.inf.ufsc.br/~visao/khoros/html-dip/c5/s2/front-page.html
 *
 * Checked with:
 * http://calculator.vhex.net/post/calculator-result/2d-discrete-fourier-transform
 */
fn calculate_2d_dft(data_matrix: &mut Vec<Vec<f64>>){
    //println!("{:?}", data_matrix);
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
        //println!("column[{}] before: {:?}", x, column);
        real::forward(&mut column);
        let complex_column = real::unpack(&column);
        //println!("column[{}] after: {:?}", x, complex_column);
        complex_data_matrix.push(complex_column);
    }

    // Perform DCT on the rows of data
    for y in 0..height {
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            row.push(complex_data_matrix[x][y]);
        }
        // Perform DCT on the row
        //println!("row[{}] before: {:?}", y, row);
        dft::complex::forward(&mut row);
        //println!("row[{}] after: {:?}", y, row);

        // Put the row values back
        for x in 0..width {
            data_matrix[x][y] = row[x].re();
        }
    }
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
    // Get the image data into a vector to perform the DFT on.
    let  width = prepared_image.image.width() as usize;
    let  height = prepared_image.image.height() as usize;
    
    // Get 2d data to 2d FFT/DFT
    let mut data_matrix: Vec<Vec<f64>> = Vec::new();
    for x in (0..width) {
        data_matrix.push(Vec::new());
        for y in (0..height) {
            let pos_x = x as u32;
            let pos_y = y as u32;
            data_matrix[x].push(prepared_image.image.get_pixel(pos_x,pos_y).channels()[0] as f64);
        }
    }

    // Perform the 2D DFT operation on our matrix
    calculate_2d_dft(&mut data_matrix);
    
    // Only need the top left quadrant
    let target_width = (width / 4) as usize;
    let target_height = (height / 4) as usize;
    let dft_width = (width / 4) as f64;
    let dft_height = (height / 4) as f64;

    //Calculate the mean
    let mut total = 0f64;
    for x in (0..target_width) {
        for y in (0..target_height) {
            total += data_matrix[x][y];
        }
    }
    let mean = total / (dft_width * dft_height); 

    // Calculating a hash based on the mean
    let mut hash = 0u64;
    for x in (0..target_width) {
//        println!("Mean: {} Values: {:?}",mean,data_matrix[x]);
        for y in (0..target_height) {
            if data_matrix[x][y] >= mean {
                hash |= 1;
                //println!("Pixel {} is >= {} therefore {:b}", pixel_sum, mean, hash);
            } else {
                hash |= 0;
                //println!("Pixel {} is < {} therefore {:b}", pixel_sum, mean, hash);
            }
            hash <<= 1;
        }
    }
    //println!("Hash for {} is {}", prepared_image.orig_path, hash);
    hash
}

#[test]
fn test_2d_dft() {
    let mut test_matrix: Vec<Vec<f64>> = Vec::new();
    test_matrix.push(vec![1f64,1f64,1f64,3f64]);
    test_matrix.push(vec![1f64,2f64,2f64,1f64]);
    test_matrix.push(vec![1f64,2f64,2f64,1f64]);
    test_matrix.push(vec![3f64,1f64,1f64,1f64]);

    println!("{:?}",test_matrix[0]);
    println!("{:?}",test_matrix[1]);
    println!("{:?}",test_matrix[2]);
    println!("{:?}",test_matrix[3]);
    
    println!("Performing 2d DFT");
    calculate_2d_dft(&mut test_matrix);

    println!("{:?}",test_matrix[0]);
    println!("{:?}",test_matrix[1]);
    println!("{:?}",test_matrix[2]);
    println!("{:?}",test_matrix[3]);
    
    assert!(test_matrix[0][0] == 24f64);
    assert!(test_matrix[0][1] == 0f64);
    assert!(test_matrix[0][2] == 0f64);
    assert!(test_matrix[0][3] == 0f64);

    assert!(test_matrix[1][0] == 0f64);
    assert!(test_matrix[1][1] == -0.0000000000000006661338147750939f64);
    assert!(test_matrix[1][2] == -2.0000000000000004f64);
    assert!(test_matrix[1][3] == 1.9999999999999993f64);

    assert!(test_matrix[2][0] == 0f64);
    assert!(test_matrix[2][1] == -2f64);
    assert!(test_matrix[2][2] == -4f64);
    assert!(test_matrix[2][3] == -2f64);

    assert!(test_matrix[3][0] == 0f64);
    assert!(test_matrix[3][1] == 2.000000000000001f64);
    assert!(test_matrix[3][2] == -1.9999999999999996f64);
    assert!(test_matrix[3][3] == 0.0000000000000006661338147750939f64);
}
