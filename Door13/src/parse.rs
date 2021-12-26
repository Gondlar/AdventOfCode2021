use std::str::FromStr;

use super::matrix::Matrix;

pub fn get_next_line<'a, 'b>(cursor : &'a mut dyn Iterator<Item = &str>) -> Result<&'a str,&'b str> {
    match cursor.next() {
        Some(str) => Ok(str),
        None => Err("EOF")
    }
}

pub fn list<'b, A : FromStr>(cursor : & mut dyn Iterator<Item = &str>, seperator : char) -> Result<Vec<A>, &'b str> {
    let line = get_next_line(cursor)?;
    let list : Result<Vec<A>,A::Err> = line.split(seperator)
               .filter(|entry| !entry.is_empty())
               .map(|entry| entry.parse::<A>())
               .collect();
    match list {
        Err(_) => Err("List Inner Type Parser Error"),
        Ok(a) => Ok(a)
    }
}

pub fn characters<'b, A : FromStr>(cursor : & mut dyn Iterator<Item = &str>, map: fn(char) -> Result<A, &'b str>) -> Result<Vec<A>, &'b str> {
    let line = get_next_line(cursor)?;
    line.chars().map(map).collect()
}

pub fn empty<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str>  {
    let line = get_next_line(cursor)?;
    if !line.is_empty() { Err("Expected empty line but got content") } else { Ok(()) }
}

pub fn n<'b, Type>(cursor : & mut dyn Iterator<Item = &str>, count : usize, f : fn(& mut dyn Iterator<Item = &str>) -> Result<Type, &'b str>) -> Result<Vec<Type>, &'b str> {
    let mut collection : Vec<Type> = vec!();
    collection.reserve(count);
    for _ in 0..count {
        collection.push(f(cursor)?);
    }
    return Ok(collection);
}

pub fn all<'b, Type>(cursor : & mut dyn Iterator<Item = &str>, f : fn(& mut dyn Iterator<Item = &str>) -> Result<Type, &'b str>) -> Result<Vec<Type>, &'b str> {
    let mut collection : Vec<Type> = vec!();
    loop {
        let next = f(cursor);
        match next {
            Ok(new) => collection.push(new),
            Err(_) => { break; }
        }
    }
    return Ok(collection);
}

pub fn tuple<'b, A : FromStr>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(A,A), &'b str> {
    let mut res = list(cursor, ',')?;
    if res.len() != 2 {
        return Err("Wrong dimension");
    }
    let snd = res.remove(1);
    let fst = res.remove(0);
    return Ok((fst, snd));
}

pub fn matrix<'b, Type>(cursor : & mut dyn Iterator<Item = &str>, f : fn(& mut dyn Iterator<Item = &str>) -> Result<Vec<Type>, &'b str>) -> Result<Matrix<Type>, &'b str> {
    let first_line = f(cursor)?;
    let size = first_line.len();
    let mut rest = all(cursor, f)?;
    if !rest.iter().all(|entry| entry.len() == size) {
        return Err("Matrix Dimensions are malformed");
    }
    let mut result = Matrix::new_from_row(first_line);
    result.append_all(&mut rest);
    return Ok(result);
}
