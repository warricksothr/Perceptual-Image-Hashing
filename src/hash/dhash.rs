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

pub struct DHash<'a> {
    prepared_image: Box<PreparedImage<'a>>,
}

impl<'a> DHash<'a> {
    pub fn new(path: &'a Path, precision: &Precision, cache: &Option<Cache>) -> Self {
        DHash {
            prepared_image: Box::new(prepare_image(&path, &HashType::DHash, &precision, cache)),
        }
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
    fn get_hash(&self, _: &Option<Cache>) -> u64 {
        match self.prepared_image.image {
            Some(ref image) => {
                let (_, _, first_pixel) = image.pixels().nth(0).unwrap();
                let (_, _, last_pixel) = image.pixels().last().unwrap();
                let first_pixel_value = first_pixel.0[0] as u64;
                let last_pixel_value = last_pixel.0[0] as u64;

                // Calculate the dhash
                let mut previous_pixel_value = 0u64;
                let mut hash = 0u64;
                for (x, y, pixel) in image.pixels() {
                    if x == 0 && y == 0 {
                        previous_pixel_value = pixel.0[0] as u64;
                        continue;
                    }
                    let pixel_val = pixel.0[0] as u64;
                    if pixel_val >= previous_pixel_value {
                        hash |= 1;
                    } else {
                        hash |= 0;
                    }
                    hash <<= 1;
                    previous_pixel_value = first_pixel_value;
                }

                if first_pixel_value >= last_pixel_value {
                    hash |= 1;
                } else {
                    hash |= 0;
                }

                hash
            }
            None => 0u64,
        }
    }
}
