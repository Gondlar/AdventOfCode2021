use std::env;
use std::fs;
use std::collections::HashSet;

mod matrix;
mod parse;

use matrix::Matrix;

#[derive(Debug)]
enum FoldDirection { Down, Left }

fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    // Parse Input
    let mut points : HashSet<(u32,u32)> = parse::all(cursor, parse::tuple::<u32>)?.into_iter().collect();
    let folds = parse::all(cursor, |line| {
        let line = parse::get_next_line(line)?;
        let mut fold = line.split('=');
        let direction = match fold.next().unwrap() {
            "fold along y" => FoldDirection::Down,
            "fold along x" => FoldDirection::Left,
            _ => { panic!("Unreachable") }
        };
        let coordinate = fold.next().unwrap().parse::<u32>().unwrap();
        return Ok((direction, coordinate));
    })?;

    // Perform Folds
    for (direction, coordinate) in folds {
        points = points.iter().map(|(x,y)| {
            let mut x = *x;
            let mut y = *y;
            match &direction {
                FoldDirection::Left => {
                    assert!(x != coordinate);
                    x = if x > coordinate { coordinate - (x - coordinate) } else { x }
                },
                FoldDirection::Down => {
                    assert!(y != coordinate);
                    y = if y > coordinate { coordinate - (y - coordinate) } else { y }
                }
            }
            return (x,y);
        }).collect();
        println!("{} dots remain after folding {:?} at {}", points.len(), direction, coordinate);
    }

    // Determine size
    let mut width = 0;
    let mut height = 0;
    for (x,y) in &points {
        if *x > width { width = *x }
        if *y > height { height = *y}
    }

    // Print Code
    println!();
    for y in 0..=height {
        for x in 0..=width {
            if points.contains(&(x,y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
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