use crate::models::helpers::get_attr_value_by_name;
use crate::models::{script::Script, service::Service};
use xml::attribute::OwnedAttribute;
Initializer! {
    #[derive(Debug, Clone)]
    pub struct Port {
        pub protocol: String,
        pub port: String,
        pub state: String,
        pub service: Service,
        pub scripts: Vec<Script>,
    }
}

impl From<&Vec<OwnedAttribute>> for Port {
    fn from(attr: &Vec<OwnedAttribute>) -> Self {
        let mut new_port = Port::new();
        new_port.protocol = get_attr_value_by_name(attr, "protocol");
        new_port.port = get_attr_value_by_name(attr, "portid");
        new_port
    }
}
