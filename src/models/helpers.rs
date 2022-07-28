use regex::Regex;
use xml::attribute::OwnedAttribute;
pub fn get_attr_value_by_name(attrs: &Vec<OwnedAttribute>, name: &str) -> String {
    for attr in attrs {
        if attr.name.to_string() == name {
            let re = Regex::new(r"\n").unwrap();
            let res = re.replace_all(&attr.value, "");
            return res.trim().to_string();
        }
    }
    String::new()
}
