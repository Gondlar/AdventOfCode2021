pub struct Population {
    age_groups : Vec<u64>,
    puberty : usize
}

impl Population {
    pub fn empty(period: usize, puberty: usize) -> Population {
        Population{age_groups: vec![0; puberty+period], puberty: puberty}
    }

    pub fn put(&mut self, age: usize) {
        self.age_groups[age] += 1;
    }

    pub fn count(&self) -> u64 {
        self.age_groups.iter().sum()
    }

    pub fn cycle(&mut self) {
        let len = self.age_groups.len() - 1;
        let mut tmp = 0;
        // Move all age groups one up, inserting 0 into the last
        for index in (0..self.age_groups.len()).rev() {
            std::mem::swap(&mut self.age_groups[index], &mut tmp);
        }
        // Age group 0 is now in tmp
        self.age_groups[len] += tmp;
        self.age_groups[len-self.puberty] += tmp;
    }
}