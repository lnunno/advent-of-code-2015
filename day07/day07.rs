extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

fn part1(){
    let signal_regex = Regex::new(r"(?P<signal>\d+) -> (?P<wire_name>[a-z]+)").unwrap();
    let combine_regex = Regex::new(r"(?P<input_a>[a-z]+) (?P<method> (?:AND|OR)) (?P<input_b>[a-z]+) -> (?P<output>[a-z]+)").unwrap();
    let shift_regex = Regex::new(r"(?P<wire_name>[a-z]+) (?P<direction> (?:L|R))SHIFT (?P<amount>\d+) -> (?P<output>[a-z]+)").unwrap();
    let not_regex = Regex::new(r"NOT (?P<wire_name>[a-z]+) -> (?P<output>[a-z]+)").unwrap();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        println!("{}", line);
    }
}

fn main() {
    part1();
}
