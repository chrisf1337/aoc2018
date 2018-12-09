use crate::marblecircle::MarbleCircle;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref marble_regex: Regex =
        Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
}

#[inline]
fn modulo(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}

fn insert_marble(marbles: &mut Vec<u32>, i: &mut usize, m: u32) {
    *i = (*i + 2) % marbles.len();
    marbles.insert(*i, m);
}

fn remove_marble(marbles: &mut Vec<u32>, i: &mut usize) -> u32 {
    *i = modulo(*i as isize - 7, marbles.len() as isize) as usize;
    marbles.remove(*i)
}

fn insert_marble2(marbles: &mut MarbleCircle<u32>, m: u32) {
    marbles.move_right();
    marbles.move_right();
    marbles.insert(m);
}

fn remove_marble2(marbles: &mut MarbleCircle<u32>) -> u32 {
    for _ in 0..7 {
        marbles.move_left();
    }
    marbles.remove()
}

pub fn part1(input: &str) -> u32 {
    let caps = marble_regex.captures(input).unwrap();
    let n_players = caps[1].parse::<usize>().unwrap();
    let n_marbles = caps[2].parse::<u32>().unwrap();
    let mut marbles = vec![0];

    let mut scores = vec![0; n_players];
    let mut current_player = 0;

    let mut i = 0;
    for marble in 1..=n_marbles {
        if marble % 23 == 0 {
            let removed = remove_marble(&mut marbles, &mut i);
            scores[current_player] += marble + removed;
        } else {
            insert_marble(&mut marbles, &mut i, marble);
        }
        current_player = (current_player + 1) % scores.len();
    }

    *scores.iter().max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let caps = marble_regex.captures(input).unwrap();
    let n_players = caps[1].parse::<usize>().unwrap();
    let n_marbles = caps[2].parse::<u32>().unwrap();
    let mut marbles = MarbleCircle::new(0);

    let mut scores = vec![0; n_players];
    let mut current_player = 0;

    for marble in 1..=n_marbles * 100 {
        if marble % 1000 == 0 {
            println!("{}", marble);
        }
        if marble % 23 == 0 {
            let removed = remove_marble2(&mut marbles);
            scores[current_player] += marble + removed;
        } else {
            insert_marble2(&mut marbles, marble);
        }
        // println!("m {} {:?}", marble, marbles);

        current_player = (current_player + 1) % scores.len();
    }

    *scores.iter().max().unwrap()
}
