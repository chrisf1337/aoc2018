use std::collections::HashMap;

fn has_exactly_2_3_letters(word: &str) -> (bool, bool) {
    let mut chars = HashMap::new();
    for c in word.chars() {
        *chars.entry(c).or_insert(0) += 1;
    }
    let mut has_two_letters = false;
    let mut has_three_letters = false;
    for &i in chars.values() {
        if i == 2 {
            has_two_letters = true;
        } else if i == 3 {
            has_three_letters = true;
        }
    }
    (has_two_letters, has_three_letters)
}

pub fn day2_1(input: &str) -> i32 {
    let mut two_letters = 0;
    let mut three_letters = 0;
    for s in input.split_whitespace() {
        let (has_two_letters, has_three_letters) = has_exactly_2_3_letters(s);
        if has_two_letters {
            two_letters += 1;
        }
        if has_three_letters {
            three_letters += 1;
        }
    }
    two_letters * three_letters
}

fn are_close(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut n_same = 0;
    let mut diff_i = 0;
    for (i, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate() {
        if c1 == c2 {
            n_same += 1;
        } else {
            diff_i = i;
        }
    }
    if n_same == s1.len() - 1 {
        Some(diff_i)
    } else {
        None
    }
}

pub fn day2_2(input: &str) -> String {
    let ids = input
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();
    for i in 0..ids.len() {
        for j in i..ids.len() {
            if let Some(diff_i) = are_close(&ids[i], &ids[j]) {
                let mut s = ids[i].clone();
                s.remove(diff_i);
                return s;
            }
        }
    }
    unreachable!();
}
