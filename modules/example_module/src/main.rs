/*
    This is a simple example module showing off the capabilities of modules.
*/

extern crate obshtml_module_lib;
extern crate yaml_rust;
extern crate json;

// use yaml_rust::{YamlLoader, YamlEmitter};
// use yaml_rust::Yaml;
use json::object;

use obshtml_module_lib::{ObsidianModuleConfig, ObsidianModule};
use obshtml_module_lib::module::options::{compile_default_options}; //get_configured_options
use obshtml_module_lib::cli::execute;

use obshtml_module_lib::stdlib::*;

fn main() {
    // define the default config options for this module that can be overwritten
    // by users in module_config: {}
    let default_options = compile_default_options("
a:
    a1: old (should be overwritten)
    a2: old (should not be overwritten)
b: old (should be overwritten)
c: old (only in default)
        ");

    // list files that this module will create/read/alter
    // you can get these via the commandline by running `<your binary name> provides` (etc)
    let provides = to_vec_string(vec!("test.json"));
    let requires = to_vec_string(vec!());
    let alters   = to_vec_string(vec!());

    // define module config
    let obs_cfg = ObsidianModuleConfig {
        module_name: "example-rs",
        module_class_name: "<crate::obshtml_module_example>",
        run_fn: run,
        accept_fn: accept,
        default_options: default_options,
        provides: provides,
        requires: requires,
        alters: alters,
    };

    execute::start(obs_cfg);
}

fn run(obsmod: ObsidianModule) {
    /*
        run this method after building, by running: 
        ./target/release/obshtml_module_example run path/to/module_data_folder
    */

    // example of how to get the module options
    // it also shows the returned values and how to unpack them
    // *note also the eprintln, we should only print valid json to stdout!*
    let val = &obsmod.options["a"];
    obsmod.stderr("debug", &format!("{}< {:?} >", get_type_of(val), val, ));

    let val = &obsmod.options["b"];
    obsmod.stderr("debug", &format!("{}< {:?} >", get_type_of(val), val, ));

    let val = &obsmod.options["b"].as_str().unwrap();
    obsmod.stderr("debug", &format!("{}< {:?} >", get_type_of(val), val, ));

    // write a random modfile
    let mod_file1 = obsmod.modfile("test.json");

    let data = object!{
        foo: false,
        bar: null,
        answer: 42,
        list: [null, "world", true]
    };   

    mod_file1.write(&data.pretty(2)).unwrap();

    // get path of modfile
    eprintln!("Debug: abs path of modfile: {}", &mod_file1.get_abs_file_path());

    // read a random modfile
    let mod_file2 = obsmod.modfile("test.json");
    eprintln!("Debug: test.json contents:\n{}", mod_file2.read().unwrap());

    // return output
    // make sure to only output valid json to stdout when running as an actual module
    let output = r#"{"result": true}"#;
    println!("{}", output);
}


fn accept(_obsmod: ObsidianModule) {
    /*
        This function is called by ObsidianHtml to test if we need to run the module proper
        In our current case, we always want to run this module when configured
    */
    let output = r#"{"result": true}"#;
    println!("{}", output);
}