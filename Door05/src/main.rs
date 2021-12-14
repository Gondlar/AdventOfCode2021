use std::env;
use std::fs;

mod parse;
mod geom;


fn parse_point<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<geom::Point, &'b str>  {
    let coordinates = parse::parse_list(cursor, ',')?;
    if coordinates.len() != 2 {
        return Err("Invalid number of coordiantes in point");
    }
    return Ok(geom::Point::from_coordinates(coordinates[0], coordinates[1]));
}

fn parse_line<'b>(line : &str) -> Result<geom::Line, &'b str>  {
    let points = parse::parse_n(&mut line.split(" -> "), 2, parse_point)?;
    return Ok(geom::Line::from_points(points[0], points[1]));
}

fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    let lines = cursor.map(parse_line)
                      .collect::<Result<Vec<geom::Line>,&str>>()?;
    let bound = lines.iter().fold(geom::Point::zero(), |lhs, rhs| lhs.skyline(&rhs.get_bound()));
    let mut map = geom::Area::new(&bound);
    for line in lines.iter().filter(|line| line.is_straight()) {
        map.draw_line(line);
    }
    let part1 = map.count_crossing();
    for line in lines.iter().filter(|line| !line.is_straight()) {
        map.draw_line(line);
    }
    println!("Overlaps: {} -> {}", part1, map.count_crossing());
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