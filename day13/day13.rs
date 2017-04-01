extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::str;
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    preferences: HashMap<String, isize>,
}

impl Person {
    fn update_preferences(&mut self, other_person_name: &str, func: &str, amount: &str) {
        let value: isize = amount.parse::<isize>().unwrap();
        match func {
            "gain" => {
                self.preferences.insert(String::from(other_person_name), value);
                return;
            }
            "lose" => {
                self.preferences.insert(String::from(other_person_name), -value);
                return;
            }
            s => {
                panic!("Unknown func '{}'", s);
            }
        }
    }

    /**
    * Calculate the total happiness of sitting by neighbors n1 and n2.
    */
    fn total_happiness(&self, n1: &Person, n2: &Person) -> isize {
        let pn1 = match self.preferences.get(n1.name.as_str()) {
            Some(v) => *v,
            None => 0,
        };
        let pn2 = match self.preferences.get(n2.name.as_str()) {
            Some(v) => *v,
            None => 0,
        };
        return pn1 + pn2;
    }
}

#[derive(Debug)]
struct Table {
    chairs: Vec<Person>,
}


// impl fmt::Debug for Table {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Table {{ chairs: }}")
//     }
// }

impl Table {
    fn contains_name(&self, name: &str) -> bool {
        for p in &(self.chairs) {
            if p.name == name {
                return true;
            }
        }
        return false;
    }

    fn get_name(&mut self, name: &str) -> Option<&mut Person> {
        for p in &mut (self.chairs) {
            if p.name == name {
                return Some(p);
            }
        }
        return None;
    }

    fn neighbors(&self, index: usize) -> (&Person, &Person) {
        if index == 0 {
            return (&self.chairs[&self.chairs.len() - 1], &self.chairs[1]);
        } else if index == (self.chairs.len() - 1) {
            return (&self.chairs[index - 1], &self.chairs[0]);
        } else {
            return (&self.chairs[index - 1], &self.chairs[index + 1]);
        }
    }

    fn total_happiness(&self) -> isize {
        let mut happiness: isize = 0;
        for (i, p) in self.chairs.iter().enumerate() {
            let (n1, n2) = self.neighbors(i);
            happiness += p.total_happiness(n1, n2);
        }
        return happiness;
    }
}

fn part1() {
    let file = File::open("test-input.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut table = Table { chairs: Vec::with_capacity(10) };
    let regex = Regex::new(r"(?P<person>[[:alpha:]]+).*(?P<func>gain|lose) (?P<amount>[[:digit:]]+) happiness units by sitting next to (?P<neighbor>[[:alpha:]]+)\.$").unwrap();
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        println!("{} ", line);
        let maybe_match = regex.captures(line.as_str());
        match maybe_match {
            Some(capture) => {
                let person_name = capture.name("person").unwrap().as_str();
                let func = capture.name("func").unwrap().as_str();
                let amount = capture.name("amount").unwrap().as_str();
                let neighbor = capture.name("neighbor").unwrap().as_str();
                println!("{:?}", capture);
                let new_map: HashMap<String, isize> = HashMap::new();
                if !table.contains_name(person_name) {
                    let person = Person {
                        name: String::from(person_name),
                        preferences: new_map,
                    };
                    table.chairs.push(person);
                }
                let mut person = table.get_name(person_name).unwrap();
                person.update_preferences(neighbor, func, amount);
            }
            None => panic!("No match for '{}'", line),
        }
    }
    println!("{:?}", table);
}

fn main() {
    part1();
}
