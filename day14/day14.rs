extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use regex::Regex;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: isize,
    speed_time: isize,
    rest_time: isize,
}

impl Reindeer {
    fn distance_traveled(&self, seconds: isize) -> isize {
        let mut distance = 0;
        let mut seconds = seconds;
        while seconds > 0 {
            if seconds > self.speed_time {
                distance += self.speed * self.speed_time;
                seconds -= self.speed_time;
            } else {
                distance += self.speed * seconds;
                break;
            }
            seconds -= self.rest_time;
        }
        return distance;
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);

    let regex = Regex::new(r"(?P<name>[[:alpha:]]+) can fly (?P<speed>[[:digit:]]+) km/s for (?P<speed_time>[[:digit:]]+) seconds, but then must rest for (?P<rest_time>[[:digit:]]+) seconds.").unwrap();
    let mut reindeers: Vec<Reindeer> = Vec::with_capacity(10);
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        let maybe_match = regex.captures(line.as_str());
        match maybe_match {
            Some(capture) => {
                let name = capture.name("name").unwrap().as_str();
                let speed = capture.name("speed").unwrap().as_str().parse::<isize>().unwrap();
                let speed_time =
                    capture.name("speed_time").unwrap().as_str().parse::<isize>().unwrap();
                let rest_time =
                    capture.name("rest_time").unwrap().as_str().parse::<isize>().unwrap();
                reindeers.push(Reindeer {
                    name: String::from(name),
                    speed: speed,
                    speed_time: speed_time,
                    rest_time: rest_time,
                });

            }
            None => panic!("No match for '{}'", line),
        }
    }
    let time_in_secs = 2503;
    for r in reindeers {
        println!("{:10} traveled {:4} km in {:5} seconds",
                 r.name,
                 r.distance_traveled(time_in_secs),
                 time_in_secs);
    }
}
