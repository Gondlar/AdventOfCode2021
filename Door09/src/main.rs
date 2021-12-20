use std::env;
use std::fs;

mod parse;
mod top;

fn parse_matrix<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<Vec<Vec<u32>>, &'b str> {
    parse::parse_matrix(cursor, |cursor| parse::parse_characters(cursor, |c| {
        match c.to_digit(10) {
            Some(n) => Ok(n),
            None => Err("Digit was no digit")
        }
    }))
}

fn find_low_points(heightmap: &Vec<Vec<u32>>) -> Vec<(usize,usize)> {
    let width = heightmap.len() - 1;
    let height = heightmap[0].len() - 1;
    let mut low_points = vec!();
    for x in 0..=width {
        for y in 0..=height {
            let val = heightmap[x][y];
            if y > 0 && val >= heightmap[x][y-1] {
                continue;
            }
            if y < height && val >= heightmap[x][y+1] {
                continue;
            }
            if x > 0 && val >= heightmap[x-1][y] {
                continue;
            }
            if x < width && val >= heightmap[x+1][y] {
                continue;
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
    println!("Part 1: {}", low_points.iter().map(|(x,y)| risk_level(heightmap[*x][*y])).sum::<u32>());
    let mut visited = vec![vec![false;heightmap[0].len()];heightmap.len()];
    let mut stack :Vec<(usize,usize)> = vec![];
    let mut biggest_basins = top::TopK::new(3);
    for point in &low_points {
        stack.push(*point);
        let mut size = 0;
        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();
            if visited[x][y] || heightmap[x][y] == 9 {
                continue;
            }
            size += 1;
            visited[x][y] = true;
            if x > 0 {
                stack.push((x-1,y));
            }
            if x < heightmap.len()-1 {
                stack.push((x+1,y));
            }
            if y > 0 {
                stack.push((x,y-1));
            }
            if y < heightmap[0].len()-1 {
                stack.push((x,y+1));
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