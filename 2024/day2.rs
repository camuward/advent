use itertools::Itertools;

fn iter_nums(line: &str) -> impl Iterator<Item = u32> + '_ {
    line.split_ascii_whitespace().map(|s| s.parse().expect("encountered non-numeric input"))
}

fn is_safe(it: impl Iterator<Item = u32>) -> bool {
    let mut asc_desc = None;
    it.tuple_windows().map(|(a, b)| b as i32 - a as i32).all(|diff| {
        if let Some(is_ascending) = asc_desc {
            if is_ascending != diff.is_positive() {
                return false;
            }
        } else {
            asc_desc = Some(diff.is_positive());
        }
        0 < diff.abs() && diff.abs() <= 3
    })
}

pub fn part1(input: &str) -> usize {
    input.lines().filter(|line| is_safe(iter_nums(line))).count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<_> = iter_nums(line).collect();
            is_safe(nums.iter().copied())
                || (0..nums.len())
                    .map(|skip| {
                        nums.iter()
                            .enumerate()
                            .filter(move |&(i, _)| i == skip)
                            .map(|(_, &num)| num)
                    }) // LETS GOO O(n^2) baby
                    .any(is_safe)
        })
        .count()
}
