use crate::utils::read_line;
use std::collections::HashMap;

pub fn solve_approximate_occurancies() {
    let string = read_line();
    let patterns: Vec<_> = read_line().split(' ').map(|x| x.to_owned()).collect();
    let d: usize = read_line().parse().unwrap();

    for i in approximate_occurancies(&string, &patterns, d) {
        print!("{i} ");
    }
    println!()
}

pub fn approximate_occurancies(string: &str, patterns: &[String], d: usize) -> Vec<usize> {
    let suffix_tree = SuffixTree::from_str(string);
    patterns.iter().fold(Vec::new(), |mut v, i| {
        v.append(&mut suffix_tree.get_all_occurancies(&i.chars().collect::<Vec<char>>(), d));
        v
    })
}

#[derive(Debug)]
struct SuffixTree {
    indices: Vec<usize>,
    transitions: HashMap<char, Box<SuffixTree>>,
}

impl SuffixTree {
    fn new() -> Self {
        SuffixTree {
            indices: Vec::new(),
            transitions: HashMap::<char, Box<SuffixTree>>::new(),
        }
    }

    fn from_str(string: &str) -> Self {
        let chars: Vec<char> = string.chars().collect();

        let mut root = SuffixTree::new();

        for i in 0..chars.len() - 1 {
            root.add(&chars[i..], i);
        }

        root
    }

    fn add(&mut self, chars: &[char], index: usize) {
        self.indices.push(index);

        if !chars.is_empty() {
            self.transitions
                .entry(chars[0])
                .or_insert_with(|| Box::new(SuffixTree::new()))
                .add(&chars[1..], index + 1)
        }
    }

    fn get_all_occurancies(&self, string: &[char], mismatches: usize) -> Vec<usize> {
        if string.is_empty() {
            return self.indices.clone();
        }

        self.transitions
            .iter()
            .filter(|(k, _)| **k == string[0] || mismatches != 0)
            .flat_map(|(k, v)| {
                v.get_all_occurancies(
                    &string[1..],
                    if *k == string[0] {
                        mismatches
                    } else {
                        mismatches - 1
                    },
                )
            })
            .into_iter()
            .map(|x| x - 1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_test() {
        let actual = super::approximate_occurancies(
            "ACATGCTACTTT",
            &[
                "ATT".to_owned(),
                "GCC".to_owned(),
                "GCTA".to_owned(),
                "TATT".to_owned(),
            ],
            1,
        );
        let expected = vec![2, 4, 4, 6, 7, 8, 9];
        assert_eq!(actual, expected)
    }
}
