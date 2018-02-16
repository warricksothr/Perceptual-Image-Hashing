// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate docopt;
extern crate pihash;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::path::Path;

// Getting the version information from cargo during compile time
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// The usage description
const USAGE: &'static str = "
Perceptual Image Hashing (pihash)
Calculate the perceptual hash values for an input or compare the
input file to a set of other images and return a list of the similar
images.

Usage:
    pihash [options] <path> [<comparison>...]
    pihash (--help | --version)

Options:
    -h, --help      Show this screen.
    -V, --version   Print version.
    -a, --ahash     Include an ahash calculation.
    -d, --dhash     Include an dhash calculation.
    -p, --phash     Include an phash calculation.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_version: bool,
    flag_ahash: bool,
    flag_dhash: bool,
    flag_phash: bool,
    arg_path: String,
    arg_comparison: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // Print version information and exit
    if args.flag_version {
        println!("Perceptual Image Hashing: v{}", VERSION);
        std::process::exit(0);
    }

    // Init the hashing library
    let lib = pihash::PIHash::new(Some(pihash::cache::DEFAULT_CACHE_DIR));

    // println!("{:?}", args);
    if args.arg_comparison.len() > 0 {
        let base_image_path = Path::new(&args.arg_path);
        let base_hash = get_requested_perceptual_hashes(&lib, &base_image_path, &args);

        let mut comparison_hashes: Vec<pihash::hash::PerceptualHashes> = Vec::new();
        for index in 0..args.arg_comparison.len() {
            comparison_hashes
                .push(get_requested_perceptual_hashes(&lib,
                                                      &Path::new(&args.arg_comparison[index]),
                                                      &args));
        }

        let mut similar_images: Vec<&str> = Vec::new();
        for comparison_hash in comparison_hashes {
            if base_hash.similar(&comparison_hash) {
                similar_images.push(&comparison_hash.orig_path);
            }
        }

        println!("Base Image:");
        println!("{}", base_image_path.to_str().unwrap());
        println!("Similar Images:");
        for similar_image in similar_images {
            println!("{}", similar_image);
        }
    } else {
        let image_path = Path::new(&args.arg_path);
        let hashes = get_requested_perceptual_hashes(&lib, &image_path, &args);
        let hash_result = format!(r#"
            file: {}
            ahash: {}
            dhash: {}
            phash: {}
            "#,
                                  hashes.orig_path,
                                  hashes.ahash,
                                  hashes.dhash,
                                  hashes.phash);
        println!("{}", hash_result);
    }
}

fn flags_get_all_perceptual_hashes(args: &Args) -> bool {
    (args.flag_ahash && args.flag_dhash && args.flag_phash) ||
        (!args.flag_ahash && !args.flag_dhash && !args.flag_phash)
}

fn get_requested_perceptual_hashes<'a>(lib: &pihash::PIHash,
                                       image_path: &'a Path,
                                       args: &Args)
                                       -> pihash::hash::PerceptualHashes<'a> {
    let ahash = if args.flag_ahash || flags_get_all_perceptual_hashes(&args) {
        lib.get_ahash(&image_path)
    } else {
        0u64
    };

    let dhash = if args.flag_dhash || flags_get_all_perceptual_hashes(&args) {
        lib.get_dhash(&image_path)
    } else {
        0u64
    };

    let phash = if args.flag_phash || flags_get_all_perceptual_hashes(&args) {
        lib.get_phash(&image_path)
    } else {
        0u64
    };

    pihash::hash::PerceptualHashes {
        orig_path: image_path.to_str().unwrap(),
        ahash: ahash,
        dhash: dhash,
        phash: phash,
    }
}
