// functions/macros to be included everywhere in this project
/*
    mod stdlib;
    use stdlib::*;
*/

use std;

// quick print statement for lazy fingers
// #[macro_export]
// macro_rules! out {
//     ($val:expr) => {
//         println!("{}", $val);
//     };
// }
// pub(crate) use out; 

// compare enum variants, not wrapped values
pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

pub fn get_type_of<T>(_: &T) -> &str {
    return std::any::type_name::<T>()
}

// get all commandline arguments
pub fn sys_argsv() -> Vec<String> {
    return std::env::args().collect();
}

// used to format panic! inline errors
pub fn format_error(etype: &str, descriptor: &str, msg: &str) -> String {
    let output = format!("\n\tError: {}: {}\n\t{}\n", etype, descriptor, msg);
    return output;
}

// for ease of defining Vec<String>s
pub fn to_vec_string(input: Vec<&str>) -> Vec<String> {
    return input.iter().map(|s| s.to_string()).collect();
}
