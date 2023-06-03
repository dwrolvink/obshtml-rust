use crate::stdlib::*;
use crate::lib::paths::{AbsolutePosixPath}; //, PosixPath, RelativePosixPath};
use crate::lib::misc::{expect_at_least_n_args};

// CONFIG
// ==================================================================================
pub enum Config {
    RunConfig(RunConfig),
    AcceptConfig(AcceptConfig),
    ProvidesConfig(MinimalConfig),
    RequiresConfig(MinimalConfig),
    AltersConfig(MinimalConfig),
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
            "provides" => Config::ProvidesConfig(MinimalConfig::new(&args)),
            "requires" => Config::RequiresConfig(MinimalConfig::new(&args)),
            "alters" => Config::AltersConfig(MinimalConfig::new(&args)),
            _ => panic!("{}",
                    format_error(
                        "Invalid arguments", 
                        format!("Command {} not valid.", command.as_str()).as_str(),
                        "Available commands: <run, accept, provides, requires, alters>"
                    )
                )
        };
    }
}

pub struct RunConfig {
    pub command: String,
    pub module_data_folder: AbsolutePosixPath,
    pub instance_id: Option<String>,
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
        let mut instance_id = None;
        if args.len() > 3 {
            instance_id = Some(args[3].clone());
        };

        // create
        return RunConfig{
            command: command, 
            module_data_folder: mdf,
            instance_id: instance_id,
        };
    }
}


pub struct AcceptConfig{
    pub command: String,
    pub module_data_folder: AbsolutePosixPath,
    pub instance_id: Option<String>,
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
        let mut instance_id = None;
        if args.len() > 3 {
            instance_id = Some(args[3].clone());
        };

        // create
        return AcceptConfig{
            command: command, 
            module_data_folder: mdf,
            instance_id: instance_id,
        };
    }
}

// This config only requires a command, this is per spec the bare minimum
pub struct MinimalConfig{
    pub command: String,
}
impl MinimalConfig {
    pub fn new(args: &Vec<String>) -> MinimalConfig {
        return MinimalConfig{
            command: args[1].clone(), 
        };
    }
}


