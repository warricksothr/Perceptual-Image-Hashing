// Copyright 2016 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use super::{HashType, PerceptualHash, Precision, PreparedImage};
use super::prepare_image;
use super::image::{GenericImage, Pixel};
use std::path::Path;
use cache::Cache;

pub struct DHash<'a> {
    prepared_image: Box<PreparedImage<'a>>,
}

impl<'a> DHash<'a> {
    pub fn new(path: &'a Path, precision: &Precision, cache: &'a Cache) -> Self {
        DHash {
            prepared_image: Box::new(prepare_image(&path, &HashType::DHash, &precision, &cache)),
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
