// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate image;
extern crate sha1;

use self::image::ImageBuffer;
use self::sha1::Sha1;
use std::path::Path;
use std::fs::{File, create_dir_all, remove_dir_all};
use std::io::{Read, Error};
use std::option::Option;
use std::result::Result;

const CACHE_DIR: &'static str = "./.hash_cache";
const CACHE_FILE_EXT: &'static str = "png";

// Creates the required directories 
pub fn prep_cache() -> Result<(), Error> {
    create_dir_all(CACHE_DIR)
}

pub fn clear_cache() -> Result<(), Error> {
    remove_dir_all(CACHE_DIR)
}

/**
 * Get the hash of the desired file and return it as a hex string
 */
fn get_file_hash(path: &Path) -> Result<String, Error> {
    let mut source = try!(File::open(&path));
    let mut buf: Vec<u8> = Vec::new();
    try!(source.read_to_end(&mut buf));
    let mut sha1 = Sha1::new();
    sha1.update(&buf);
    // Return the hex result of the hash
    Ok(sha1.hexdigest())
}

/**
 * Put an image buffer in the cache
 */
pub fn put_image_in_cache(path: &Path, size: u32, image: &ImageBuffer<image::Luma<u8>, Vec<u8>>)  {
    let hash = get_file_hash(&path);
    match hash {
        Ok(sha1) => {
            let cache_path_str = format!("{}/{}x{}_{}.{}",CACHE_DIR, size, size, sha1, CACHE_FILE_EXT);
            let cached_path = Path::new(&cache_path_str);
            // Save the file into the cache
            match image.save(cached_path) {
                Ok(_) => {},
                Err(e) => println!("Error: {}", e),
            }
        },
        Err(e) =>  println!("Error: {}", e),
    }
}

/**
 * Expects a slice of slices that represents lines in the file
 */
pub fn put_file_in_cache(path: &Path, size: u32, file_contents: &[&[u8]])  {

}

/**
 * Get an image buffer out of the cache
 */
pub fn get_image_from_cache(path: &Path, size: u32) -> Option<ImageBuffer<image::Luma<u8>, Vec<u8>>> {
    let hash = get_file_hash(&path);
    match hash {
        Ok(sha1) => {
            // Check if the file exists in the cache
            let cache_path_str = format!("{}/{}x{}_{}.{}",CACHE_DIR, size, size, sha1, CACHE_FILE_EXT);
            let cached_path = Path::new(&cache_path_str);
            // Try to open, if it does, then we can read the image in
            match File::open(&cached_path) {
                Ok(_) => {
                    let image = image::open(&cached_path).unwrap();
                    Some(image.to_luma())
                },
                // Don't really care here, it just means an existing cached
                // file doesn't exist, or can't be read.
                Err(_) => {
                    None
                },
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            None
        },
    }
}

#[test]
fn test_get_file_hash() {
    let target = "test_images/sample_01_large.jpg";
    let target_path = Path::new(target);
    let hash = get_file_hash(&target_path);
    match hash {
        Ok(v) => {
            println!("Hash: {}", v);
            assert!(v == "4beb6f2d852b75a313863916a1803ebad13a3196");
        }
        Err(e) => {
            println!("Error: {:?}", e);
            assert!(false);
        }
    }
}
