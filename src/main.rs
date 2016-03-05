// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate pihash;
extern crate rustc_serialize;
extern crate docopt;

use std::path::Path;
use docopt::Docopt;

// Getting the version information from cargo during compile time
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// The usage description
const USAGE: &'static str = "
Perceptual Image Hashing (pihash)

Usage:
    pihash [options] <path>...
    pihash (--help | --version)

Options:
    -h, --help      Show this screen.
    -V, --version   Print version.
    -a, --ahash     Include an ahash calculation.
    -d, --dhash     Include an dhash calculation.
    -p, --phash     Include an phash calculation.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version: bool,
    flag_ahash: bool,
    flag_dhash: bool,
    flag_phash: bool,
    arg_path: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());
    
    // Print version information and exit
    if args.flag_version {
        println!("Perceptual Image Hashing: v{}", VERSION);
        std::process::exit(0);
    }

    // Init the hashing library
    pihash::init();

    // println!("{:?}", args);

    // All flags set or, no flags set
    if (args.flag_ahash && args.flag_dhash && args.flag_phash) ||
       (!args.flag_ahash && !args.flag_dhash && !args.flag_phash) {
        for path in args.arg_path {
            let image_path = Path::new(&path);
            let hashes = pihash::get_phashes(&image_path);
            let hash_result = format!(r#"
                file: {}
                ahash: {}
                dhash: {}
                phash: {}
                "#,
                hashes.orig_path,
                hashes.ahash,
                hashes.dhash,
                hashes.phash
            );
            println!("{}", hash_result);
        }
        // Otherwise process only specific hashes
    } else {
        for path in args.arg_path {
            println!("file: {}", path);
            let image_path = Path::new(&path);
            if args.flag_ahash {
                let ahash = pihash::get_ahash(&image_path);
                println!("ahash: {}", ahash);
            }
            if args.flag_dhash {
                let dhash = pihash::get_dhash(&image_path);
                println!("dhash: {}", dhash);
            }
            if args.flag_phash {
                let phash = pihash::get_phash(&image_path);
                println!("phash: {}", phash);
            }
            println!("");
        }
    }
}
