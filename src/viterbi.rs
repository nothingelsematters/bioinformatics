use crate::utils::read_line;
use std::cmp::Ordering;

pub type Matrix<T> = Vec<Vec<T>>;

macro_rules! matrix {
    ($elem:expr; $x:expr, $y:expr) => {
        vec![vec![$elem; $y]; $x]
    };
}

fn read_splitted() -> Vec<String> {
    read_line()
        .split(|x: char| x.is_whitespace())
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect()
}

fn read_table(len: usize) -> Matrix<f64> {
    let _header = read_splitted();

    (0..len)
        .map(|_| {
            read_splitted()
                .iter()
                .skip(1)
                .map(|x| x.parse().expect("successful f64 parsing"))
                .collect()
        })
        .collect()
}

pub fn solve_viterbi() {
    let string = read_line();
    let _ = read_line();
    let alphabet = read_splitted();
    let _ = read_line();
    let states = read_splitted();
    let _ = read_line();
    let transitions = read_table(states.len());
    let _ = read_line();
    let emissions = read_table(states.len());

    let observations = string
        .chars()
        .map(|x| {
            alphabet
                .iter()
                .position(|y| x.to_string() == *y)
                .expect("non alphabet observation")
        })
        .collect();

    let result = viterbi(observations, &states, transitions, emissions);
    println!("{}", result)
}

fn max_with_index(iter: impl Iterator<Item = f64>) -> (usize, f64) {
    iter.enumerate()
        .max_by(|(_, a), (_, b)| {
            if a > b {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .expect("non empty iterator")
}

pub fn viterbi(
    observations: Vec<usize>,
    states: &[String],
    transitions: Matrix<f64>,
    emissions: Matrix<f64>,
) -> String {
    let states_len = states.len();
    let observations_len = observations.len();

    let mut omega = matrix![0.0; observations_len, states_len];
    omega[0] = emissions
        .iter()
        .map(|vec| (vec[observations[0]] * 0.5).log2())
        .collect();

    let mut previous = matrix![0; observations_len, states_len];

    for t in 1..observations_len {
        for j in 0..states_len {
            let emission = emissions[j][observations[t]].log2();

            let (index, value) = max_with_index(
                omega[t - 1]
                    .iter()
                    .zip(transitions.iter().map(|x| x[j].log2()))
                    .map(|(x, y)| x + y + emission),
            );

            previous[t - 1][j] = index;
            omega[t][j] = value;
        }
    }

    let mut result = Vec::with_capacity(observations_len);
    result.push(max_with_index(omega[observations_len - 1].iter().copied()).0);

    for i in (0..observations_len - 1).rev() {
        result.push(previous[i][result[observations_len - i - 2]]);
    }

    result.iter().rev().map(|x| states[*x].clone()).collect()
}

#[cfg(test)]
pub mod tests {
    use super::viterbi;

    #[test]
    fn first_sample_test() {
        let observations = vec![0, 1, 0, 2, 2, 0, 1, 0, 1, 1];
        let states = vec!["A".to_owned(), "B".to_owned()];
        let transitions = vec![vec![0.641, 0.359], vec![0.729, 0.271]];
        let emissions = vec![vec![0.117, 0.691, 0.192], vec![0.097, 0.42, 0.483]];

        let result = viterbi(observations, &states, transitions, emissions);
        assert_eq!(&result, "AAABBAAAAA");
    }

    #[test]
    fn second_sample_test() {
        let observations = vec![
            2, 0, 0, 0, 0, 1, 2, 2, 0, 1, 0, 1, 0, 1, 2, 0, 2, 2, 0, 2, 2, 2, 1, 2, 2, 0, 0, 0, 2,
            0, 0, 1, 1, 1, 2, 0, 1, 0, 2, 1, 0, 1, 0, 1, 2, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 2, 2, 0,
            2, 0, 2, 1, 2, 2, 2, 2, 1, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 2, 1, 1, 2, 1, 1, 1, 0,
            2, 2, 2, 2, 1, 2, 0, 1, 2, 2, 1, 1, 1,
        ];
        let states = vec!["A".to_owned(), "B".to_owned()];
        let transitions = vec![vec![0.634, 0.366], vec![0.387, 0.613]];
        let emissions = vec![vec![0.532, 0.226, 0.241], vec![0.457, 0.192, 0.351]];

        let result = viterbi(observations, &states, transitions, emissions);
        assert_eq!(&result, "AAAAAAAAAAAAAABBBBBBBBBBBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBBBBAAAAAAAAAAAAAAAAAAAAABBBBBBBBBBAAA");
    }
}
