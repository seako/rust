// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::BytesContainer;
use std::io::{Command, fs, USER_RWX};
use std::os;

fn main() {
    // If we're the child, make sure we were invoked correctly
    let args = os::args();
    if args.len() > 1 && args[1].as_slice() == "child" {
        return assert_eq!(args[0],
                          format!("mytest{}", os::consts::EXE_SUFFIX));
    }

    test();
}

fn test() {
    // If we're the parent, copy our own binary to a new directory.
    let my_path = os::self_exe_name().unwrap();
    let my_dir  = my_path.dir_path();

    let child_dir = Path::new(my_dir.join("issue-15149-child"));
    drop(fs::mkdir(&child_dir, USER_RWX));

    let child_path = child_dir.join(format!("mytest{}",
                                            os::consts::EXE_SUFFIX));
    fs::copy(&my_path, &child_path).unwrap();

    // Append the new directory to our own PATH.
    let mut path = os::split_paths(os::getenv("PATH").unwrap_or(String::new()));
    path.push(child_dir.clone());
    let path = os::join_paths(path.as_slice()).unwrap();

    let child_output = Command::new("mytest").env("PATH", path.as_slice())
                                             .arg("child")
                                             .output().unwrap();

    assert!(child_output.status.success(),
            format!("child assertion failed\n child stdout:\n {}\n child stderr:\n {}",
                    child_output.output.container_as_str().unwrap(),
                    child_output.error.container_as_str().unwrap()));
}
