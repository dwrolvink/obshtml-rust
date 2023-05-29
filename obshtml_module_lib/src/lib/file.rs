use std::fs;
use std::io;

pub fn read(file_path: &str) -> Option<String> {
    let contents = fs::read_to_string(file_path);
    return match contents {
        Ok(x) => Some(x),
        Err(e) => {
            eprintln!("read_file function failed with error: \"{}\" (path: {})", e, file_path);
            None
        }
    };
}

pub fn write(file_path: &str, contents: &str) -> io::Result<()> {
    let res = fs::write(file_path, contents);
    return match res {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("write_file function failed with error: \"{}\" (path: {})", e, file_path);
            Err(e)
        },
    };
}
