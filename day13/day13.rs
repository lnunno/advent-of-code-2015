extern crate regex;
extern crate permutohedron;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::fmt;

use regex::Regex;
use permutohedron::Heap;

#[derive(Debug, Clone)]
struct Person {
    name: String,
    preferences: HashMap<String, isize>,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
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

    /// Calculate the total happiness of sitting by neighbors n1 and n2.
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

#[derive(Debug, Clone)]
struct Table {
    chairs: Vec<Person>,
}

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
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut table = Table { chairs: Vec::with_capacity(10) };

    // Comment this out to get day 01 answer.
    table.chairs.push(Person {
        name: String::from("Me"),
        preferences: HashMap::new(),
    });

    let regex = Regex::new(r"(?P<person>[[:alpha:]]+).*(?P<func>gain|lose) (?P<amount>[[:digit:]]+) happiness units by sitting next to (?P<neighbor>[[:alpha:]]+)\.$").unwrap();
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        let maybe_match = regex.captures(line.as_str());
        match maybe_match {
            Some(capture) => {
                let person_name = capture.name("person").unwrap().as_str();
                let func = capture.name("func").unwrap().as_str();
                let amount = capture.name("amount").unwrap().as_str();
                let neighbor = capture.name("neighbor").unwrap().as_str();
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
    println!("{:?} {}", table, table.total_happiness());

    let mut optimal_happiness = 0;
    let heap = Heap::new(&mut table.chairs);

    for data in heap {
        for p in &data {
            print!("{},", p);
        }
        print!("\n");
        let temp_table = Table { chairs: data };
        let temp_happiness = temp_table.total_happiness();
        if temp_happiness > optimal_happiness {
            optimal_happiness = temp_happiness;
        }
    }

    println!("Optimal happiness={}", optimal_happiness);
}

fn main() {
    part1();
}
