#[derive(Debug)]
pub struct Bingo {
    numbers : Vec<Vec<u32>>,
    called : Vec<Vec<bool>>
}

impl Bingo {
    pub fn new<'b>(numbers : Vec<Vec<u32>>) -> Result<Bingo, &'b str> {
        let size = numbers[0].len();
        if numbers.len() != size {
            return Err("Bingo has invalid row count")
        }
        for line in &numbers {
            if line.len() != size {
                return Err("Bingo row has invalid column count");
            }
        }
        let called = vec![vec![false; size]; size];
        return Ok(Bingo{numbers : numbers, called : called});
    }

    fn size(&self) -> usize {
        return self.numbers.len();
    }

    pub fn call(& mut self, number : u32) {
        let size = self.size();
        for row in 0..size {
            for col in 0..size {
                if self.numbers[row][col] == number {
                    self.called[row][col] = true;
                }
            }
        }
    }

    fn is_row_complete(&self, row : usize) -> bool {
        for col in 0..self.size() {
            if !self.called[row][col] {
                return false;
            }
        }
        return true;
    }

    fn is_column_complete(&self, col : usize) -> bool {
        for row in 0..self.size() {
            if !self.called[row][col] {
                return false;
            }
        }
        return true;
    }

    pub fn is_complete(&self) -> bool {
        for index in 0..self.size() {
            if self.is_row_complete(index) {
                return true;
            }
            if self.is_column_complete(index) {
                return true;
            }
        }
        return false;
    }

    pub fn score(&self) -> u32 {
        let mut result = 0;
        for row in 0..self.size() {
            for col in 0..self.size() {
                if !self.called[row][col] {
                    result += self.numbers[row][col];
                }
            }
        }
        return result;
    }
}