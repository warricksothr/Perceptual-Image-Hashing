// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern image;

use self::image::ImageBuffer;
use std::path::Path;

/**
 * Get the hash of the desired file and return it as a hex string
 */
fn get_file_hash(path: &Path) -> String {
    
}

/**
 * Put an image buffer in the cache
 */
pub fn put_in_cache(path: &Path, image: &ImageBuffer)  {

}

/**
 * Get an image buffer out of the cache
 */
pub fn get_from_cache(path: &Path) -> Some(ImageBuffer) {

}
