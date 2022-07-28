# Parsenmap

This is a tool for parsing nmap xml file to csv or json.

**USAGE:** <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`$ parsenmap <from> <to> -f <filetype>`

**FLAGS:** <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-h, --help - Prints help information <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-V, --version - Prints version information <br/>

**OPTIONS:** <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-f {filetype} - Output file format csv or json

**ARGS:** <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;{from} - Nmap xml file path <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;{to} - Output file path

**EXAMPLE:** <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`$ parsenmap nmap.xml nmap.csv -f csv`

**IN-CODE USAGE EXAMPLE:** <br/>

```rust
use parsenmap::models::scan::FileType;

fn main() {
    let scan:Scan = parsenmap::parse("nmap.xml").unwrap();
    scan.write_to_file(FileType::CSV, "nmap.csv".unwrap());
    let json:String =  scan.to_json();
    let csv:String =  scan.to_csv();
}
```
