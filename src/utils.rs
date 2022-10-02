use std::{fmt::Debug, io::stdin, str::FromStr};

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
