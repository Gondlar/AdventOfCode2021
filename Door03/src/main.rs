use std::ops::Index;
use std::str::FromStr;
use std::env;
use std::fs;

struct Bits {
    bs : Vec<bool>
}

#[derive(Debug)]
struct BitstringParseError {}

impl FromStr for Bits {
    type Err = BitstringParseError;

    fn from_str (bs : &str) -> Result<Bits,BitstringParseError> {
        let bits : Result<Vec<bool>, BitstringParseError>
            = bs.chars()
                .map(|ch| match ch {
                    '0' => Ok(false),
                    '1' => Ok(true),
                    _ => Err(BitstringParseError{})
                })
                .collect();
        bits.map(|vec| Bits{bs : vec})
    }
}

impl From<&Bits> for u32 {
    fn from (bs : &Bits) -> u32 {
        let mut result = 0;
        for bit in bs.bs.iter() {
            result = result * 2 + (*bit) as u32;
        }
        println!("{:?} -> {}", bs.bs, result);
        return result;
    }
}

impl Index<usize> for Bits {
    type Output = bool;

    fn index(&self, index : usize) -> &Self::Output {
        return &self.bs[index];
    }
}

impl Bits {
    fn len(&self) -> usize {
        return self.bs.len();
    }
}

struct Diagnostic {
    total : u32,
    counts : Vec<u32>
}

impl Diagnostic {
    fn new() -> Diagnostic {
        Diagnostic{total: 0, counts: vec!()}
    }

    fn add(&mut self, bits: &Bits) {
        let len = bits.len();
        if len > self.counts.len() {
            self.counts.resize(len, 0);
        }
        for i in 0..len {
            self.counts[i] += bits[i] as u32;
        }
        self.total += 1;
    }

    fn gamma(&self) -> Bits {
        let threshold = self.total / 2;
        let bits = self.counts.iter()
                              .map(|count| *count > threshold)
                              .collect();
        return Bits{bs : bits};
    }

    fn epsilon(&self) -> Bits {
        let threshold = self.total / 2;
        let bits = self.counts.iter()
                              .map(|count| *count <= threshold)
                              .collect();
        return Bits{bs : bits};
    }
}

fn oxy_rating(ones : usize, zeros : usize) -> bool {
    return ones >= zeros
}

fn co2_rating(ones : usize, zeros : usize) -> bool {
    return ones < zeros
}

fn ls_rating(data : Vec<&Bits>, digit : usize, pred: fn(usize, usize) -> bool) -> &Bits {
    let remaining = data.len();
    if remaining == 0 {
        panic!("No Bitstrings left. This shouldn't happen");
    } else if remaining == 1 {
        return data[0];
    }
    if data[0].len() <= digit {
        panic!("Multiple Bitstrings left when we're out of bits. This shouldn't happen.");
    }
    let ones = data.iter()
                         .filter(|bits| bits[digit])
                         .count();
    let target_value = pred(ones, remaining - ones);
    return ls_rating(data.into_iter()
                              .filter(|bits| bits[digit] == target_value)
                              .collect(),
                     digit+1, pred);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went terribly wrong while reading the file!");
    let mut diag : Diagnostic = Diagnostic::new();
    let bits : Vec<Bits> = contents.split("\n")
                       .map(|line| line.parse::<Bits>())
                       .filter(|line| line.is_ok())
                       .map(|line| line.unwrap())
                       .filter(|line| line.len() > 0)
                       .collect();
    bits.iter().for_each(|entry| diag.add(entry));
    println!("{}", u32::from(&diag.epsilon())* u32::from(&diag.gamma()));
    let oxy = u32::from(ls_rating(bits.iter().collect(), 0, oxy_rating));
    let co2 = u32::from(ls_rating(bits.iter().collect(), 0, co2_rating));
    println!("Oxy: {} / CO2: {}\n{}", oxy, co2, oxy*co2);
}