use yaml_rust::{YamlLoader}; //, YamlEmitter};
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

use crate::module::baseclass::ObsidianModule;
use crate::stdlib::*;

pub fn compile_default_options(options_yaml_str: &str) -> Yaml {
    let default_module_config_options_arr = YamlLoader::load_from_str(options_yaml_str).unwrap();
    let obj = &default_module_config_options_arr[0];
    return obj.clone();
}

pub fn get_configured_options(obsmod: &ObsidianModule) -> Option<Yaml> {
    // get module name
    let module_name = obsmod.module_name.as_str();
    

    // extract only the module config of our module
    let module_configs = &obsmod.config["module_config"];
    let modconf = &module_configs[module_name];

    return match modconf {
        Yaml::BadValue => {
            None
        },
        _ => Some(modconf.clone())
    };
}

//pub fn get_options(default: Yaml, configured: &Yaml) -> Hash {
pub fn get_options(obsmod: &ObsidianModule) -> Yaml {
    // get default options
    let default = obsmod.default_options.clone();
    let configured_opt = get_configured_options(obsmod);
    

    // if no configured config could be found, just take the default config, no merging required
    let configured;
    match configured_opt {
        None => {
            return default
        },
        _ => {
            configured = configured_opt.unwrap();
        } 
    }

    fn rec(mut default: Hash, configured: &Yaml) -> Hash {// -> Yaml {
        for entry in default.iter_mut() {
            // unpack
            let key = &entry.0;
            let val = &entry.1;
            let key_str = key.as_str().unwrap();

            // skip entry if configured config does not have the key that default has
            // (e.g. "c")
            if variant_eq(&configured[key_str], &Yaml::BadValue) {
                continue;
            }

            // recurse and exit this branch if val = hash
            match val {
                Yaml::Hash(inner) => {
                    *entry.1 = Yaml::Hash(
                        rec(inner.clone(), &configured[key_str])
                    );
                    continue;
                },
                _ => (),
            };

            // update value only if types match
            if ! variant_eq(entry.1, &configured[key_str]) {
                eprintln!("Error: value types do not agree for key {}", key_str);
                continue;
            }

            // Update value
            *entry.1 = configured[key_str].clone();
        }
        return default
    }

    // get hashmap from Yaml type and then compile options
    let defaults_h = default.as_hash().unwrap().clone();
    let options = rec(defaults_h, &configured);

    // // Dump the YAML object
    // let mut out_str = String::new();
    // {
    //     let mut emitter = YamlEmitter::new(&mut out_str);
    //     emitter.dump(&Yaml::Hash(options.clone())).unwrap(); 
    // }
    // println!("{}", out_str);

    return Yaml::Hash(options);

}

