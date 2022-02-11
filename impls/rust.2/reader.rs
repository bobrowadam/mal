use regex::Regex;

pub fn tokenize(str: String) {
    Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
}
