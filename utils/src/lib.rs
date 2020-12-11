use std::fs::File;
use std::io::prelude::*;

mod parsers;

pub use parsers::basic_parser;

pub fn parse_input<T>(
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
