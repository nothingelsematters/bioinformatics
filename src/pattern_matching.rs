use crate::utils::{read_line, read_lines};
use std::collections::HashMap;

pub fn solve_pattern_matching() {
    let string = read_line();
    let patterns = read_lines();
    for i in pattern_matching(&string, &patterns) {
        print!("{i} ");
    }
    println!()
}

pub fn pattern_matching(string: &str, patterns: &[String]) -> Vec<usize> {
    let suffix_tree = SuffixTree::from_str(string);
    patterns.iter().fold(Vec::new(), |mut v, i| {
        v.append(&mut suffix_tree.get_all_occurancies(&i.chars().collect::<Vec<char>>()));
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

    fn get_all_occurancies(&self, string: &[char]) -> Vec<usize> {
        if string.is_empty() {
            return self.indices.clone();
        }

        self.transitions[&string[0]]
            .get_all_occurancies(&string[1..])
            .into_iter()
            .map(|x| x - 1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_test() {
        let actual = super::pattern_matching(
            "AATCGGGTTCAATCGGGGT",
            &["ATCG".to_owned(), "GGGT".to_owned()],
        );
        let expected = vec![1, 4, 11, 15];
        assert_eq!(actual, expected)
    }
}
