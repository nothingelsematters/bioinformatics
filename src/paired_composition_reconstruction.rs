use crate::utils::{read_line, read_lines};
use std::collections::HashMap;

pub fn solve_paired_composition_reconstruction() {
    let v = read_line()
        .split(' ')
        .map(|x| x.parse::<usize>().expect("number"))
        .collect::<Vec<_>>();
    let k = v[0];
    let d = v[1];

    let composition = read_lines()
        .into_iter()
        .map(|x| {
            x.split_once('|')
                .map(|(l, r)| (l.to_owned(), r.to_owned()))
                .expect("composition format")
        })
        .collect::<Vec<_>>();

    let reconstructed = paired_composition_reconstruction(k, d, &composition);
    println!("{}", reconstructed);
}

/// Works unexpectedly with repetitions.
pub fn paired_composition_reconstruction(
    k: usize,
    d: usize,
    composition: &[(String, String)],
) -> String {
    let mut v = vec!['0'; k * 2 + d + composition.len() - 1];

    let shifts = composition
        .iter()
        .filter_map(|(l, r)| {
            composition
                .iter()
                .find(|(ll, rr)| l[1..] == ll[..ll.len() - 1] && r[1..] == rr[..rr.len() - 1])
                .map(|x| ((l, r), x))
        })
        .collect::<HashMap<_, _>>();

    let mut current = composition
        .iter()
        .find(|(l, r)| !shifts.values().any(|(ll, rr)| (ll, rr) == (l, r)));

    let mut position = 0;

    while let Some((left, right)) = current {
        left.chars()
            .enumerate()
            .for_each(|(i, x)| v[position + i] = x);
        right
            .chars()
            .enumerate()
            .for_each(|(i, x)| v[position + i + d + k] = x);

        position += 1;
        if position >= composition.len() {
            break;
        }

        current = shifts.get(&(left, right)).copied();
    }

    v.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        let actual = paired_composition_reconstruction(
            4,
            2,
            &[
                ("GAGA".to_owned(), "TTGA".to_owned()),
                ("TCGT".to_owned(), "GATG".to_owned()),
                ("CGTG".to_owned(), "ATGT".to_owned()),
                ("TGGT".to_owned(), "TGAG".to_owned()),
                ("GTGA".to_owned(), "TGTT".to_owned()),
                ("GTGG".to_owned(), "GTGA".to_owned()),
                ("TGAG".to_owned(), "GTTG".to_owned()),
                ("GGTC".to_owned(), "GAGA".to_owned()),
                ("GTCG".to_owned(), "AGAT".to_owned()),
            ],
        );
        let expected = "GTGGTCGTGAGATGTTGA".to_owned();
        assert_eq!(actual, expected)
    }
}
