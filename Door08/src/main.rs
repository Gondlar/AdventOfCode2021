use std::env;
use std::fs;

mod parse;

enum Segment {
    Top = 0,
    TopLeft = 1,
    TopRight = 2,
    Middle = 3,
    BottomLeft = 4,
    BottomRight = 5,
    Bottom = 6
}

#[derive(Debug)]
struct Signal {
    segments : [bool;7]
}

impl std::str::FromStr for Signal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut ret = Signal{segments: [false; 7]};
        for character in s.chars() {
            let index = match character {
                'a' => Ok(Segment::Top),
                'b' => Ok(Segment::TopLeft),
                'c' => Ok(Segment::TopRight),
                'd' => Ok(Segment::Middle),
                'e' => Ok(Segment::BottomLeft),
                'f' => Ok(Segment::BottomRight),
                'g' => Ok(Segment::Bottom),
                _ => Err("Unknown signal")
            }? as usize;
            ret.segments[index] = true;
        }
        return Ok(ret);
    }
}

impl Signal {
    fn count(&self) -> u32{
        self.segments.iter().map(|entry| *entry as u32).sum()
    }

    fn embeds(&self, other: &Signal) -> bool {
        for position in 0..7 {
            if other.segments[position] && !self.segments[position] {
                return false;
            }
        }
        return true;
    }

    fn without(&self, other:&Signal) -> Signal {
        let mut res = Signal{segments: [false; 7]};
        for index in 0..7 {
            if self.segments[index] && !other.segments[index] {
                res.segments[index] = true;
            }
        }
        return res;
    }

    fn equals(&self, other: &Signal) -> bool {
        self.segments.iter()
                     .zip(other.segments.iter())
                     .all(|(lhs, rhs)| lhs==rhs)
    }

    fn is_one(&self) -> bool { self.count() == 2 }
    fn is_seven(&self) -> bool { self.count() == 3 }
    fn is_four(&self) -> bool { self.count() == 4 }
    fn is_eight(&self) -> bool { self.count() == 7 }
    fn is_three(&self, one: &Signal) -> bool { self.count() == 5 && self.embeds(one) }
    fn is_zero(&self, one: &Signal) -> bool { self.count() == 6 && self.embeds(one) }
    fn is_nine(&self, three: &Signal) -> bool { self.count() == 6 && self.embeds(three) }
    fn is_six(&self, one: &Signal) -> bool { self.count() == 6 && !self.embeds(one) }
    fn is_five(&self, six: &Signal) -> bool { self.count() == 5 && six.embeds(self) }

    fn find_signal(list: &mut Vec<Signal>, pred: impl Fn(&Signal) -> bool) -> Signal {
        for index in 0..list.len() {
            if pred(&list[index]) {
                return list.remove(index);
            }
        }
        panic!("Unreachable in {:?}", list);
    }
}

fn parse_row<'b>(cursor : & mut dyn Iterator<Item = &str>)
 -> Result<(Vec<Signal>,Vec<Signal>), &'b str> {
    let mut line = parse::get_next_line(cursor)?.split(" | ");
    let input :Vec<Signal> = parse::parse_list(&mut line, ' ')?;
    let output :Vec<Signal> = parse::parse_list(&mut line, ' ')?;
    return Ok((input, output))
}

fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    let data = parse::parse_all(cursor, parse_row)?;

    let mut part1 = 0;
    let mut part2 = 0;
    for (mut input, output) in data {
        let one   = Signal::find_signal(&mut input, Signal::is_one);
        let seven = Signal::find_signal(&mut input, Signal::is_seven);
        let four  = Signal::find_signal(&mut input, Signal::is_four);
        let eight = Signal::find_signal(&mut input, Signal::is_eight);
        let three = Signal::find_signal(&mut input, |signal| signal.is_three(&one));
        let nine  = Signal::find_signal(&mut input, |signal| signal.is_nine(&three));
        let zero  = Signal::find_signal(&mut input, |signal| signal.is_zero(&one));
        let six   = Signal::find_signal(&mut input, |signal| signal.is_six(&one));
        let five  = Signal::find_signal(&mut input, |signal| signal.is_five(&six));
        assert!(input.len() == 1);
        let two = input.remove(0);
        let decoder = [zero,one,two,three,four,five,six,seven,eight,nine];

        let mut value = 0;
        'outer: for segment in output {
            for digit in 0..10 {
                if segment.equals(&decoder[digit]) {
                    value *= 10;
                    value += digit;
                    if digit == 1 || digit == 4 || digit == 7 || digit == 8 {
                        part1 += 1;
                    }
                    continue 'outer;
                }
            }
            panic!("Unreachable");
        }
        part2 += value;
    }
    println!("Part 1: {}\n Part 2: {}", part1, part2);

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