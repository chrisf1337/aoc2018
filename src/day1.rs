use std::collections::HashSet;

pub fn part1(input: &str) -> i32 {
    let mut freq = 0;
    for s in input.split_whitespace() {
        freq += s.parse::<i32>().unwrap();
    }
    freq
}

pub fn part2(input: &str) -> i32 {
    find_dup_freq(
        &input
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>(),
    )
}

pub fn find_dup_freq(input: &[i32]) -> i32 {
    let mut freq = 0;
    let mut i = 0;
    let mut hs = HashSet::new();
    loop {
        freq += input[i % input.len()];
        i += 1;
        if hs.contains(&freq) {
            return freq;
        }
        hs.insert(freq);
    }
}
