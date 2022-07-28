use xml::attribute::OwnedAttribute;
use crate::models::helpers::get_attr_value_by_name;

Initializer! {
    #[derive(Debug, Clone)]
    pub struct Address {
        pub kind: String,
        pub addr: String,
    }
}

impl From<&Vec<OwnedAttribute>> for Address {
    fn from(attr: &Vec<OwnedAttribute>) -> Self {
        let mut addr = Address::new();
        addr.addr = get_attr_value_by_name(attr, "addr");
        addr.kind = get_attr_value_by_name(attr, "addrtype");
        addr
    }
}