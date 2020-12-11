use std::str::FromStr;
use std::fmt::Debug;

pub fn basic_parser<T>(line: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    line.parse::<T>().unwrap()
}

