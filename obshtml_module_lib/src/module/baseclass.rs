use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

use crate::module::verbosity::{verbose_enough, Verbosity, ConfiguredVerbosity, MessageVerbosity};
use crate::module::modfile::{Modfile};

// use crate::lib::errors;
use crate::lib::paths::{AbsolutePosixPath}; //, RelativePosixPath, PosixPath};
use super::options;

/*
    What does a rust module need to do to be called by ObsidianHtml, and work as intended?
    What needs to be in the python wrapper?
    How will we call the rust module?

    obshtml: do you want to run? rust_module.accept(module_data_folder) -> json(bool)

*/

// DATACLASS
// =============================================================================================
pub struct ObsidianModule {
    pub module_name: String,
    pub module_class_name: String,
    pub persistent: bool,
    pub default_options: Yaml,
    pub config: Yaml,
    pub options: Yaml,
    pub module_data_folder: AbsolutePosixPath,
    pub states: ObsidianModuleStates,
    pub run_fn: fn(ObsidianModule),
    pub accept_fn: fn(ObsidianModule),
    pub provides: Vec<String>,
    pub verbosity: ConfiguredVerbosity,
}
impl Default for ObsidianModule {
    fn default() -> ObsidianModule {
        ObsidianModule {
            module_name: "<module_name>".to_string(),
            module_class_name: "ObsidianModule".to_string(),
            verbosity: ConfiguredVerbosity(Verbosity::Error),
            persistent: false,
            default_options: Yaml::Null,
            config: Yaml::Null,
            options: Yaml::Null,
            module_data_folder: AbsolutePosixPath::new("".to_string()).unwrap(),
            run_fn: placeholder_run_fn,
            accept_fn: placeholder_accept_fn,
            provides: Vec::new(),
            states: ObsidianModuleStates{
                _cancelled_run: false
            }
        }
    }
}
#[allow(unused_variables)]
fn placeholder_run_fn(obsmod: ObsidianModule) {
    panic!("No run_fn function passed in, this should not be possible");
}
#[allow(unused_variables)]
fn placeholder_accept_fn(obsmod: ObsidianModule) {
    panic!("No accept_fn function passed in, this should not be possible");
}
/*
    // usage of Default:
    let obsmod = ObsidianModule { module_name: "rust_module".to_string(), ..Default::default() };
*/

pub struct ObsidianModuleStates {
    _cancelled_run: bool
}

// INSTANTIATION
// =============================================================================================
// required parameters for instantiating thes ObsidianModule via new()
pub struct ObsidianModuleConfig<'a> {
    pub module_name: &'a str,
    pub module_class_name: &'a str,
    pub default_options: Yaml,
    pub run_fn: fn(ObsidianModule),
    pub accept_fn: fn(ObsidianModule),
    pub provides: Vec<String>,
}

// METHODS
// =============================================================================================
impl ObsidianModule {

    // recommended way to instantiate a new ObsidianModule struct
    pub fn new(config: ObsidianModuleConfig, mdf: AbsolutePosixPath) -> ObsidianModule {
        let mut obsmod = ObsidianModule {
            module_name: config.module_name.to_string(),
            module_class_name: config.module_class_name.to_string(),
            module_data_folder: mdf,
            default_options: config.default_options.clone(),
            run_fn: config.run_fn,
            accept_fn: config.accept_fn,
            provides: config.provides,
            ..Default::default()
        };

        obsmod.load_settings();

        return obsmod;
    }

    pub fn load_settings(&mut self) {
        self.config = self.get_config();
        self.options = options::get_options(self);
        self.verbosity = self.get_configured_verbosity();

    }

    pub fn modfile(&self, file_path: &str) -> Modfile {
        Modfile::new(&self, file_path)
    }

    pub fn get_config(&self) -> Yaml {
        let contents = self.modfile("config.yml").read().unwrap();
        let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
        let doc = &docs[0];
        return doc.clone();
    }
    
    pub fn get_configured_verbosity(&self) -> ConfiguredVerbosity {
        let verbosity_str = self.config["verbosity"].as_str().unwrap();
        return ConfiguredVerbosity(Verbosity::from_str(verbosity_str).unwrap());
    }

    // return f'{self.module_name} ({self.module_class_name})'
    pub fn nametag(&self) -> String {
        return format!("{} ({})", self.module_name, self.module_class_name);
    }

    pub fn verbose_enough(&self, message_verbosity: MessageVerbosity,) -> bool {
        return verbose_enough(self.verbosity, message_verbosity);
    }

    pub fn stderr(&self, verbosity_str: &str, msg: &str) {
        let verbosity = Verbosity::from_str(verbosity_str).unwrap();
        let message_verbosity = MessageVerbosity(verbosity.clone());
        if self.verbose_enough(message_verbosity) {
            eprint!("[{:^13}]   ", format!("{:?}", verbosity));
            eprintln!("{}", msg);
        }
    }

    pub fn requires(&self) -> Option<Vec<String>> {
        return None;
    }
    pub fn provides(&self) -> Option<Vec<String>> {
        return None;
    }
    pub fn alters(&self) -> Option<Vec<String>> {
        return None;
    }
    pub fn accept(&self, _module_data_folder: Option<AbsolutePosixPath>) -> bool {
        return true;
    }

    pub fn run(&self, _module_data_folder: Option<AbsolutePosixPath>) -> bool {
        return true;
    }
}


// TESTS
// =============================================================================================

#[cfg(test)]
mod tests {
    use crate::module::baseclass::{ObsidianModule, ObsidianModuleConfig};
    use crate::module::verbosity::{ConfiguredVerbosity, MessageVerbosity, Verbosity};

    fn get_obsmod() -> ObsidianModule {
        let obscfg = ObsidianModuleConfig{
            module_name: "hello",
            module_class_name: "<rust impl>",
            persistent: false,
        };
        return ObsidianModule::new(obscfg);
    }

    #[test]
    fn implement_verbose_enough() {
        let obsmod = get_obsmod();
        let print = obsmod.verbose_enough(ConfiguredVerbosity(Verbosity::Info), MessageVerbosity(Verbosity::Info));
        assert_eq!(print, true);
    }

    #[test]
    fn implement_nametag() {
        let obsmod = get_obsmod();
        let tag = obsmod.nametag();
        assert_eq!(tag, "hello (<rust impl>)");
    }
}