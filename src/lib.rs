// Copyright 2015 Drew Short <drew@sothr.com>.
//
// Licensed under the MIT license<LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

pub fn hello(mut result: String) -> String {
    let helloworld = "Hello, World!";
    result.push_str(helloworld);
    result
}
