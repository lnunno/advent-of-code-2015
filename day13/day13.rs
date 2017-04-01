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
}


impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        return self.name == other.name;
    }
}

impl Eq for Person {}

// impl fmt::Debug for Person {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Person {{ x: {}, y: {} }}", self.x, self.y)
//     }
// }

struct Table {
    chairs: Vec<Person>,
}

fn part1() {
    let file = File::open("test-input.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut people: HashMap<String, Person> = HashMap::new();
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
                let person: &mut Person;
                if !people.contains_key(person_name) {
                    people.insert(String::from(person_name),
                                  Person {
                                      name: String::from(person_name),
                                      preferences: new_map,
                                  });
                }
                person = people.get_mut(person_name).unwrap();
                person.update_preferences(neighbor, func, amount);
            }
            None => panic!("No match for '{}'", line),
        }
    }
    println!("{:?}", people);
}

fn main() {
    part1();
}
