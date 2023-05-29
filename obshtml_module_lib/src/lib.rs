pub mod stdlib;

pub mod module {
    pub mod verbosity;
    pub mod baseclass;
    pub mod options;
    pub mod modfile;
}
pub mod cli {
    pub mod config;
    pub mod execute;
}
pub mod markdown {
    pub mod misc;
}
pub mod lib {
    pub mod errors;
    pub mod paths;
    pub mod misc;
    pub mod file;
}

pub type ObsidianModuleConfig<'a> = module::baseclass::ObsidianModuleConfig<'a>;
pub type ObsidianModule = module::baseclass::ObsidianModule;