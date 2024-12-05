use std::collections::HashMap;
use std::io;

#[allow(dead_code)]
fn part1(mut vec_a: Vec<i64>, mut vec_b: Vec<i64>) {
    vec_a.sort_unstable();
    vec_b.sort_unstable();

    let mut total = 0;
    for (a, b) in vec_a.iter().zip(vec_b.iter()) {
        total += (b - a).abs();
    }

    println!("{}", total);
}

fn part2(vec_a: Vec<i64>, vec_b: Vec<i64>) {
    let mut vec_b_counts = HashMap::new();

    for v in vec_b {
        *vec_b_counts.entry(v).or_insert(0) += 1;
    }

    let mut total = 0;
    for v in vec_a {
        let count = *vec_b_counts.get(&v).unwrap_or(&0);
        total += v * count;
    }

    println!("{}", total);
}


fn main() {
    let mut line = String::new();
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();

    while io::stdin().read_line(&mut line).is_ok_and(|v| v != 0) {
        let line = std::mem::take(&mut line);
        let parts = line.split_whitespace().map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>();
        vec_a.push(parts[0]);
        vec_b.push(parts[1]);
    }

    // part1(vec_a, vec_b);
    part2(vec_a, vec_b);
}
