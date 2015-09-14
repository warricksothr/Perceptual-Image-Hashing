// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate pihash;

fn main() {
    let mut string = String::new();
    string = pihash::hello(string);
    println!("{}",string);
}
