extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use regex::Regex;

#[derive(Debug, Clone)]
struct Reindeer {
    name: String,
    speed: isize,
    speed_time: isize,
    rest_time: isize,
    current_distance: isize,
    points: isize,
    rest_time_left: isize,
    speed_time_left: isize,
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

    /// Move the race forward one second.
    fn tick(&mut self) {
        if self.speed_time_left > 0 {
            self.current_distance += self.speed;
            self.speed_time_left -= 1;
            if self.speed_time_left == 0 {
                self.rest_time_left = self.rest_time;
            }
        } else {
            self.rest_time_left -= 1;
            if self.rest_time_left == 0 {
                self.speed_time_left = self.speed_time;
            }
        }
    }
}

fn award_points(reindeers: &mut Vec<Reindeer>) {
    let mut max_distance = 0;
    for r in reindeers.iter() {
        if r.current_distance > max_distance {
            max_distance = r.current_distance;
        }
    }

    for r in reindeers.iter_mut() {
        if r.current_distance >= max_distance {
            r.points += 1;
        }
    }
}

fn part1(reindeers: Vec<Reindeer>) {
    let time_in_secs = 2503;
    for r in reindeers.iter() {
        println!("{:10} traveled {:4} km in {:5} seconds",
                 r.name,
                 r.distance_traveled(time_in_secs),
                 time_in_secs);
    }
}

fn part2(reindeers: Vec<Reindeer>) {
    let mut reindeers = reindeers;
    let time_in_secs = 2503;
    for _ in 0..time_in_secs {
        for r in reindeers.iter_mut() {
            r.tick();
        }

        // Find the winning reindeer and award them a point.
        award_points(&mut reindeers);
    }
    for r in reindeers.iter() {
        println!("{:10} traveled {:4} km in {:5} seconds and received {:5} points.",
                 r.name,
                 r.distance_traveled(time_in_secs),
                 time_in_secs,
                 r.points);
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
                    current_distance: 0,
                    points: 0,
                    rest_time_left: 0,
                    speed_time_left: speed_time,
                });

            }
            None => panic!("No match for '{}'", line),
        }
    }
    part1(reindeers.clone());
    part2(reindeers);
}
