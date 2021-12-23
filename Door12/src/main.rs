use std::env;
use std::fs;
use std::collections::HashMap;

mod matrix;
mod parse;

use matrix::Matrix;

struct IdMap {
    name_to_id : HashMap<String,usize>,
    sizes: Vec<bool>
}

impl IdMap {
    fn new() -> IdMap { IdMap{name_to_id: HashMap::new(), sizes: vec![]}  }

    fn put(&mut self, data: String) {
        if !self.name_to_id.contains_key(&data) {
            let is_large = (*data).chars().next().unwrap().is_uppercase();
            self.name_to_id.insert(data, self.sizes.len());
            self.sizes.push(is_large);
        }
    }

    fn get(&self, data: &String) -> usize{ *self.name_to_id.get(data).unwrap() }

    fn get_start(&self) -> usize { self.get(&String::from("start")) }
    fn get_end(&self) -> usize { self.get(&String::from("end")) }

    fn len(&self) -> usize { self.name_to_id.len() }

    fn is_large(&self, id: usize) -> bool { self.sizes[id] }
}

fn parse_connections<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<Vec<(String, String)>, &'b str> {
    parse::parse_all(cursor, |line| {
        let line = parse::get_next_line(line)?;
        let mut parts = line.split('-');
        let fst = String::from(parse::get_next_line(&mut parts)?);
        let snd = String::from(parse::get_next_line(&mut parts)?);
        return Ok((fst, snd));
    })
}

fn dfs(ids: &IdMap, adjacency: &Matrix<bool>, visited: &mut Vec<bool>, current: usize) -> (u32, u32) {
    if current == ids.get_end() {
        return (1, 1);
    }
    if !ids.is_large(current) {
        visited[current] = true;
    }
    let mut paths = 0;
    let mut double_paths = 0;
    for other in 0..adjacency.get_width() {
        if *adjacency.get(other, current) {
            let recurred_id = visited.len()-1; // Special position to remember whether we repeated a cave once
            if !visited[other] {
                let (newpaths, newdoublepaths) = dfs(ids, adjacency, visited, other);
                paths += newpaths;
                double_paths += newdoublepaths;
            } else if !visited[recurred_id] && other != ids.get_end() && other != ids.get_start() {
                visited[recurred_id] = true;
                let (_, newdoublepaths) = dfs(ids, adjacency, visited, other);
                double_paths += newdoublepaths;
                visited[recurred_id] = false;
                visited[other] = true;
            }
        }
    }
    visited[current] = false;
    return (paths, double_paths);
}

fn do_work<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str> {
    let connections = parse_connections(cursor)?;
    let keys = {
        let mut keys = IdMap::new();
        for (from, to) in &connections {
            keys.put(from.clone());
            keys.put(to.clone());
        }
        keys
    };
    let adjacency = {
        let mut adjacency = Matrix::new(keys.len(), keys.len());
        for (from, to) in &connections {
            let from = keys.get(from);
            let to = keys.get(to);
            adjacency.set(from, to, true);
            adjacency.set(to, from, true);
        }
        adjacency
    };
    let mut visited = vec![false; keys.len()+1];
    let (pathcount, double_pathcount) = dfs(&keys, &adjacency, &mut visited, keys.get_start());
    println!("Number of Paths: {}", pathcount);
    println!("Number of Repeated Paths: {}", double_pathcount);

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