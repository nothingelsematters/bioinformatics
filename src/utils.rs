use std::{fmt::Debug, io::stdin, ops::Deref, str::FromStr};

pub fn read_line() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("stdio reading");
    buffer.trim_end_matches('\n').to_owned()
}

pub fn read_line_and_parse<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    read_line().parse::<T>().expect("parsing")
}

pub fn read_lines() -> Vec<String> {
    stdin()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("stdio reading")
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OrdF64(pub f64);

impl Eq for OrdF64 {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Deref for OrdF64 {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
