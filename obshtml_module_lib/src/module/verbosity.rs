use crate::lib::errors::{Error};

// maximum level of logging to show
// (if you choose Info, Warning and Error will also be shown)
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Verbosity {
    Debug = 0,
    Info = 10,
    Deprecation = 20,
    Warning = 30,
    Error = 40,
    Quiet = 50
}

impl Verbosity {
    pub fn from_str(input: &str) -> Result<Verbosity, Error> {
        let l_input = input.to_lowercase();

        match l_input.as_str() {
            "debug"  => Ok(Verbosity::Debug),
            "info"  => Ok(Verbosity::Info),
            "deprecation"  => Ok(Verbosity::Deprecation),
            "warning" => Ok(Verbosity::Warning),
            "error" => Ok(Verbosity::Error),
            "quiet" => Ok(Verbosity::Quiet),
            _      => Err(Error::InvalidArgument(
                format!("Verbosity type {} is not known, known types: <debug, info, deprecation, warning, error, quiet>", l_input)
            )),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MessageVerbosity(pub Verbosity);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfiguredVerbosity(pub Verbosity);

// compares whether the requested verbosity is higher or equal to the set verbosity
// which would mean that the message is allowed to be printed
pub fn verbose_enough(
    configured_verbosity: ConfiguredVerbosity,
    message_verbosity: MessageVerbosity,
) -> bool {
    let conf_level = configured_verbosity.0 as i32;
    let msg_level = message_verbosity.0 as i32;
    return msg_level >= conf_level;
}


// TESTS
// ====================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_false_if_not_verbose_enough() {
        let print = verbose_enough(ConfiguredVerbosity(Verbosity::Error), MessageVerbosity(Verbosity::Info));
        assert_eq!(print, false);
    }

    #[test]
    fn return_true_if_verbose_enough() {
        let print = verbose_enough(ConfiguredVerbosity(Verbosity::Info), MessageVerbosity(Verbosity::Error));
        assert_eq!(print, true);
    }

    #[test]
    fn return_true_if_verbose_enough_equal() {
        let print = verbose_enough(ConfiguredVerbosity(Verbosity::Deprecation), MessageVerbosity(Verbosity::Deprecation));
        assert_eq!(print, true);
    }
}
