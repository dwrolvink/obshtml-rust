use crate::lib::errors::{Error};

// POSIX PATHS
// ===============================================================================
/*
    let abspath = AbsolutePosixPath::new("/path/to/file.json".to_string());
    println!("{}", abspath.to_string());

    let pxp = PosixPath::AbsolutePosixPath(AbsolutePosixPath("/path/to/files.json".to_string()));
    println!("my path: {}", pxp.to_string());

*/

#[derive(Clone, Debug, PartialEq)]
pub struct RelativePosixPath(pub String);

impl RelativePosixPath {
    pub fn new(path: String) -> RelativePosixPath {
        // tests here:
        // *
        // return if okay
        return RelativePosixPath(path);
    }
    // pub fn to_absolute(root: AbsolutePosixPath) -> RelativePosixPath {
    //     // ??
    // }
    pub fn to_string(&self) -> String {
        return self.0.clone();
    }
}

pub struct AbsolutePosixPath(pub Option<String>);
impl AbsolutePosixPath 
{
    pub fn new(path: String) -> Result<AbsolutePosixPath, Error> 
    {
        /*
            [SPEC]
            If an empty string is passed in -> AbsolutePosixPath(None)
            If the path does not start with an /, we cannot be sure that it is absolute -> fail
            If the path ends with a /, remove it to have a consistent path ending.
        */

        // empty path
        if path.len() == 0 {
            return Ok(AbsolutePosixPath(None));
        }

        // relative path
        if !path.starts_with('/') {
            return Err(Error::WrongPathType(
                format!("Path \"{}\" is not absolute!", path.as_str())
            ));
        }

        // strip final /
        if path.chars().last().unwrap() == '/' {
            let corrected_path = &path[0..path.len() - 1];
            return Ok(AbsolutePosixPath(Some(corrected_path.to_string())));
        };

        // happy path
        return Ok(AbsolutePosixPath(Some(path)));
    }

    // pub fn to_relative(root: AbsolutePosixPath) -> RelativePosixPath {
    //     // ??
    // }

    pub fn to_string(&self) -> String {
        let inner = self.0.as_ref();
        match inner {
            None => panic!("Cannot unpack absolute path, as contents are none!"),
            Some(s) => return s.clone(),
        }
    }

    pub fn clone(&self) -> AbsolutePosixPath {
        AbsolutePosixPath::new(self.to_string()).unwrap()
    }
}

pub enum PosixPath {
    RelativePosixPath(RelativePosixPath),
    AbsolutePosixPath(AbsolutePosixPath),
}
impl PosixPath {
    pub fn to_string(&self) -> String {
        return match self {
            PosixPath::RelativePosixPath(value) => value.to_string(),
            PosixPath::AbsolutePosixPath(value) => value.to_string(),
        }
    }
}

