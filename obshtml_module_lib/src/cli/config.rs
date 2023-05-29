use crate::stdlib::*;
use crate::lib::paths::{AbsolutePosixPath}; //, PosixPath, RelativePosixPath};
use crate::lib::misc::{expect_at_least_n_args};

// CONFIG
// ==================================================================================
pub enum Config {
    RunConfig(RunConfig),
    AcceptConfig(AcceptConfig),
}
impl Config {
    pub fn new() -> Config {
        let args = sys_argsv();

        expect_at_least_n_args(
            &args, 1, 
            format!("e.g. `{} <command>`", args[0].as_str()).as_str()
        );

        let command = args[1].clone();

        return match command.as_str() {
            "run" => Config::RunConfig(RunConfig::new(&args)),
            "accept" => Config::AcceptConfig(AcceptConfig::new(&args)),
            _ => panic!("{}",
                    format_error(
                        "Invalid arguments", 
                        format!("Command {} not valid.", command.as_str()).as_str(),
                        "Available commands: <run, accept>"
                    )
                )
        };
    }
}

pub struct RunConfig {
    pub command: String,
    pub module_data_folder: AbsolutePosixPath,
}
impl RunConfig {
    pub fn new(args: &Vec<String>) -> RunConfig {
        expect_at_least_n_args(
            args, 2, 
            format!("e.g. `{} {} <module_data_folder>`", args[0].as_str(), args[1].as_str()).as_str()
        );
        // extract data
        let command = args[1].clone();
        let mdf = AbsolutePosixPath::new(args[2].clone());
        let mdf = match mdf {
            Ok(x) => x,
            Err(err) => {
                panic!("Casting module_data_folder commandline input to AbsolutePosixPath failed: \"{}\"", err);
            }
        };
        // create
        return RunConfig{
            command: command, 
            module_data_folder: mdf
        };
    }
}


pub struct AcceptConfig{
    pub command: String,
    pub module_data_folder: AbsolutePosixPath,
}
impl AcceptConfig {
    pub fn new(args: &Vec<String>) -> AcceptConfig {
        expect_at_least_n_args(
            args, 2, 
            format!("e.g. `{} {} <module_data_folder>`", args[0].as_str(), args[1].as_str()).as_str()
        );
        // extract data
        let command = args[1].clone();
        let mdf = AbsolutePosixPath::new(args[2].clone());
        let mdf = match mdf {
            Ok(x) => x,
            Err(err) => {
                panic!("Casting module_data_folder commandline input to AbsolutePosixPath failed: \"{}\"", err);
            }
        };
        // create
        return AcceptConfig{
            command: command, 
            module_data_folder: mdf
        };
    }
}


