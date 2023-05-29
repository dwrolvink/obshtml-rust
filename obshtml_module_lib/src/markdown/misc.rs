use lazy_static::lazy_static;
use regex::Regex;

pub fn strip_code_sections(content: &str) -> String {
    lazy_static! {
        static ref RE_CODE_BLOCK: Regex = Regex::new(r"(?m)^```([\s\S]*?)```[\s]*?$").unwrap();
        static ref RE_CODE_LINE:  Regex = Regex::new(r"`(.*?)`").unwrap();
    }
    let mut output = content.to_owned();
    output = RE_CODE_BLOCK.replace_all(&output, "").to_string();
    output = RE_CODE_LINE.replace_all(&output, "").to_string();
    return output;
}