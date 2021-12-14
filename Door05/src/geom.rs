#[derive(Clone,Copy,Debug)]
pub struct Point {
    x : u32,
    y : u32
}

impl Point {
    pub fn zero() -> Point {
        return Point{x: 0, y: 0};
    }

    pub fn from_coordinates(x : u32, y: u32) -> Point{
        return Point{x: x, y: y};
    }

    pub fn get_x(&self) -> u32 { self.x }
    pub fn get_y(&self) -> u32 { self.y }

    pub fn skyline(&self, other: &Point) -> Point {
        return Point{
            x: std::cmp::max(self.x, other.x),
            y: std::cmp::max(self.y, other.y),
        }
    }
}

pub struct Line {
    start : Point,
    end : Point
}

impl Line {
    pub fn from_points(start: Point, end: Point) -> Line {
        return Line{start: start, end: end}
    }

    pub fn is_horizontal(&self) -> bool { self.start.y == self.end.y }
    pub fn is_vertical(&self) -> bool { self.start.x == self.end.x }
    pub fn is_straight(&self) -> bool { self.is_horizontal() || self.is_vertical() }

    /**
     * length measures the number of steps in this line.
     * Since diagonal lines are always 45°, their distance is equal to both their horizontal and vertical distance
     */
    pub fn length(&self) -> usize {
        if !self.is_vertical() {
            return (self.start.x + self.end.x - 2*std::cmp::min(self.start.x, self.end.x)) as usize +1;
        } else {
            return (self.start.y + self.end.y - 2*std::cmp::min(self.start.y, self.end.y)) as usize +1;
        }
    }

    pub fn get_bound(&self) -> Point { self.start.skyline(&self.end) }

    fn get_coordinates(start: u32, end: u32, length: usize, straight: bool) -> Vec<u32> {
        if straight {
            return std::iter::repeat(start).take(length).collect();
        }
        let range = std::cmp::min(start, end) ..= std::cmp::max(start, end);
        if start > end {
            return range.rev().collect();
        }
        return range.collect();
    }

    fn get_x_coordinates(&self) -> Vec<u32> {
        return Line::get_coordinates(self.start.x, self.end.x, self.length(), self.is_vertical());
    }

    fn get_y_coordinates(&self) -> Vec<u32> {
        return Line::get_coordinates(self.start.y, self.end.y, self.length(), self.is_horizontal());
    }

    pub fn get_points(&self) -> Vec<Point> {
        let horizontal = self.get_x_coordinates();
        let vertical = self.get_y_coordinates();
        horizontal.iter()
                .zip(vertical.iter())
                .map(|(x,y)| Point::from_coordinates(*x, *y))
                .collect()
    }
}

#[derive(Debug)]
pub struct Area {
    counts : Vec<Vec<u32>>
}

impl Area {
    pub fn new(bound : &Point) -> Area {
        let x = (bound.x + 1).try_into().unwrap();
        let y = (bound.y + 1).try_into().unwrap();
        Area{counts: vec![vec![0; y]; x]}
    }

    pub fn draw_point(&mut self, point: &Point) {
        self.counts[point.x as usize][point.y as usize] += 1;
    }

    pub fn draw_line(&mut self, line: &Line) {
        for point in &line.get_points() {
            self.draw_point(point);
        }
    }

    pub fn count_crossing(&self) -> u32 {
        self.counts.iter()
                   .map(|vec| vec.iter()
                                 .filter(|val| **val > 1)
                                 .count()
                    ).map(|val| val as u32)
                    .sum()
    }
}
