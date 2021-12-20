use std::env;
use std::fs;

mod parse;

fn find_corresponding_closing_bracket(ch: char) -> Option<char> {
    let brackets = [('(',')'), ('[',']'), ('{','}'), ('<','>')];
    for (open, close) in brackets {
        if ch == open {
            return Some(close);
        }
    }
    None
}

fn error_score(ch: char) -> u32 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => {
            panic!("Unknown char");
        }
    }
}

fn complete_score(ch: char) -> u64 {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => {
            panic!("Unknown char");
        }
    }
}

fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    let mut total_error_score = 0;
    let mut complete_scores = vec!();
    'line: for line in cursor {
        let mut stack = vec![];
        for ch in line.chars() {
            match find_corresponding_closing_bracket(ch) {
                Some(close) => {
                    stack.push(close);
                },
                None => {
                    let next =stack.pop();
                    if next.is_none() || next.unwrap() != ch {
                        total_error_score += error_score(ch);
                        continue 'line;
                    }
                }
            }
        }
        let mut my_complete_score = 0;
        for ch in stack.iter().rev() {
            my_complete_score *= 5;
            my_complete_score += complete_score(*ch);
        }
        complete_scores.push(my_complete_score);
    }
    complete_scores.sort();
    println!("Part 1: {}\nPart 2: {}", total_error_score, complete_scores[complete_scores.len()/2]);
    
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
