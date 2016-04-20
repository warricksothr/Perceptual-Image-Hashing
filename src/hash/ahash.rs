// Copyright 2016 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use super::{HashType, PerceptualHash, Precision, PreparedImage};
use super::prepare_image;
use super::image::{GenericImage, Pixel};
use std::path::Path;
use cache::Cache;

pub struct AHash<'a> {
    prepared_image: Box<PreparedImage<'a>>,
}

impl<'a> AHash<'a> {
    pub fn new(path: &'a Path, precision: &Precision, cache: &Option<Cache>) -> Self {
        AHash {
            prepared_image: Box::new(prepare_image(&path, &HashType::AHash, &precision, cache)),
        }
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
    fn get_hash(&self, _: &Option<Cache>) -> u64 {
        match self.prepared_image.image {
            Some(ref image) => {
                let (width, height) = image.dimensions();

                // calculating the average pixel value
                let mut total = 0u64;
                for pixel in image.pixels() {
                    let channels = pixel.channels();
                    // println!("Pixel is: {}", channels[0]);
                    total += channels[0] as u64;
                }
                let mean = total / (width * height) as u64;
                // println!("Mean for {} is {}", prepared_image.orig_path, mean);

                // Calculating a hash based on the mean
                let mut hash = 0u64;
                for pixel in image.pixels() {
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
            None => 0u64,
        }
    }
}
