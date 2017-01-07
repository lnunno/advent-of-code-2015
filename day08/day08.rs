extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::str;
use std::fs::File;
use regex::Regex;

fn remove_substring(s: String, start: usize, end: usize) -> String {
    let mut new_str = String::new();
    for (i, c) in s.chars().enumerate() {
        if (i >= start) && (i <= end) {
            // Don't add this character.
            continue;
        } else {
            new_str.push(c);
        }
    }
    return new_str;
}

fn to_real_string(s: String) -> String {
    let s = &s[1..s.len() - 1];
    let mut s_copy = s.to_string().clone();
    let hex_regex = Regex::new(r"(?:[^\\]|^)(?P<escape>\\x(?P<hex>[[:xdigit:]][[:xdigit:]]))")
        .unwrap();
    loop {
        let regex_str_copy = &s_copy.clone();
        let maybe_match = hex_regex.captures(regex_str_copy);
        match maybe_match {
            Some(capture) => {
                let escape = capture.name("escape").unwrap();
                let hex = capture.name("hex").unwrap();
                s_copy = remove_substring(s_copy.clone(), escape.start(), escape.end() - 1);
                let expanded_ascii_char = 'R'; // u8::from_str_radix(hex.as_str(), 16).unwrap() as char;
                s_copy.insert(escape.start(), expanded_ascii_char);
            }
            None => break,
        }
    }
    s_copy = s_copy.replace(r"\\", r"\");
    s_copy = s_copy.replace(r#"\""#, r#"""#);
    let real_string = s_copy;
    return real_string;
}

fn part1() {
    let file = File::open("test-input.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut num_chars_code = 0;
    let mut num_chars_memory = 0;
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        let real_string = to_real_string(line.to_string());
        println!("{:45} Chars in code: {:>8} {:45} Chars in memory: {:>8} ",
                 line,
                 line.len(),
                 real_string,
                 real_string.len());
        num_chars_code += line.len();
        num_chars_memory += real_string.len();
    }
    println!("Answer #1={} - {} = {}",
             num_chars_code,
             num_chars_memory,
             num_chars_code - num_chars_memory);
}

fn main() {
    part1();
}
