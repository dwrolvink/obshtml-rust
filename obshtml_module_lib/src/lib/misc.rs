use crate::stdlib::*;
use json;
use yaml_rust::Yaml;

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