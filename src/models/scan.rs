//! # Result of parsing nmap xml
use std::fs::File;
use std::io::Write;
use std::path::Path;

use xml::attribute::OwnedAttribute;

use crate::models::helpers::get_attr_value_by_name;
use crate::models::host::Host;

/// Enum with type for an output file
pub enum FileType {
    CSV,
    JSON,
}

Initializer! {
    #[derive(Debug, Clone)]
    pub struct Scan {
        pub scanner: String,
        pub args: String,
        pub start: String,
        pub version: String,
        pub verbose: String,
        pub debugging: String,
        pub hosts: Vec<Host>,
    }
}

impl Scan {
    pub fn from_reparent(&mut self, attr: &Vec<OwnedAttribute>) {
        self.version = get_attr_value_by_name(attr, "version");
        self.scanner = get_attr_value_by_name(attr, "scanner");
        self.args = get_attr_value_by_name(attr, "args");
        self.start = get_attr_value_by_name(attr, "start");
    }
    /// Writes parsed result to csv or json file
    pub fn write_to_file(&self, f_type: FileType, path: &str) {
        let mut file = File::create(Path::new(path)).unwrap();
        let typed_buf;
        match f_type {
            FileType::CSV => {
                typed_buf = self.to_csv();
            }
            FileType::JSON => {
                typed_buf = self.to_json();
            }
        };
        file.write(typed_buf.as_bytes()).unwrap();
    }
    /// Generates json ready string
    pub fn to_json(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
    /// Generates csv ready string
    pub fn to_csv(&self) -> String {
        let mut column_names = vec![
            "host".to_string(),
            "port_num".to_string(),
            "port_proto".to_string(),
            "state".to_string(),
            "service_name".to_string(),
            "service_product".to_string(),
            "service_version".to_string(),
        ];
        let mut lines = vec![];
        let mut max_scripts_count = 0;
        let mut max_count = 0;
        for host in &self.hosts {
            for port in &host.ports {
                let mut line = vec![];
                for addr in &host.addrs {
                    if addr.kind == "ipv4" {
                        line.push(addr.addr.clone());
                    }
                }
                line.push(port.port.clone());
                line.push(port.protocol.clone());
                line.push(port.state.clone());
                line.push(port.service.name.clone());
                line.push(port.service.product.clone());
                line.push(port.service.version.clone());
                let scripts_len = port.scripts.len();
                if scripts_len > max_scripts_count {
                    max_scripts_count = scripts_len;
                }
                for script in &port.scripts {
                    line.push(script.id.clone());
                    line.push(script.output.clone());
                }
                if line.len() > max_count {
                    max_count = line.len();
                }
                lines.push(line);
            }
        }
        for line in &mut lines {
            for _ in line.len()..max_count {
                line.push("".into());
            }
        }
        let mut str_lines: Vec<String> = vec![];
        for line in lines {
            str_lines.push(line.join(";"));
        }
        for i in 0..max_scripts_count {
            let id = format!("script_{}_id", i + 1);
            let output = format!("script_{}_output", i + 1);
            column_names.push(id);
            column_names.push(output);
        }
        let column_names = column_names.join(";");
        str_lines.insert(0, column_names);
        return str_lines.join("\r\n");
    }
}
