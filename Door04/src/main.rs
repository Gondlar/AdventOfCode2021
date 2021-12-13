use std::env;
use std::fs;

mod parse;
mod bingo;

fn parse_bingo<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<bingo::Bingo, &'b str> {
    let first_line : Vec<u32> = parse::parse_list(cursor, ' ')?;
    let size = first_line.len();
    let mut other_lines : Vec<Vec<u32>> = parse::parse_n(cursor, size-1, |cur| parse::parse_list(cur, ' '))?;
    parse::parse_empty(cursor)?;
    let mut bingo = vec!(first_line);
    bingo.append(& mut other_lines);
    return bingo::Bingo::new(bingo);
}

fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    let numbers : Vec<u32> = parse::parse_list(cursor, ',')?;
    let mut first : Option<u32> = None;
    parse::parse_empty(cursor)?;
    let mut bingos = parse::parse_all(cursor, parse_bingo)?;
    let mut last : bool = bingos.len() == 1;
    for number in numbers {
        let mut remaining_bingos = vec!();
        for mut bingo in bingos {
            bingo.call(number);
            if bingo.is_complete() {
                if first.is_none() {
                    first = Some(bingo.score() * number);
                }
                if last {
                    println!("First: {}\nLast: {}", first.unwrap(), bingo.score() * number);
                    return Ok(());
                }
            } else {
                remaining_bingos.push(bingo);
            }
        }
        if remaining_bingos.len() == 1 {
            last = true;
        }
        bingos = remaining_bingos;
    }
    return Err("unreachable");
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