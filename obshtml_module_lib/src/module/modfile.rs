use std::io;
use std;
use std::io::prelude::*;

use crate::lib::paths::{RelativePosixPath, AbsolutePosixPath};
use crate::lib::file;
use crate::lib::misc::{get_datetime};

use crate::module::baseclass::ObsidianModule;


pub struct Modfile {
    module_data_folder: AbsolutePosixPath,
    resource_access_log: Option<AbsolutePosixPath>,
    provides: Vec<String>,
    requires: Vec<String>,
    file_path: RelativePosixPath,
}

impl Modfile  {
    pub fn new(obsmod: &ObsidianModule, file_path: &str) -> Modfile {
        let irl_path = get_instance_resource_log_path(&obsmod.module_data_folder, obsmod.instance_id.clone());
        Modfile {
            module_data_folder: obsmod.module_data_folder.clone(),
            resource_access_log: irl_path,
            provides: obsmod.provides.clone(),
            requires: obsmod.requires.clone(),
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
        // test if file is in requires
        let rel_path = self.file_path.to_string();
        if ! self.requires.contains(&rel_path) {
            if rel_path != "config.yml" {
                panic!("Error: Trying to read modfile {}, but it is not listed in the module's requires property.", rel_path);
            }
        }

        // log access
        self.log_access("read");

        // read file and return contents in result
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

        // log access
        self.log_access("write");

        // write file and return result
        let abs_file_path = self.get_abs_file_path();
        file::write(&abs_file_path, contents)
    }

    pub fn log_access(&self, access_type: &str) {
        // if resource_access_log is not set, we cannot log this, and so we won't
        if self.resource_access_log.is_none() {
            return;
        }

        let ral_path_abs_path = match &self.resource_access_log {
            Some(abs_path) => abs_path.clone(),
            None => panic!("Fall through"),
        };
        let ral_path_str = ral_path_abs_path.to_string();
    
        // ensure folder exists
        let path = std::path::Path::new(&ral_path_str);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        // ensure file exists
        if ! path.exists() {
            file::write(&ral_path_str, "access_type; modfile; time_of_access\n").unwrap();
        };
    
        // compile log line
        let datetime = get_datetime();
        let line = format!("{}; {}; {}", access_type, self.file_path.to_string(), datetime.to_iso_string());
    
        // append to log file
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap();
    
        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}; {}", &ral_path_str, e);
        }
    }

}

pub fn get_instance_resource_log_path(module_data_folder: &AbsolutePosixPath, instance_id: Option<String>) -> Option<AbsolutePosixPath> {
    if instance_id.is_none() {
        return None
    };

    let mdf_path_str = module_data_folder.to_string();
    let irl_path_str = format!("{}/instances/resources_access/{}.csv", mdf_path_str, instance_id.unwrap());

    return Some(AbsolutePosixPath::new(irl_path_str).unwrap());
}

