// extern crate json;
extern crate yaml_rust;
extern crate regex;

mod module {
    pub mod verbosity;
    pub mod baseclass;
    pub mod options;
    pub mod modfile;
}
mod cli {
    pub mod config;
    pub mod execute;
}
mod lib {
    pub mod errors;
    pub mod paths;
    pub mod misc;
    pub mod file;
}

mod stdlib;
use crate::stdlib::*;

use crate::module::options::{compile_default_options, get_configured_options, get_options};
use crate::module::verbosity;


fn main() {
    println!("{}", "I have no feet but I must run");

//     let default = compile_default_options("
// a:
//   a1: old (should be overwritten)
//   a2: old (should not be overwritten)
// b: old (should be overwritten)
// c: old (only in default)
//     ");

//     let configured = get_configured_options("test").unwrap();
//     let options = get_options(default, &configured);

}
