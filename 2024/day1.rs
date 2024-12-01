use std::collections::BTreeMap;

use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    // split each line into left and right numbers
    let (mut a, mut b): (Vec<u32>, Vec<u32>) = input
        .as_bytes()
        .split(|b| !b.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(lexical_core::parse)
        .map(|n: Result<u32, _>| n.expect("encountered non-numeric input"))
        .tuples()
        .multiunzip();

    // sort each list
    a.sort_unstable();
    b.sort_unstable();

    // total of distances between each pair
    std::iter::zip(a, b).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn part2(input: &str) -> u32 {
    // just a vec of the left numbers
    let mut l_list = vec![];

    // count the occurrences of each number on the right
    let mut r_counts = BTreeMap::new();

    let lines = input
        .as_bytes()
        .split(|b| !b.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(lexical_core::parse)
        .map(|n: Result<u32, _>| n.expect("encountered non-numeric input"))
        .tuples();

    for (left, right) in lines {
        l_list.push(left);
        r_counts.entry(right).and_modify(|count| *count += 1).or_insert(1);
    }

    // multiply each left number by the number of times it appears on the right
    l_list.into_iter().map(|n| n * r_counts.get(&n).unwrap_or(&0)).sum()
}
