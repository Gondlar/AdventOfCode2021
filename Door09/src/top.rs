#[derive(Debug)]
pub struct TopK {
    contents : Vec<u32>,
    max : usize
}

impl TopK {
    pub fn new(k: usize) -> TopK {
        return TopK{contents: vec![], max: k}
    }

    pub fn push(&mut self, val: u32) {
        let len = self.contents.len();
        if len < self.max {
            self.contents.push(val);
        } else if val > self.contents[len-1] {
            self.contents[len-1] = val;
        } else {
            return;
        }
        // We inserted something, we need to sort
        for index in (1..self.contents.len()).rev() {
            if self.contents[index] > self.contents[index-1] {
                self.contents.swap(index, index-1);
            } else {
                break;
            }
        }
    }

    pub fn iter(&self) -> std::slice::Iter<u32> {
        return self.contents.iter();
    }
}