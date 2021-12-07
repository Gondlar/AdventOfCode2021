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

    fn add(&mut self, bits: Bits) {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    //let use_aim : bool = args[2].parse::<bool>().unwrap_or(false);

    let contents = fs::read_to_string(filename)
        .expect("Something went terribly wrong while reading the file!");
    let mut diag : Diagnostic = Diagnostic::new();
    contents.split("\n")
            .map(|line| line.parse::<Bits>())
            .filter(|line| line.is_ok())
            .for_each(|line| {
                let bits = line.unwrap();
                diag.add(bits);
            });
    println!("{}", u32::from(&diag.epsilon())* u32::from(&diag.gamma()));
}