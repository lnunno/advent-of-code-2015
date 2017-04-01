extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use regex::Regex;

fn main() {
    let file = File::open("test-input.txt").unwrap();
    let reader = BufReader::new(&file);

    let regex = Regex::new(r".*").unwrap();
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        let maybe_match = regex.captures(line.as_str());
        match maybe_match {
            Some(capture) => {}
            None => panic!("No match for '{}'", line),
        }

    }
}
