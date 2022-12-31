use std::collections::{HashMap, VecDeque};

use crate::utils::read_line;

pub fn solve_additive_phylogeny() {
    let n = read_line().parse().expect("number");
    let d: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            read_line()
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    for (i, j, k) in additive_phylogeny(&d) {
        println!("{i}->{j}:{k}");
    }
}

macro_rules! insert_both {
    ($g:expr, $from:expr, $to:expr, $value:expr) => {
        $g[$from].insert($to, $value);
        $g[$to].insert($from, $value);
    };
}

pub fn additive_phylogeny(d: &[Vec<usize>]) -> Vec<(usize, usize, usize)> {
    let n = d.len();
    let mut result: Vec<_> = (0..n).map(|_| HashMap::new()).collect();

    result[0].insert(1, d[0][1]);
    result[1].insert(0, d[1][0]);

    for j in 2..n {
        let i = if j > 0 { j - 1 } else { j + 1 };
        let (k, limb) = (0..j + 1)
            .filter(|k| i != *k && *k != j)
            .map(|k| (k, (d[i][j] + d[j][k] - d[i][k]) / 2))
            .min_by_key(|x| x.1)
            .unwrap();

        let x = d[i][j] - limb;

        let mut distance = vec![usize::MAX; result.len()];
        let mut parent = vec![0; result.len()];
        let mut q = VecDeque::new();

        distance[i] = 0;
        q.push_back(i);

        'outer: while let Some(mut current) = q.pop_front() {
            for (node, weight) in result[current].iter() {
                if usize::MAX != distance[*node] {
                    continue;
                }

                distance[*node] = distance[current] + weight;
                parent[*node] = current;
                q.push_back(*node);

                if *node != k {
                    continue;
                }

                let mut previous = *node;

                while x < distance[previous] {
                    current = previous;
                    previous = parent[current];
                }

                if x == distance[previous] {
                    insert_both!(result, previous, j, limb);
                } else {
                    result[previous].remove(&current);
                    result[current].remove(&previous);

                    result.push(HashMap::new());
                    let new = result.len() - 1;

                    insert_both!(result, j, new, limb);
                    insert_both!(result, previous, new, x - distance[previous]);
                    insert_both!(result, current, new, distance[current] - x);
                }

                break 'outer;
            }
        }
    }

    result
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().map(|(k, v)| (i, *k, *v)).collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let d = vec![
            vec![0, 13, 21, 22],
            vec![13, 0, 12, 13],
            vec![21, 12, 0, 13],
            vec![22, 13, 13, 0],
        ];

        assert_eq!(
            super::additive_phylogeny(&d),
            vec![
                (0, 4, 11),
                (1, 4, 2),
                (2, 5, 6),
                (3, 5, 7),
                (4, 0, 11),
                (4, 1, 2),
                (4, 5, 4),
                (5, 3, 7),
                (5, 2, 6),
                (5, 4, 4),
            ]
        );
    }
}
