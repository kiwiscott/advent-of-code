use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_all_lines(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let v: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    return v;
}

pub struct FileSplitter {
    reader: BufReader<File>,
}

impl FileSplitter {
    pub fn new(name: String) -> FileSplitter {
        let file = File::open(name).unwrap();
        let reader = BufReader::new(file);

        FileSplitter { reader }
    }
    fn process(&self, buf: Vec<u8>) -> Option<String> {
        if buf.len() != 0 {
            let buf = String::from_utf8(buf.to_vec()).unwrap();
            Some(buf)
        } else {
            None
        }
    }
}

impl Iterator for FileSplitter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut buf = Vec::new();
        let ret;
        let mut last_len = 0;

        loop {
            match self.reader.read_until(b'\n', &mut buf) {
                Ok(0) => {
                    ret = self.process(buf);
                    break;
                }
                Ok(_n) => {
                    //Trim
                    if buf.ends_with(&[10]) {
                        buf.pop();
                        if buf.ends_with(&[13]) {
                            buf.pop();
                        }
                    }

                    if buf.len() != last_len {
                        buf.push(32);
                        last_len = buf.len();
                    } else {
                        ret = self.process(buf);
                        break;
                    }
                }
                Err(_e) => {
                    ret = self.process(buf);
                    break;
                }
            };
        }

        ret
    }
}
