extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::str;
use std::fs::File;
use regex::Regex;

fn to_real_string(s: String) -> String {
    let s = &s[1..s.len() - 1];
    let s_copy = s;
    for mat in Regex::new(r"(?:[^\\]|^)(?P<escape>\\x(?P<hex>\d+))").unwrap().captures_iter(&s) {
        let escape = mat.name("escape").unwrap();
        let hex = mat.name("hex").unwrap();
        // &s_copy[escape.start()..escape.end()] = "\u";
        println!("{:?} {:?}", escape, hex);
    }
    let s = str::replace(s, r"\\", r"\");
    let s = str::replace(&s, r#"\""#, r#"""#);
    let real_string = s;
    return real_string;
}

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        let real_string = to_real_string(line.to_string());
        println!("{:45} Chars in code: {:>8} {:45} Chars in memory: {:>8} ",
                 line,
                 line.len(),
                 real_string,
                 real_string.len());
    }
}

fn main() {
    part1();
}
