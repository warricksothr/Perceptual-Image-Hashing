// Copyright 2016 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate complex;
extern crate flate2;
extern crate image;
extern crate sha1;

use self::image::ImageBuffer;
use self::sha1::Sha1;
use self::flate2::Compression;
use self::flate2::write::ZlibEncoder;
use self::flate2::read::ZlibDecoder;
use std::str::FromStr;
use std::path::Path;
use std::fs::{File, create_dir_all, remove_dir_all};
use std::io::{Read, Error, Write};
use std::option::Option;
use std::result::Result;
use super::rustc_serialize::json;

pub const DEFAULT_CACHE_DIR: &'static str = "./.hash_cache";
const CACHED_IMAGE_EXT: &'static str = "png";
const CACHED_MATRIX_EXT: &'static str = "dft";
// Caching version information
const CACHE_VERSION: u32 = 1;
const CACHE_METADATA_FILE: &'static str = "cache.meta";

#[derive(RustcDecodable, RustcEncodable)]
struct CacheMetadata {
    cache_version: u32,
}

impl Default for CacheMetadata {
    fn default() -> CacheMetadata {
        CacheMetadata { cache_version: CACHE_VERSION }
    }
}

impl PartialEq<CacheMetadata> for CacheMetadata {
    fn eq(&self, other: &CacheMetadata) -> bool {
        self.cache_version == other.cache_version
    }

    fn ne(&self, other: &CacheMetadata) -> bool {
        !self.eq(&other)
    }
}

/**
* Structure to hold implementation of the cache
*/
pub struct Cache<'a> {
    pub cache_dir: &'a str,
    pub use_cache: bool,
}

impl<'a> Default for Cache<'a> {
    fn default() -> Cache<'a> {
        Cache {
            cache_dir: DEFAULT_CACHE_DIR,
            use_cache: true,
        }
    }
}

