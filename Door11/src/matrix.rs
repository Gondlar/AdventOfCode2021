use std::iter::Iterator;

pub struct Matrix<A> {
    height: usize,
    width: usize,
    data: Vec<A>
}

pub type Coordinates = (usize, usize);
pub type RelativeCoordinates = (isize, isize);

impl<A> Matrix<A> {
    pub fn new_from_row(row: Vec<A>) -> Matrix<A> {
        return Matrix{width: row.len(), height: 1, data: row};
    }

    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }
    pub fn len(&self) -> usize { self.width * self.height }

    fn calc_coordinates(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y < self.height);
        y*self.width + x
    }
    pub fn get(&self, x: usize, y: usize) -> &A { &self.data[self.calc_coordinates(x, y)] }
    pub fn set(&mut self , x: usize, y: usize, val: A) {
        let index = self.calc_coordinates(x,y);
        self.data[index] = val;
    }

    pub fn append_row(&mut self, row: &mut Vec<A>) {
        assert!(row.len() == self.width);
        self.data.append(row);
        self.height += 1;
    }

    pub fn append_all(&mut self, rows: &mut Vec<Vec<A>>) {
        self.data.reserve(rows.len() * self.width);
        for row in rows {
            self.append_row(row);
        }
    }
}

impl<'a, A> Matrix<A> 
    where A: 'a
{
    pub fn row_iter(&'a self, y: usize) -> Box<dyn std::iter::Iterator<Item = &A>+'a> {
        Box::new((0..self.width).map(move |x| self.get(x, y)))
    }

    pub fn col_iter(&'a self, x: usize) -> Box<dyn std::iter::Iterator<Item = &A>+'a> {
        Box::new((0..self.width).map(move |y| self.get(x, y)))
    }

    pub fn iter(&'a self) -> Box<dyn std::iter::Iterator<Item = &A>+'a> {
        Box::new(self.data.iter())
    }

    pub fn coords_iter(& self) -> Box<dyn std::iter::Iterator<Item = Coordinates>> {
        let height = self.get_height();
        let width = self.get_width();
        Box::new(
            (0..width).flat_map(move |x| (0..height).map(move |y| (x,y)))
        )
    }

    pub fn get_all(&'a self, iter: &'a mut dyn Iterator<Item = Coordinates>) -> Box<dyn Iterator<Item = &A>+'a> {
        Box::new(iter.map(|(x,y)| { self.get(x,y)} ))
    }
    
    pub fn relativ_coords(&self, x: usize, y: usize, neighbors: &'a Vec<RelativeCoordinates>) -> Box<dyn Iterator<Item = Coordinates>+'a> {
        let width = self.get_width();
        let height = self.get_height();
        Box::new(neighbors.iter()
                          .filter(move |(rel_x, rel_y)| {
                            let neighbor_x = (x as isize)+*rel_x;
                            let neighbor_y = (y as isize)+*rel_y;
                            return !(neighbor_x < 0 || neighbor_x >= width as isize || neighbor_y < 0 || neighbor_y >= height as isize);
                          })
                          .map(move |(rel_x, rel_y)| {
                                let neighbor_x = ((x as isize)+*rel_x) as usize;
                                let neighbor_y = ((y as isize)+*rel_y) as usize;
                                return (neighbor_x, neighbor_y)
                          }))
    }

    fn relativ_coords_arr(&self, x: usize, y: usize, neighbors: &'a [RelativeCoordinates]) -> Box<dyn Iterator<Item = Coordinates>+'a> {
        let width = self.get_width();
        let height = self.get_height();
        Box::new(neighbors.iter()
                          .filter(move |(rel_x, rel_y)| {
                            let neighbor_x = (x as isize)+*rel_x;
                            let neighbor_y = (y as isize)+*rel_y;
                            return !(neighbor_x < 0 || neighbor_x >= width as isize || neighbor_y < 0 || neighbor_y >= height as isize);
                          })
                          .map(move |(rel_x, rel_y)| {
                                let neighbor_x = ((x as isize)+*rel_x) as usize;
                                let neighbor_y = ((y as isize)+*rel_y) as usize;
                                return (neighbor_x, neighbor_y)
                          }))
    }

    pub fn neighbor_coords(&self, x: usize, y: usize) -> Box<dyn Iterator<Item = Coordinates>+'a> {
        static NEIGHBORS : [(isize,isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        self.relativ_coords_arr(x, y, &NEIGHBORS)
    }

    pub fn around_coords(&self, x: usize, y: usize) -> Box<dyn Iterator<Item = Coordinates>+'a> {
        static AROUND : [(isize,isize); 9] = [(-1,  0), (0,  0), (1,  0),
                                              (-1, -1), (0, -1), (1, -1),
                                              (-1,  1), (0,  1), (1,  1)];
        self.relativ_coords_arr(x, y, &AROUND)
    }
}

impl<A:Clone+Copy> Matrix<A> {
    pub fn new_with_init(width: usize, height: usize, init: &A) -> Matrix<A> {
        return Matrix{width: width, height: height, data: vec![*init; width*height]}
    }
}

impl<A:Default + Clone + Copy> Matrix<A> {
    pub fn new(width: usize, height: usize) -> Matrix<A> {
        return Matrix::new_with_init(width, height, &A::default());
    }
}

impl<A> std::ops::Index<Coordinates> for Matrix<A> {
    type Output = A;

    fn index(&self, coords: Coordinates) -> &Self::Output {
        let (x, y) = coords;
        return self.get(x, y);
    }
}

impl<A> std::ops::IndexMut<Coordinates> for Matrix<A> {
    fn index_mut(&mut self, coords: Coordinates) -> &mut Self::Output {
        let (x, y) = coords;
        let index = self.calc_coordinates(x, y);
        return &mut self.data[index];
    }
}
