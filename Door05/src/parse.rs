use std::str::FromStr;

pub fn get_next_line<'a, 'b>(cursor : &'a mut dyn Iterator<Item = &str>) -> Result<&'a str,&'b str> {
    match cursor.next() {
        Some(str) => Ok(str),
        None => Err("EOF")
    }
}

pub fn parse_list<'b, A : FromStr>(cursor : & mut dyn Iterator<Item = &str>, seperator : char) -> Result<Vec<A>, &'b str> {
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

pub fn parse_empty<'b>(cursor : & mut dyn Iterator<Item = &str>) -> Result<(), &'b str>  {
    let line = get_next_line(cursor)?;
    if !line.is_empty() { Err("Expected empty line but got content") } else { Ok(()) }
}

pub fn parse_n<'b, Type>(cursor : & mut dyn Iterator<Item = &str>, count : usize, f : fn(& mut dyn Iterator<Item = &str>) -> Result<Type, &'b str>) -> Result<Vec<Type>, &'b str> {
    let mut collection : Vec<Type> = vec!();
    collection.reserve(count);
    for _ in 0..count {
        collection.push(f(cursor)?);
    }
    return Ok(collection);
}

pub fn parse_all<'b, Type>(cursor : & mut dyn Iterator<Item = &str>, f : fn(& mut dyn Iterator<Item = &str>) -> Result<Type, &'b str>) -> Result<Vec<Type>, &'b str> {
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