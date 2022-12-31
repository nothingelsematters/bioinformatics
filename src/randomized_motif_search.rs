use rand::seq::IteratorRandom;

use crate::utils::{read_line, OrdF64};

lazy_static::lazy_static! {
    static ref SYMBOLS: Vec<char> = vec!['A', 'C', 'G', 'T'];
}

pub fn solve_randomized_motif_search() {
    let v = read_line()
        .split(' ')
        .map(|x| x.parse::<usize>().expect("number"))
        .collect::<Vec<_>>();
    let k = v[0];
    let t = v[1];

    let strings: Vec<String> = (0..t).map(|_| read_line()).collect();

    println!("{:#?}", randomized_motif_search(k, &strings));
}

pub fn randomized_motif_search(k: usize, strings: &[String]) -> Vec<String> {
    let transformed: Vec<Vec<usize>> = strings
        .iter()
        .map(|string| {
            string
                .chars()
                .map(|x| {
                    SYMBOLS
                        .iter()
                        .enumerate()
                        .find(|(_, y)| x == **y)
                        .unwrap()
                        .0
                })
                .collect()
        })
        .collect();

    (0..1_000)
        .map(|_| one_time_search(k, &transformed))
        .min_by_key(|motifs| score(motifs))
        .unwrap()
        .into_iter()
        .map(|v| v.iter().map(|i| SYMBOLS[*i]).collect())
        .collect()
}

fn one_time_search(k: usize, strings: &[Vec<usize>]) -> Vec<&[usize]> {
    fn random_motif(k: usize, strings: &[Vec<usize>]) -> Vec<&[usize]> {
        let mut rng = rand::thread_rng();

        strings
            .iter()
            .map(|s| {
                let i = (0..strings[0].len() - k).choose(&mut rng).unwrap();
                &s[i..i + k]
            })
            .collect::<Vec<_>>()
    }

    let mut current = random_motif(k, strings);
    let mut best = random_motif(k, strings);

    loop {
        let profile = profile(&current);
        let new_motifs: Vec<_> = strings
            .iter()
            .map(|x| {
                (0..x.len() - k)
                    .map(|i| &x[i..i + k])
                    .max_by_key(|m| {
                        let value = m.iter().enumerate().map(|(i, x)| profile[*x][i]).product();
                        OrdF64(value)
                    })
                    .unwrap()
            })
            .collect();

        if score(&new_motifs) < score(&best) {
            current = new_motifs.clone();
            best = new_motifs;
        } else {
            break best;
        }
    }
}

fn profile(strings: &[&[usize]]) -> Vec<Vec<f64>> {
    (0..SYMBOLS.len())
        .map(|i| {
            (0..strings[0].len())
                .map(|j| {
                    let count = strings.iter().map(|x| x[j]).filter(|x| *x == i).count();
                    (count + 1) as f64 / (strings.len() + 1) as f64
                })
                .collect()
        })
        .collect()
}

fn score(strings: &[&[usize]]) -> usize {
    (0..strings[0].len())
        .map(|i| {
            strings.len()
                - (0..SYMBOLS.len())
                    .map(|symbol| {
                        strings
                            .iter()
                            .map(|string| string[i])
                            .filter(|x| *x == symbol)
                            .count()
                    })
                    .max()
                    .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::randomized_motif_search;

    #[test]
    fn sample() {
        let k = 8;
        let strings = vec![
            "CGCCCCTCTCGGGGGTGTTCAGTAAACGGCCA".to_owned(),
            "GGGCGAGGTATGTGTAAGTGCCAAGGTGCCAG".to_owned(),
            "TAGTACCGAGACCGAAAGAAGTATACAGGCGT".to_owned(),
            "TAGATCAAGTTTCAGGTGCACGTCGGTGAACC".to_owned(),
            "AATCCACCAGCTCCACGTGCAATGTTGGCCTA".to_owned(),
        ];

        assert_eq!(
            randomized_motif_search(k, &strings),
            vec!["TCTCGGGG", "CCAAGGTG", "TACAGGCG", "TTCAGGTG", "TCCACGTG"]
        )
    }
}
