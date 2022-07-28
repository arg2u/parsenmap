use xml::attribute::OwnedAttribute;
use crate::models::helpers::get_attr_value_by_name;

Initializer! {
#[derive(Debug, Clone)]
    pub struct Script {
        pub id: String,
        pub output: String,
    }
}

impl From<&Vec<OwnedAttribute>> for Script {
    fn from(attr: &Vec<OwnedAttribute>) -> Self {
        let mut script = Script::new();
        script.id = get_attr_value_by_name(attr, "id");
        script.output = get_attr_value_by_name(attr, "output");
        script
    }
}
