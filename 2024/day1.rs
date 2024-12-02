use std::collections::{BTreeMap, BinaryHeap};

use itertools::Itertools;

fn nums_per_line(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse().expect("encountered non-numeric input"))
        .tuples()
}

pub fn part1(input: &str) -> u32 {
    // split each line into list of left and right numbers (sorted)
    let (a, b): (BinaryHeap<u32>, BinaryHeap<u32>) = nums_per_line(input).unzip();
    // total of distances between each pair
    std::iter::zip(a, b).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn part2(input: &str) -> u32 {
    // just a vec of the left numbers
    let mut l_list = vec![];
    // count the occurrences of each number on the right
    let mut r_counts = BTreeMap::new();

    for (left, right) in nums_per_line(input) {
        l_list.push(left);
        r_counts.entry(right).and_modify(|count| *count += 1).or_insert(1);
    }

    // multiply each left number by the number of times it appears on the right
    l_list.into_iter().map(|n| n * r_counts.get(&n).unwrap_or(&0)).sum()
}
