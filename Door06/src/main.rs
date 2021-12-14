use std::env;
use std::fs;

mod parse;
mod population;



fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    let mut population = population::Population::empty(7, 2);
    for age in parse::parse_list(cursor, ',')? {
        population.put(age);
    }
    for _ in 0..80 {
        population.cycle();
    }
    println!("After 80 Generations: {}", population.count());
    for _ in 0..(256-80) {
        population.cycle();
    }
    println!("After 256 Generations: {}", population.count());
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