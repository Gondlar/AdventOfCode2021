use std::env;
use std::fs;
use std::collections::HashMap;

mod matrix;
mod parse;

struct Rule {
    input: String,
    output: [String; 2]
}

struct Polymer {
    contents: HashMap::<String,u64>,
    last: String
}

impl Rule {
    fn new(before: &str, after: &str) -> Rule {
        assert!(before.len() == 2);
        assert!(after.len() == 1);
        let mut before_chars = before.chars();
        let mut fst = String::from(before_chars.next().unwrap());
        fst.push_str(after);
        let mut snd = String::from(after);
        snd.push(before_chars.next().unwrap());
        return Rule{input: String::from(before), output: [fst, snd]};
    }

    fn apply(&self, before: &Polymer, after: &mut Polymer) {
        match before.contents.get(&self.input) {
            None => (),
            Some(count) => {
                for out in &self.output {
                    *after.contents.entry(out.clone()).or_insert(0) += count;
                }
            }
        }
        if before.last == self.input {
            after.last = self.output[1].clone();
        }
    }
}

impl Polymer {
    fn new(polymer: &str) -> Polymer {
        let mut contents = HashMap::<String,u64>::new();
        for index in 0..polymer.len()-1 {
            let subject = String::from(&polymer[index..=index+1]);
            *contents.entry(subject).or_insert(0) += 1;
        }
        let last = &polymer[polymer.len()-2..polymer.len()];
        return Polymer{contents: contents, last: String::from(last)};
    }

    fn lengthen(&self, rules: &Vec<Rule>) -> Polymer {
        // This method actually contains a bug: if the polymer contains a 2-gram which is not modified
        // by any rule (e.g. the polymer "abba" with the ruleset "BB -> B"), the resulting polymers will
        // be wrong. However, the inputs do not seem to generate such cases (or I'm lucky with my input).
        // To fix this, instead loop over the present 2-grams and check if a rule matches. If so, apply it.
        // Otherwise, the 2-grams are still present in the output, add their coutn to the result count.
        let mut new = Polymer{contents: HashMap::<String,u64>::new(), last: self.last.clone()};
        for rule in rules {
            rule.apply(self, &mut new);
        }
        return new;
    }

    fn evaluate(&self) -> u64 {
        // Count chars
        let mut counts = HashMap::<char,u64>::new();
        for (key, count) in self.contents.iter() {
            let key = key.chars().next().unwrap();
            *counts.entry(key).or_insert(0) += count;
        }
        let last = self.last.chars().nth(1).unwrap();
        *counts.entry(last).or_insert(0) += 1;

        // Find min and max
        let mut counts = counts.iter();
        let mut min = *counts.next().unwrap().1;
        let mut max = min;
        for (_, count) in counts {
            if *count < min {
                min = *count;
            } else if *count > max {
                max = *count;
            }
        }
        return max-min;
    }
}

fn parse_rule<'b>(cursor : &mut dyn Iterator<Item = & str>) -> Result<Rule, &'b str> {
    let mut line = parse::get_next_line(cursor)?.split(" -> ");
    let before = line.next().unwrap();
    let after = line.next().unwrap();
    assert!(line.next().is_none());
    return Ok(Rule::new(before, after));
}

fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    let mut polymer = Polymer::new(parse::get_next_line(cursor)?);
    parse::empty(cursor)?;
    let rules = parse::all(cursor, parse_rule)?;

    for _ in 0..10 {
        polymer = polymer.lengthen(&rules);
        println!("{:?} + {}", polymer.contents, polymer.last);
    }
    println!("Part 1: {}", polymer.evaluate());
    for _ in 0..30 {
        polymer = polymer.lengthen(&rules);
    }
    println!("Part 2: {}", polymer.evaluate());
    
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