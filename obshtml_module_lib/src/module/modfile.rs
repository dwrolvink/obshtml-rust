use std::io;
use crate::lib::paths::{RelativePosixPath, AbsolutePosixPath};
use crate::lib::file;
use crate::module::baseclass::ObsidianModule;

pub struct Modfile {
    module_data_folder: AbsolutePosixPath,
    provides: Vec<String>,
    file_path: RelativePosixPath,
}

impl Modfile  {
    pub fn new(obsmod: &ObsidianModule, file_path: &str) -> Modfile {
        Modfile {
            module_data_folder: obsmod.module_data_folder.clone(),
            provides: obsmod.provides.clone(),
            file_path: RelativePosixPath(file_path.to_string()),
        }
    }

    pub fn get_abs_file_path(&self) -> String {
        let mut abs_file_path = self.module_data_folder.to_string();
        abs_file_path.push_str("/");
        abs_file_path.push_str(&self.file_path.to_string());
        return abs_file_path;
    }

    pub fn read(&self) -> Option<String> {
        let abs_file_path = self.get_abs_file_path();
        file::read(&abs_file_path)
    }

    pub fn write(&self, contents: &str) -> io::Result<()> {
        // test if file is in provides
        let rel_path = self.file_path.to_string();
        if ! self.provides.contains(&rel_path) {
            eprintln!("Error: Trying to write to modfile {}, but it is not listed in the module's provides property.", rel_path);
            return Err(io::Error::new(io::ErrorKind::Other, "File not listed in provides."));
        }
        let abs_file_path = self.get_abs_file_path();
        file::write(&abs_file_path, contents)
    }
}

pub fn compile_provides(input: Vec<&str>) -> Vec<String> {
    return input.iter().map(|s| s.to_string()).collect();
}