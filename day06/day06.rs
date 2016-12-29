use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

type Lights = [[u8; 1000]; 1000];

fn turn<F>(lights: &mut Lights, xstart: u16, ystart: u16, xend: u16, yend: u16, updateFunction: F)
where F: Fn(u8) -> u8{
    for x in xstart..(xend+1) {
        let ref mut v = lights[x as usize];
        for mut y in ystart..(yend+1) {
            v[y as usize] = 1;
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

fn main() {
    let mut lights  = [[0u8; 1000]; 1000];
    turn_on(&mut lights, 0, 0, 0, 0);
    println!("{:?}", lights[0][0]);
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let l = line.unwrap();
        // println!("{}", l);
    }
}
