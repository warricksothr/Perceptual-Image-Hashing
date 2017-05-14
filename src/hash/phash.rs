// Copyright 2016 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use super::dft;
use super::dft::Transform;
use super::{HashType, PerceptualHash, Precision, PreparedImage};
use super::prepare_image;
use super::image::{GenericImage, Pixel};
use std::path::Path;
use cache::Cache;

pub struct PHash<'a> {
    prepared_image: Box<PreparedImage<'a>>,
}

impl<'a> PHash<'a> {
    pub fn new(path: &'a Path, precision: &Precision, cache: &Option<Cache>) -> Self {
        PHash {
            prepared_image: Box::new(prepare_image(&path, &HashType::PHash, &precision, cache)),
        }
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
    fn get_hash(&self, cache: &Option<Cache>) -> u64 {
        match self.prepared_image.image {
            Some(ref image) => {
                // Get the image data into a vector to perform the DFT on.
                let width = image.width() as usize;
                let height = image.height() as usize;

                // Get 2d data to 2d FFT/DFT
                // Either from the cache or calculate it
                // Pretty fast already, so caching doesn't make a huge difference
                // Atleast compared to opening and processing the images
                let data_matrix: Vec<Vec<f64>> = match *cache {
                    Some(ref c) => {
                        match c.get_matrix_from_cache(&Path::new(self.prepared_image.orig_path),
                                                      width as u32) {
                            Some(matrix) => matrix,
                            None => {
                                let matrix = create_data_matrix(width, height, &image);
                                match c.put_matrix_in_cache(&Path::new(self.prepared_image.orig_path),
                                                            width as u32,
                                                            &matrix) {
                                    Ok(_) => {}
                                    Err(e) => println!("Unable to store matrix in cache. {}", e),
                                };
                                matrix
                            }
                        }
                    }
                    None => create_data_matrix(width, height, &image),
                };

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
                    for y in 0..target_height {
                        if data_matrix[x][y] >= mean {
                            hash |= 1;
                        } else {
                            hash |= 0;
                        }
                        hash <<= 1;
                    }
                }
                hash
            }
            None => 0u64,
        }
    }
}

fn create_data_matrix(width: usize,
                      height: usize,
                      image: &super::image::ImageBuffer<super::image::Luma<u8>, Vec<u8>>)
                      -> Vec<Vec<f64>> {
    let mut data_matrix: Vec<Vec<f64>> = Vec::new();
    // Preparing the results
    for x in 0..width {
        data_matrix.push(Vec::new());
        for y in 0..height {
            let pos_x = x as u32;
            let pos_y = y as u32;
            data_matrix[x].push(image.get_pixel(pos_x, pos_y).channels()[0] as f64);
        }
    }

    // Perform the 2D DFT operation on our matrix
    calculate_2d_dft(&mut data_matrix);
    data_matrix
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
    if f >= super::FLOAT_PRECISION_MAX_1 || f <= super::FLOAT_PRECISION_MIN_1 {
        f
    } else if f >= super::FLOAT_PRECISION_MAX_2 || f <= super::FLOAT_PRECISION_MIN_2 {
        (f * 10_f64).round() / 10_f64
    } else if f >= super::FLOAT_PRECISION_MAX_3 || f <= super::FLOAT_PRECISION_MIN_3 {
        (f * 100_f64).round() / 100_f64
    } else if f >= super::FLOAT_PRECISION_MAX_4 || f <= super::FLOAT_PRECISION_MIN_4 {
        (f * 1000_f64).round() / 1000_f64
    } else if f >= super::FLOAT_PRECISION_MAX_5 || f <= super::FLOAT_PRECISION_MIN_5 {
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
