//! # Parsenmap
//!
//! `parsenmap` is a tool for parsing nmap xml file to csv or json.
#[macro_use]
mod macroses;
pub mod error;
pub mod models;
use error::*;
use models::{
    address::*, helpers::get_attr_value_by_name, host::*, port::*, scan::*, script::*, service::*,
};
use xml::reader::XmlEvent;
use xml::EventReader;

extern crate xml;

use std::fs::File;
use std::io::BufReader;

/// # Parse nmap xml file
///
/// This function returns ready for data-manipulation Scan object,
/// which contains nmap scan info. 
/// You can use result for your needs or convert it to json or csv file.
pub fn parse(path: &str) -> Result<Scan, ParsenmapError> {
    type PE = ParsenmapError;
    let file = File::open(path);
    match file {
        Ok(file) => {
            let file = BufReader::new(file);
            let parser = EventReader::new(file);
            let mut parent: String = String::new();
            let mut scan = Scan::new();
            let mut hosts = vec![Host::new()];
            for e in parser {
                match e {
                    Ok(XmlEvent::StartElement {
                        name, attributes, ..
                    }) => {
                        let s_name = name.to_string();
                        let is_run = s_name == "nmaprun";
                        let is_host = s_name == "host";
                        let is_port = s_name == "port";
                        let is_parent_run = parent == "nmaprun";
                        let is_parent_host = parent == "host";
                        let is_parent_port = parent == "port";
                        let is_parent_entry = is_run || is_host || is_port;
                        if is_parent_entry {
                            if is_host && parent != "nmaprun" {
                                hosts.push(Host::new());
                            }
                            if is_port {
                                let new_port = Port::from(&attributes);
                                hosts.last_mut().unwrap().ports.push(new_port);
                            }
                            if is_run {
                                scan.from_reparent(&attributes);
                            }
                            parent = s_name.clone();
                        }
                        let last_host = hosts.last_mut().unwrap();
                        if is_parent_run {
                            if s_name == "verbose" {
                                scan.verbose = get_attr_value_by_name(&attributes, "level");
                            }
                            if s_name == "debugging" {
                                scan.debugging = get_attr_value_by_name(&attributes, "level");
                            }
                        }
                        if is_parent_host {
                            if s_name == "status" {
                                last_host.state = get_attr_value_by_name(&attributes, "state");
                            }
                            if s_name == "address" {
                                let addr = Address::from(&attributes);
                                last_host.addrs.push(addr);
                            }
                        }
                        if is_parent_port {
                            if last_host.ports.last().is_some() {
                                let last_port = last_host.ports.last_mut().unwrap();
                                if s_name == "state" {
                                    last_port.state = get_attr_value_by_name(&attributes, "state");
                                }
                                if s_name == "service" {
                                    last_port.service = Service::from(&attributes);
                                }
                                if s_name == "script" {
                                    let script = Script::from(&attributes);
                                    last_port.scripts.push(script);
                                }
                            }
                        }
                    }
                    Err(e) => return Err(PE::from(e)),
                    _ => {}
                }
            }
            scan.hosts = hosts;
            Ok(scan)
        }
        Err(err) => return Err(PE::from(err)),
    }
}
