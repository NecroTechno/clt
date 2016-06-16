// Rust command line utility.
// Copyright (c) 2015 by Shipeng Feng.
// Originally licensed under the BSD License.
// Copyright (c) 2016 by Shaun Gillies.
// Licensed under the GPLv3 License, see LICENSE for more details.

//! clt is a Rust crate for creating beautiful command line applications.

#![crate_name = "clt"]
#![crate_type = "lib"]
#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://shaungillies.github.io/clt/")]
#![deny(non_camel_case_types)]

extern crate libc;
extern crate getopts;

/* public api */
pub use utils::sprintln;
pub use term::{
    Style,
    Color,
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        White,
    prompt,
    confirm,
    get_terminal_size,
    print_via_pager,
    isatty,
    clear,
    ProgressBar,
    Editor,
};

mod core;
mod types;
mod utils;
mod term;
mod formatting;
