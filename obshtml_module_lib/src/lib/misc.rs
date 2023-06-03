use crate::stdlib::*;
use json;
use yaml_rust::Yaml;

use chrono::prelude::{DateTime, Local};

pub struct DateTimeParts {
    pub date: String,
    pub time: String,
    pub timezone: String,
}

pub fn get_datetime() -> DateTimeParts {
    // get current datetime
    let local: DateTime<Local> = Local::now(); 

    // split on whitespace
    let local = local.to_string();
    let mut datetime_clean = local.split_whitespace();
    
    // extract fields
    let date = datetime_clean.next().unwrap();
    let time = datetime_clean.next().unwrap();

    // not sure if timezone is always present, wrap this
    let timezone = match datetime_clean.next() {
        Some(tz) => tz,
        _ => "",
    };

    return DateTimeParts{
        date: date.to_owned(),
        time: time.to_owned(),
        timezone: timezone.to_owned(),
    }
}

impl DateTimeParts {
    pub fn to_iso_string(&self) -> String {
        format!("{}T{}", self.date, self.time).to_owned()
    }
}

pub fn expect_at_least_n_args(args: &Vec<String>, n: usize, error_text: &str) {
    // number of args expected minus 1 (the command, arg[0])
    if args.len() < n + 1 {
        panic!("{}",
            format_error(
                "Invalid arguments", 
                format!("Expected at least {} argument(s), got {} arguments", n, args.len() - 1).as_str(),
                error_text
            )
        )
    }
}

pub fn yaml_to_json(yaml: &Yaml) -> Result<json::JsonValue, String> {
    match yaml {
        Yaml::Real(_) => {
            // Handle conversion of YAML number to JSON number if needed
            Ok(json::JsonValue::Number(json::number::Number::from(yaml.as_f64().unwrap())))
        }
        Yaml::Integer(val) => Ok(json::JsonValue::Number((*val).into())),
        Yaml::Boolean(val) => Ok(json::JsonValue::Boolean(*val)),
        Yaml::String(val) => Ok(json::JsonValue::String(val.clone())),
        Yaml::Array(vals) => {
            let mut json_array = Vec::new();
            for val in vals {
                json_array.push(yaml_to_json(val)?);
            }
            Ok(json::JsonValue::Array(json_array))
        }
        Yaml::Hash(hash) => {
            let mut json_obj = json::JsonValue::new_object();
            for (key, val) in hash {
                if let Some(key_str) = key.as_str() {
                    json_obj[key_str.to_owned()] = yaml_to_json(val)?;
                }
            }
            Ok(json_obj)
        }
        _ => Err("Unsupported YAML value".to_string()),
    }
}