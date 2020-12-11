use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

mod parsers;

pub use parsers::basic_parser;

pub fn parse_input_to_vec<T>(
    filename: &str,
    sep: &str,
    parser: fn(&str) -> T,
) -> Vec<T>
{
    let mut file = File::open(filename).unwrap();
    let mut result = Vec::<T>::new();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.split(sep) {
        result.push(parser(line));
    }

    result
}

pub fn parse_input_to_hashmap<K, V>(
    filename: &str,
    sep: &str,
    parser: fn(&str) -> (K, V),
) -> HashMap<K, V>
where
    K: Eq + Hash,
{
    let mut file = File::open(filename).unwrap();
    let mut result = HashMap::<K, V>::new();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.split(sep) {
        let tmp = parser(line);
        result.insert(tmp.0, tmp.1);
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
