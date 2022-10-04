use crate::utils::{read_line_and_parse, read_lines};

pub fn solve_median_string() {
    let size: usize = read_line_and_parse();
    let strings = read_lines();
    println!("{}", median_string(size, &strings));
}

pub fn median_string(size: usize, strings: &[String]) -> String {
    let strings = strings
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    strings
        .iter()
        .flat_map(|s| s.windows(size).collect::<Vec<_>>())
        .min_by_key(|substring| {
            strings
                .iter()
                .map(|string| {
                    string
                        .windows(size)
                        .map(|window| hamming_distance(window, substring))
                        .min()
                        .expect("non empty string")
                })
                .sum::<usize>()
        })
        .expect("find substring")
        .iter()
        .collect()
}

fn hamming_distance<T: Eq>(left: &[T], right: &[T]) -> usize {
    left.iter()
        .zip(right.iter())
        .filter(|(x, y)| x != y)
        .count()
}

#[cfg(test)]
pub mod tests {
    use super::median_string;

    #[test]
    fn sample_test() {
        let strings = vec![
            "AAATTGACGCAT".to_owned(),
            "GACGACCACGTT".to_owned(),
            "CGTCAGCGCCTG".to_owned(),
            "GCTGAGCACCGG".to_owned(),
            "AGTACGGGACAG".to_owned(),
        ];
        let actual = median_string(3, &strings);
        let expected = "GAC".to_owned();
        assert_eq!(actual, expected);
    }
}
