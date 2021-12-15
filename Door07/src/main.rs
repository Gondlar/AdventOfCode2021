use std::env;
use std::fs;

mod parse;

fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    let crabs : Vec<usize> = parse::parse_list(cursor, ',')?;
    let width = crabs.iter().max().unwrap();
    let mut area = vec![0; *width+1];
    for crab in &crabs {
        area[*crab] += 1;
    }
    let mut fuel = 0;
    let mut left = 0;
    let mut right = *width;
    while left != right {
        if area[left] < area[right] {
            let old = area[left];
            left += 1;
            area[left] += old;
            fuel += old;
        } else {
            let old = area[right];
            right -= 1;
            area[right] += old;
            fuel += old;
        }
    }
    println!("End position: {}\nFuel used: {}", left, fuel);
    let mut minfuel = usize::MAX;
    let mut minposition = 0; 
    for position in 0..=*width {
        let mut fuel = 0;
        for crab in &crabs {
            let distance = position + *crab - 2*std::cmp::min(position, *crab);
            fuel += (distance + distance*distance)/2;
        }
        if fuel < minfuel {
            minfuel = fuel;
            minposition = position;
        }
    }
    println!("End position: {}\nFuel used: {}", minposition, minfuel);
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