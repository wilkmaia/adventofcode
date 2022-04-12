use std::fmt::Debug;
use std::str::FromStr;

pub fn basic_parser<T>(line: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    line.parse::<T>().unwrap()
}
