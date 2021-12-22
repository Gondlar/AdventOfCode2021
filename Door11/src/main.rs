use std::env;
use std::fs;

mod matrix;
mod parse;

use matrix::Matrix;

fn parse_matrix<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<Matrix<u32>, &'b str> {
    parse::parse_matrix(cursor, |cursor| parse::parse_characters(cursor, |c| {
        match c.to_digit(10) {
            Some(n) => Ok(n),
            None => Err("Digit was no digit")
        }
    }))
}

static MAX : u32 = 10;

fn round(energylevel: &mut Matrix<u32>) -> u32 {
    let mut flashes = 0;
    // Step 1
    for coords in energylevel.coords_iter() {
        energylevel[coords] += 1;
    }
    // Step 2
    let mut will_flash = vec![];
    for coords in energylevel.coords_iter() {
        if energylevel[coords] == MAX {
            will_flash.push(coords);
        }
    }
    while !will_flash.is_empty() {
        let (x, y) = will_flash.pop().unwrap();
        flashes += 1;
        for coords in energylevel.around_coords(x, y) {
            energylevel[coords] += 1;
            if energylevel[coords] == MAX { // i.e. it is now 9
                will_flash.push(coords);
            }
        }
    }
    // Step 3
    for coords in energylevel.coords_iter() {
        if energylevel[coords] >= MAX {
            energylevel[coords] = 0;
        }
    }
    return flashes;
}

fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    let mut energylevel = parse_matrix(cursor)?;
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += round(&mut energylevel);
    }
    println!("Total Flashes: {}", total_flashes);
    
    let mut rounds = 101;
    while round(&mut energylevel) < 100 {
        rounds += 1;
    }
    println!("Synchronized in Round {}", rounds);

    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went terribly wrong while reading the file!");
    let mut iter = contents.split("\n");
    match do_work(& mut iter) {
        Ok(_) => {},
        Err(msg) => { println!("Whoops: {}", msg) }
    }
}