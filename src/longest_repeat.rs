use crate::utils::read_line;
use std::collections::HashMap;

pub fn solve_longest_repeat() {
    let string = read_line();
    println!("{}", longest_repeat(&string))
}

pub fn longest_repeat(string: &str) -> String {
    let suffix_tree = SuffixTree::from_str(string);
    suffix_tree.longest_path()
}

#[derive(Debug)]
struct SuffixTree {
    is_terminal: bool,
    transitions: HashMap<char, Box<SuffixTree>>,
}

impl SuffixTree {
    fn new() -> Self {
        SuffixTree {
            is_terminal: false,
            transitions: HashMap::<char, Box<SuffixTree>>::new(),
        }
    }

    fn from_str(string: &str) -> Self {
        let chars: Vec<char> = string.chars().collect();

        let mut root = SuffixTree::new();

        for i in 0..chars.len() - 1 {
            root.add(&chars[i..]);
        }

        root
    }

    fn add(&mut self, chars: &[char]) {
        if chars.is_empty() {
            self.is_terminal = true;
            return;
        }

        self.transitions
            .entry(chars[0])
            .or_insert_with(|| Box::new(SuffixTree::new()))
            .add(&chars[1..])
    }

    fn longest_path(&self) -> String {
        self.longest_path_internal()
            .map(|chars| chars.into_iter().rev().collect())
            .unwrap_or_default()
    }

    fn longest_path_internal(&self) -> Option<Vec<char>> {
        if self.transitions.is_empty() {
            return None;
        }

        let results: Vec<_> = self
            .transitions
            .iter()
            .map(|(k, v)| (k, v.longest_path_internal()))
            .collect();

        if !self.is_terminal && results.iter().all(|(_, substring)| substring.is_none()) {
            if results.len() > 1 {
                return Some(Vec::new());
            }
            return None;
        }

        let substring = results
            .into_iter()
            .filter_map(|(k, substring)| substring.map(|s| (k, s)))
            .max_by_key(|(_, substring)| substring.len())
            .map(|(k, mut substring)| {
                substring.push(*k);
                substring
            })
            .unwrap_or_default();
        Some(substring)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        let actual = longest_repeat("ATATCGTTTTATCGTT");
        let expected = "TATCGTT".to_owned();
        assert_eq!(actual, expected)
    }
}
