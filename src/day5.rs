fn can_react(a: char, b: char) -> bool {
    let a2 = a.to_ascii_lowercase();
    let b2 = b.to_ascii_lowercase();
    a2 == b2
        && ((a.is_ascii_uppercase() && b.is_ascii_lowercase())
            || (a.is_ascii_lowercase() && b.is_ascii_uppercase()))
}

fn react_polymer(polymer: &[char]) -> Vec<char> {
    let mut changed = true;
    let mut polymer = polymer.to_vec();
    while changed {
        if polymer.is_empty() {
            return vec![];
        }
        let mut i = 0;
        changed = false;
        while i < polymer.len() - 1 {
            if can_react(polymer[i], polymer[i + 1]) {
                polymer.remove(i);
                polymer.remove(i);
                changed = true;
            } else {
                i += 1;
            }
        }
    }

    polymer
}

pub fn part1(input: &str) -> usize {
    react_polymer(&input.trim().chars().collect::<Vec<char>>()).len()
}

pub fn part2(input: &str) -> usize {
    let polymer = input.trim().chars().collect::<Vec<char>>();
    let mut min_len = std::usize::MAX;
    for c in 'a' as u8..'z' as u8 {
        let c = c as char;
        let new_polymer = polymer
            .clone()
            .into_iter()
            .filter(|&x| x != c && x.to_ascii_lowercase() != c)
            .collect::<Vec<char>>();
        min_len = usize::min(min_len, react_polymer(&new_polymer).len());
    }
    min_len
}
