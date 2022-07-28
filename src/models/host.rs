use crate::models::{address::Address, port::Port};

Initializer! {
    #[derive(Debug, Clone)]
    pub struct Host {
        pub state: String,
        pub addrs: Vec<Address>,
        pub ports: Vec<Port>,
    }
}
