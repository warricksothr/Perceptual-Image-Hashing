// Copyright 2016 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate image;

use std::path::Path;

use cache::Cache;

use super::prepare_image;
use super::{HashType, PerceptualHash, Precision, PreparedImage};

use self::image::GenericImageView;

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
                for (_, _, pixel) in image.pixels() {
                    total += pixel.0[0] as u64;
                }
                let mean = total / (height * width) as u64;
                // println!("Mean for {} is {}", prepared_image.orig_path, mean);

                // Calculating a hash based on the mean
                let mut hash = 0u64;
                for (_, _, pixel) in image.pixels() {
                    if pixel.0[0] as u64 >= mean {
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