impl<'a> Cache<'a> {
    /**
     * Create the required directories for the cache
     */
    pub fn init(&self) -> Result<(), Error> {
        match create_dir_all(self.cache_dir) {
            Ok(_) => {
                let metadata_path_str = format!("{}/{}", self.cache_dir, CACHE_METADATA_FILE);
                let metadata_path = Path::new(&metadata_path_str);
                let current_metadata: CacheMetadata = Default::default();
                match File::open(&metadata_path) {
                    Ok(mut file) => {
                        // Metadata file exists, compare them
                        let mut loaded_metadata_string = String::new();
                        match file.read_to_string(&mut loaded_metadata_string) {
                            Ok(_) => {
                                let loaded_metadata: CacheMetadata =
                                    match json::decode(&loaded_metadata_string) {
                                        Ok(data) => data,
                                        Err(_) => CacheMetadata { cache_version: 0 },
                                    };

                                // If they match, continue
                                if current_metadata != loaded_metadata {
                                    // If they don't wipe the cache to start new
                                    match remove_dir_all(self.cache_dir) {
                                        Ok(_) => {
                                            match create_dir_all(self.cache_dir) {
                                                Ok(_) => (),
                                                Err(e) => println!("Error: {}", e),
                                            }
                                        }
                                        Err(e) => println!("Error: {}", e),
                                    };
                                };
                            }
                            Err(e) => println!("Error: {}", e),
                        };
                    }
                    // Metadata file doesn't exist, do nothing assume all is well,
                    // create new metadata file
                    Err(_) => {}
                };
                let encoded_cache_metadata = json::encode(&current_metadata).unwrap();
                match File::create(&metadata_path) {
                    Ok(mut file) => {
                        let _ = file.write(&encoded_cache_metadata.as_bytes());
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    /**
     * Clean the cache directory completely
     */
    pub fn clean(&self) -> Result<(), Error> {
        remove_dir_all(self.cache_dir)
    }

    /**
     * Get the hash of the desired file and return it as a hex string
     */
    pub fn get_file_hash(&self, path: &Path) -> Result<String, Error> {
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
    pub fn put_image_in_cache(&self,
                              path: &Path,
                              size: u32,
                              image: &ImageBuffer<image::Luma<u8>, Vec<u8>>)
                              -> Result<bool, Error> {
        let hash = self.get_file_hash(&path);
        match hash {
            Ok(sha1) => {
                let cache_path_str = format!("{}/image/{}x{}/{}.{}",
                                             self.cache_dir,
                                             size,
                                             size,
                                             sha1,
                                             CACHED_IMAGE_EXT);
                let cache_dir_str = format!("{}/image/{}x{}", self.cache_dir, size, size);
                match create_dir_all(cache_dir_str) {
                    Ok(_) => {
                        let cached_path = Path::new(&cache_path_str);
                        // Save the file into the cache
                        match image.save(cached_path) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error: {}", e);
                                return Err(e);
                            }
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(e);
            }
        }
        Ok(true)
    }

    /**
     * Get an image buffer out of the cache
     */
    pub fn get_image_from_cache(&self,
                                path: &Path,
                                size: u32)
                                -> Option<ImageBuffer<image::Luma<u8>, Vec<u8>>> {
        if self.use_cache {
            let hash = self.get_file_hash(&path);
            match hash {
                Ok(sha1) => {
                    // Check if the file exists in the cache
                    let cache_path_str = format!("{}/image/{}x{}/{}.{}",
                                                 self.cache_dir,
                                                 size,
                                                 size,
                                                 sha1,
                                                 CACHED_IMAGE_EXT);
                    let cached_path = Path::new(&cache_path_str);
                    // Try to open, if it does, then we can read the image in
                    match File::open(&cached_path) {
                        Ok(_) => {
                            let image = image::open(&cached_path).unwrap();
                            Some(image.to_luma())
                        }
                        // Don't really care here, it just means an existing cached
                        // file doesn't exist, or can't be read.
                        Err(_) => None,
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    None
                }
            }
        } else {
            None
        }
    }

    /**
     * Expects a slice of slices that represents lines in the file
     */
    pub fn put_matrix_in_cache(&self,
                               path: &Path,
                               size: u32,
                               file_contents: &Vec<Vec<f64>>)
                               -> Result<bool, Error> {
        let hash = self.get_file_hash(&path);
        match hash {
            Ok(sha1) => {
                let cache_path_str = format!("{}/matrix/{}x{}/{}.{}",
                                             self.cache_dir,
                                             size,
                                             size,
                                             sha1,
                                             CACHED_MATRIX_EXT);
                let cache_dir_str = format!("{}/matrix/{}x{}", self.cache_dir, size, size);
                match create_dir_all(cache_dir_str) {
                    Ok(_) => {
                        let cached_path = Path::new(&cache_path_str);
                        // Save the file into the cache
                        match File::create(&cached_path) {
                            Ok(mut file) => {
                                let mut compressor = ZlibEncoder::new(Vec::new(),
                                                                      Compression::Default);
                                for row in file_contents {
                                    let mut row_str = row.iter().fold(String::new(),
                                                                      |acc, &item| {
                                                                          acc +
                                                                          &format!("{},", item)
                                                                      });
                                    // remove the last comma
                                    let desire_len = row_str.len() - 1;
                                    row_str.truncate(desire_len);
                                    row_str.push_str("\n");
                                    try!(compressor.write(&row_str.into_bytes()));
                                }
                                let compressed_matrix = match compressor.finish() {
                                    Ok(data) => data,
                                    Err(e) => {
                                        println!("Unable to compress matrix data: {}", e);
                                        return Err(e);
                                    }
                                };
                                try!(file.write(&compressed_matrix));
                                try!(file.flush());
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(e);
            }
        }
        Ok(true)
    }

    /**
     * Get a matrix out of the cache
     */
    pub fn get_matrix_from_cache(&self, path: &Path, size: u32) -> Option<Vec<Vec<f64>>> {
        if self.use_cache {
            let hash = self.get_file_hash(&path);
            match hash {
                Ok(sha1) => {
                    // Check if the file exists in the cache
                    let cache_path_str = format!("{}/matrix/{}x{}/{}.{}",
                                                 self.cache_dir,
                                                 size,
                                                 size,
                                                 sha1,
                                                 CACHED_MATRIX_EXT);
                    let cached_path = Path::new(&cache_path_str);
                    // Try to open, if it does, then we can read the image in
                    match File::open(&cached_path) {
                        Ok(file) => {
                            let mut decoder = ZlibDecoder::new(&file);
                            let mut matrix_data_str = String::new();
                            match decoder.read_to_string(&mut matrix_data_str) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Unable to decompress matrix: {}", e);
                                    return None;
                                }
                            };
                            // convert the matrix
                            let matrix: Vec<Vec<f64>> = matrix_data_str.trim()
                                                                       .split("\n")
                                                                       .map(|line| {
                                                                           line.split(",")
                                                                               .map(|f| {
                                                                                   f64::from_str(f)
                                                                                       .unwrap()
                                                                               })
                                                                               .collect()
                                                                       })
                                                                       .collect();

                            Some(matrix)
                        }
                        // Don't really care here, it just means an existing cached
                        // file doesn't exist, or can't be read.
                        Err(_) => None,
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    None
                }
            }
        } else {
            None
        }
    }
}

#[test]
fn test_get_file_hash() {
    let target = "test_images/sample_01_large.jpg";
    let target_path = Path::new(target);
    let cache: Cache = Default::default();
    let hash = cache.get_file_hash(&target_path);
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
