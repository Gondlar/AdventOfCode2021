use std::iter::Iterator;

#[derive(Copy,Clone)]
pub struct Coordinates<T = usize>((T, T));

type RelativeCoordinates = Coordinates<isize>;

impl<T> std::ops::Deref for Coordinates<T> {
    type Target = (T, T);

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T:PartialOrd+Copy> Coordinates<T> {
    pub fn within(&self, l: T, r: T, b: T, t: T) -> bool {
        let (x,y) = **self;
        return x >= l && x <= r && y >= b && y <= t;
    }
}

impl RelativeCoordinates {
    pub fn to_coordinates(&self) -> Coordinates {
        let (x,y) = **self;
        assert!(x >= 0 && y >= 0);
        return Coordinates((x as usize, y as usize));
    }
}

impl std::ops::Add<RelativeCoordinates> for Coordinates {
    type Output = RelativeCoordinates;

    fn add(self, rhs: RelativeCoordinates) -> RelativeCoordinates {
        let (x, y) = *self;
        let (rel_x, rel_y) = *rhs;
        assert!(x <= isize::MAX as usize && y <= isize::MAX as usize);
        let sum_x = (x as isize)+rel_x;
        let sum_y = (y as isize)+rel_y;
        return Coordinates((sum_x, sum_y));
    }
}



pub struct Matrix<A> {
    height: usize,
    width: usize,
    data: Vec<A>
}

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
            (0..width).flat_map(move |x| (0..height).map(move |y| Coordinates((x,y))))
        )
    }

    pub fn get_all(&'a self, iter: &'a mut dyn Iterator<Item = Coordinates>) -> Box<dyn Iterator<Item = &A>+'a> {
        Box::new(iter.map(|coords| { &self[coords] } ))
    }
    
    pub fn relativ_coords(&self, origin: Coordinates, neighbors: &'a Vec<RelativeCoordinates>) -> Box<dyn Iterator<Item = Coordinates>+'a> {
        let width = self.get_width() as isize;
        let height = self.get_height() as isize;
        Box::new(neighbors.iter()
                          .filter(move |rel| (origin + **rel).within(0, width-1, 0, height-1) )
                          .map(move |rel| (origin + *rel).to_coordinates() ))
    }

    fn relativ_coords_arr(&self, origin: Coordinates, neighbors: &'a [RelativeCoordinates]) -> Box<dyn Iterator<Item = Coordinates>+'a> {
        let width = self.get_width() as isize;
        let height = self.get_height() as isize;
        Box::new(neighbors.iter()
                          .filter(move |rel| (origin + **rel).within(0, width-1, 0, height-1) )
                          .map(move |rel| (origin + *rel).to_coordinates() ))
    }

    pub fn neighbor_coords(&self, origin: Coordinates) -> Box<dyn Iterator<Item = Coordinates>+'a> {
        static NEIGHBORS : [RelativeCoordinates; 4]
            = [Coordinates((-1, 0)), Coordinates((1, 0)), Coordinates((0, -1)), Coordinates((0, 1))];
        self.relativ_coords_arr(origin, &NEIGHBORS)
    }

    pub fn around_coords(&self, origin: Coordinates) -> Box<dyn Iterator<Item = Coordinates>+'a> {
        static AROUND : [RelativeCoordinates; 9] = [Coordinates((-1,  0)), Coordinates((0,  0)), Coordinates((1,  0)),
                                                    Coordinates((-1, -1)), Coordinates((0, -1)), Coordinates((1, -1)),
                                                    Coordinates((-1,  1)), Coordinates((0,  1)), Coordinates((1,  1))];
        self.relativ_coords_arr(origin, &AROUND)
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
        let (x, y) = *coords;
        return self.get(x, y);
    }
}

impl<A> std::ops::IndexMut<Coordinates> for Matrix<A> {
    fn index_mut(&mut self, coords: Coordinates) -> &mut Self::Output {
        let (x, y) = *coords;
        let index = self.calc_coordinates(x, y);
        return &mut self.data[index];
    }
}
