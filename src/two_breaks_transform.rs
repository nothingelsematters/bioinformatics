use std::collections::BTreeMap as HashMap;

use crate::utils::read_line;

fn get_i32<T>(slice: &[T], index: i32) -> &T {
    let len = slice.len();
    &slice[(index + len as i32) as usize % len]
}

fn get_mut_i32<T>(slice: &mut [T], index: i32) -> &mut T {
    let len = slice.len();
    &mut slice[(index + len as i32) as usize % len]
}

pub fn solve_two_breaks_transformation() {
    fn read_genome() -> Vec<i32> {
        read_line()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(' ')
            .map(|x| x[1..].parse::<i32>().unwrap() * if x.starts_with('+') { 1 } else { -1 })
            .collect()
    }

    let left = read_genome();
    let right = read_genome();

    two_breaks_transformation(&left, &right)
        .into_iter()
        .for_each(|answer| {
            answer
                .into_iter()
                .filter(|numbers| !numbers.is_empty())
                .for_each(|numbers| {
                    print!("(");
                    numbers.iter().enumerate().for_each(|(i, n)| {
                        print!(
                            "{}{n}{}",
                            if *n > 0 { "+" } else { "" },
                            if i != numbers.len() - 1 { " " } else { "" },
                        );
                    });
                    print!(")");
                });

            println!()
        });
}

pub fn two_breaks_transformation(left: &[i32], right: &[i32]) -> Vec<Vec<Vec<i32>>> {
    let mut left_graph = get_graph(left);
    let right_graph = get_graph(right);

    let mut cycles = get_cycles(&left_graph, &right_graph);
    let mut transformations = vec![vec![left.to_vec()]];

    while cycles.iter().any(|x| x.len() > 2) {
        let mut index = None;
        let mut new_cycles = vec![];

        let cycle = cycles.iter().enumerate().find(|(_, cycle)| cycle.len() > 3);
        if let Some((i, cycle)) = cycle {
            new_cycles.push(vec![(cycle[2].0, cycle[0].1), cycle[1]]);
            let mut new_cycle = vec![(cycle[0].0, cycle[2].1)];
            new_cycle.append(&mut cycle[3..].to_vec());
            new_cycles.push(new_cycle);
            index = Some(i);
        }

        cycles
            .into_iter()
            .enumerate()
            .filter(|(i, _)| Some(*i) != index)
            .for_each(|(_, cycle)| new_cycles.push(cycle));
        cycles = new_cycles;
        left_graph = HashMap::new();

        cycles.iter().for_each(|cycle| {
            let mut use_left = true;

            cycle.iter().for_each(|edge| {
                if use_left {
                    left_graph.insert(edge.0, edge.1);
                    left_graph.insert(edge.1, edge.0);
                }
                use_left = !use_left;
            });
        });
        transformations.push(get_transformation(&left_graph));
    }

    transformations
}

fn start(x: i32) -> i32 {
    if x >= 0 {
        x * 2
    } else {
        -x * 2 - 1
    }
}

fn end(x: i32) -> i32 {
    if x >= 0 {
        x * 2 - 1
    } else {
        -x * 2
    }
}

fn get_graph(genome: &[i32]) -> HashMap<i32, i32> {
    let mut graph = HashMap::new();
    graph.insert(0, end(genome[0]));
    graph.insert(end(genome[0]), 0);

    for i in 0..genome.len() {
        if i == genome.len() - 1 {
            graph.insert(-1, start(genome[i]));
            graph.insert(start(genome[i]), -1);
        } else {
            graph.insert(start(genome[i]), end(genome[i + 1]));
            graph.insert(end(genome[i + 1]), start(genome[i]));
        }
    }

    graph
}

fn get_cycles(left: &HashMap<i32, i32>, right: &HashMap<i32, i32>) -> Vec<Vec<(i32, i32)>> {
    let mut cycles = Vec::new();
    let mut used = vec![false; left.len()];

    for mut i in left.keys().cloned() {
        if *get_i32(&used, i) {
            continue;
        }

        let mut new_cycle = vec![];
        let mut use_left = true;

        while !get_i32(&used, i) {
            let new_ind = if use_left { left[&i] } else { right[&i] };
            use_left = !use_left;
            new_cycle.push((i, new_ind));

            *get_mut_i32(&mut used, i) = true;
            i = new_ind;
        }

        cycles.push(new_cycle);
    }

    cycles
}

fn get_transformation(left: &HashMap<i32, i32>) -> Vec<Vec<i32>> {
    let mut genome = vec![vec![]];

    let mut visited = vec![false; left.len()];
    *visited.last_mut().unwrap() = true;
    let mut i = 0;

    while genome.iter().map(|x| x.len()).sum::<usize>() < left.len() / 2 - 1 {
        if visited[i] {
            genome.push(vec![]);

            visited
                .iter()
                .enumerate()
                .take(left.len() - 1)
                .filter(|(_, v)| !**v)
                .for_each(|(j, _)| i = j);
        }

        let index = &(i as i32);
        visited[i] = true;
        *get_mut_i32(&mut visited, left[index]) = true;

        if left[index] != -1 && left[index] != left.len() as i32 - 1 {
            genome.last_mut().unwrap().push(if left[index] % 2 == 1 {
                (left[index] + 1) / 2
            } else {
                -left[index] / 2
            });

            i = start(*genome.last().unwrap().last().unwrap()) as usize;
        }
    }

    genome
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_test() {
        let left = vec![1, -2, -3, 4];
        let right = vec![1, 2, -4, -3];

        assert_eq!(
            super::two_breaks_transformation(&left, &right),
            vec![
                vec![vec![1, -2, -3, 4]],
                vec![vec![1], vec![-3, 4, -2]],
                vec![vec![1, 4, -2, -3]],
                vec![vec![1, 2, -4, -3]],
            ],
        )
    }
}
