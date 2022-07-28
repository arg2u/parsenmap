use std::path::PathBuf;
use std::process::exit;

use parsenmap::models::scan::FileType;
use structopt;
use structopt::StructOpt;

#[allow(unused)]
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Parsenmap",
    about = "This is a tool for parsing nmap xml file to csv or json."
)]
struct Opt {
    /// Output file format csv or json
    #[structopt(short)]
    filetype: String,
    /// Nmap xml file path
    #[structopt(parse(from_os_str))]
    from: PathBuf,
    /// Output file path
    #[structopt(parse(from_os_str))]
    to: PathBuf,
}
fn main() {
    let opt = Opt::from_args();
    let scan = parsenmap::parse(opt.from.to_str().unwrap()).unwrap();
    let f_type;
    if opt.filetype.to_lowercase().contains("csv") {
        f_type = FileType::CSV;
    } else if opt.filetype.to_lowercase().contains("json") {
        f_type = FileType::JSON;
    } else {
        eprintln!("Incorrect file type. You can pass csv or json file types");
        exit(1);
    }
    scan.write_to_file(f_type, opt.to.to_str().unwrap());
}
