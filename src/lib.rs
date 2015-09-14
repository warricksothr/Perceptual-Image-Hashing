// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

pub fn hello(mut result: String) -> String {
    let helloworld = "Hello, World!";
    result.push_str(helloworld);
    result
}

/*
 * Module for the tests
 */
#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::path;

    #[test]
    fn can_get_test_images() {
        let paths = fs::read_dir(&path::Path::new("./test_images")).unwrap();
        let mut num_paths = 0;
        for path in paths {
            let orig_path = path.unwrap().path();
            let ext = path::Path::new(&orig_path).extension();
            match ext {
                Some(_) => { 
                    if ext.unwrap() == "jpg" {
                        println!("Is a image {}: {:?}", num_paths, orig_path) ;
                        num_paths += 1;
                    }
                },
                _ => {
                    println!("Not an image: {:?}", orig_path) ;
                    continue
                }
            }
            //println!("Name: {}", path.unwrap().path().display())
        }
        // Currently 12 images in the test imaages directory
        assert!(num_paths == 12);
    }

}
