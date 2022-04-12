use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;

mod parsers;

pub use parsers::basic_parser;

fn read_file_contents(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

pub fn parse_input_to_vec<T>(filename: &str, sep: &str, parser: fn(&str) -> T) -> Vec<T> {
    let mut result = Vec::<T>::new();

    let contents = read_file_contents(filename);
    for line in contents.split(sep) {
        if line != "" {
            result.push(parser(line));
        }
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
    let mut result = HashMap::<K, V>::new();

    let contents = read_file_contents(filename);
    for line in contents.split(sep) {
        let tmp = parser(line);
        result.insert(tmp.0, tmp.1);
    }

    result
}

pub fn parse_input<T>(filename: &str, parser: fn(String) -> T) -> T {
    let contents = read_file_contents(filename);
    parser(contents)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
