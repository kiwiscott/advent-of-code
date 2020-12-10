#[macro_use]
extern crate lazy_static;

pub mod machine; 

pub mod common {
    use std::fs::File;
    use std::io::prelude::*;

    pub fn read_file(fname: &str) -> std::io::Result<String> {
        let mut file = File::open(fname)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        return Ok(content);
    }
    pub fn empty_line_split_into_single_line(desc: String) -> Vec<String> {
        let mut ps = Vec::new();

        let mut entry = String::new();
        for line in desc.lines() {
            if line.trim() == "" {
                ps.push(entry);
                entry = String::new();
            } else {
                entry = format!("{} {}", entry, line);
            }
        }
        ps.push(entry);

        ps
    }
}
