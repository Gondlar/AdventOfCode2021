use std::env;
use std::fs;

mod matrix;
mod parse;
mod top;

use matrix::Matrix;

fn parse_matrix<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<Matrix<u32>, &'b str> {
    parse::parse_matrix(cursor, |cursor| parse::parse_characters(cursor, |c| {
        match c.to_digit(10) {
            Some(n) => Ok(n),
            None => Err("Digit was no digit")
        }
    }))
}

fn find_low_points(heightmap: &Matrix<u32>) -> Vec<(usize,usize)> {
    let mut low_points = vec!();
    for x in 0..heightmap.get_width() {
        'fields: for y in 0..heightmap.get_height() {
            let &val = heightmap.get(x, y);
            for &neighbor in heightmap.get_all(&mut heightmap.neighbor_coords(x, y)) {
                if neighbor <= val {
                    continue 'fields
                }
            }
            low_points.push((x, y));
        }
    }
    return low_points;
}

fn risk_level(height: u32) -> u32 { height +1 }

fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    let heightmap = parse_matrix(cursor)?;
    let low_points = find_low_points(&heightmap);
    println!("Part 1: {}", low_points.iter().map(|(x,y)| risk_level(*heightmap.get(*x, *y))).sum::<u32>());
    let mut visited = Matrix::<bool>::new(heightmap.get_width(), heightmap.get_height());
    let mut stack :Vec<(usize,usize)> = vec![];
    let mut biggest_basins = top::TopK::new(3);
    for point in low_points {
        stack.push(point);
        let mut size = 0;
        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();
            if *visited.get(x, y) || *heightmap.get(x, y) == 9 {
                continue;
            }
            size += 1;
            visited.set(x, y,true);
            for coords in heightmap.neighbor_coords(x, y) {
                stack.push(coords);
            }
        }
        biggest_basins.push(size);
    }
    println!("Part 2: {} <- {:?}", biggest_basins.iter().product::<u32>(), biggest_basins);
    
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