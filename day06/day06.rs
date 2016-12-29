extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;

type Lights = [[u8; 1000]; 1000];

fn turn<F>(lights: &mut Lights, xstart: u16, ystart: u16, xend: u16, yend: u16, update_function: F)
where F: Fn(u8) -> u8{
    for x in xstart..(xend+1) {
        let ref mut v = lights[x as usize];
        for y in ystart..(yend+1) {
            v[y as usize] = update_function(v[y as usize]);
        }
    }
}

fn turn_on(lights: &mut Lights, xstart: u16, ystart: u16, xend: u16, yend: u16) {
    turn(lights, xstart, ystart, xend, yend, |_: u8| -> u8 { 1 });
}

fn turn_off(lights: &mut Lights, xstart: u16, ystart: u16, xend: u16, yend: u16) {
    turn(lights, xstart, ystart, xend, yend, |_: u8| -> u8 { 0 });
}

fn toggle(lights: &mut Lights, xstart: u16, ystart: u16, xend: u16, yend: u16) {
    turn(lights, xstart, ystart, xend, yend, |x: u8| -> u8 { x ^ 1 });
}

// Get the number of lights that are on.
fn num_on_lights(lights: &Lights) -> u64 {
    let mut i = 0;
    for arr in lights.iter() {
        for value in arr.iter() {
            if (*value) == 1 {
                i += 1;
            }
        }
    }
    return i
}

// Get the total brightness of all the lights.
fn total_brightness(lights: &Lights) -> u64 {
    let mut i = 0;
    for arr in lights.iter() {
        for value in arr.iter() {
            i += (*value);
        }
    }
    return i
}

fn main() {
    let mut lights  = [[0u8; 1000]; 1000];

    let command_regex = Regex::new(r"(?P<action>(?:toggle|(?:turn (?:on|off)))) (?P<xstart>\d+),(?P<ystart>\d+) through (?P<xend>\d+),(?P<yend>\d+)").unwrap();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        let captures = command_regex.captures(&line).unwrap();
        let action = captures.name("action").unwrap();
        let xstart = captures.name("xstart").unwrap().parse::<u16>().unwrap();
        let ystart = captures.name("ystart").unwrap().parse::<u16>().unwrap();
        let xend = captures.name("xend").unwrap().parse::<u16>().unwrap();
        let yend = captures.name("yend").unwrap().parse::<u16>().unwrap();
        match action {
            "turn on" => turn_on(&mut lights, xstart, ystart, xend, yend),
            "turn off" => turn_off(&mut lights, xstart, ystart, xend, yend),
            "toggle" => toggle(&mut lights, xstart, ystart, xend, yend),
            _ => panic!(format!("Can not do action '{}'", action))
        }
    }
    println!("Number of on lights: {}", num_on_lights(&lights));
    println!("Total brightness of lights: {}", total_brightness(&lights));
}
