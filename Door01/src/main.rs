use std::env;
use std::fs;

// Instead of providing two separate solutions, we solve the more general problem of windows of size
// n. Then, the solution for Part 1 is n=1 and part 2 is n=3.

fn main() {
    // Read the input. We expect two parameters: the input file and the window size
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let window_size = args[2].parse::<usize>().unwrap_or(1);

    // Read the file from disk
    let contents = fs::read_to_string(filename)
        .expect("Something went terribly wrong while reading the file!");
    let mut lines = contents.split("\n")
                    .map(|line| line.parse::<i32>())
                    .filter(|line| line.is_ok())
                    .map(|line| line.unwrap());

    // Initialize vec with first n numbers from the file
    let mut window: Vec<i32> = vec![];
    window.reserve(window_size);
    for _ in 0..window_size {
        let first = lines.next();
        if first.is_none() {
            println!("0");
            return;
        }
        window.push(first.unwrap());
    }

    // Process the remaining lines
    //
    // Represent the window as a ringbuffer of constant size. With every new line, one value leaves
    // the window and one value is added to the window. Therefore, the sum over the window increases
    // iff the new number is larger than the removed one.
    let mut count = 0;
    let mut results = 0;
    for line in lines {
        let pos = count % window_size;
        if line > window[pos] {
            results += 1;
        }
        window[pos] = line;
        count += 1;
    }
    println!("{}", results);
}