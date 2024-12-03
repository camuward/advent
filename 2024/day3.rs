use std::sync::LazyLock;

use regex::Regex;

static RE_MUL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
static RE_DONT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s).+?(do|don't)\(\)").unwrap());

pub fn part1(input: &str) -> u64 {
    RE_MUL
        .captures_iter(input)
        .map(|c| c.extract())
        .filter_map(|(_, [a, b])| Some((a.parse().ok()?, b.parse().ok()?)))
        .map(|(a, b)| u64::wrapping_mul(a, b))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut currently_doing = true;
    let all_but_last = RE_DONT
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(sequence, [do_dont])| {
            match std::mem::replace(&mut currently_doing, do_dont == "do") {
                true => part1(sequence),
                false => 0,
            }
        })
        .sum::<u64>();

    if currently_doing {
        let last = input.rsplit("do()").map(part1).next().unwrap_or(0);
        all_but_last + last
    } else {
        all_but_last
    }
}
