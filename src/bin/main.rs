use aoc2018::*;
use clap::{App, Arg};
use std::{collections::HashMap, fs::File, io::prelude::*};

fn main() {
    let matches = App::new("aoc2018")
        .arg(Arg::with_name("day").takes_value(true).required(true))
        .arg(Arg::with_name("part").takes_value(true).required(true))
        .get_matches();
    let day = matches.value_of("day").unwrap();
    let part = matches.value_of("part").unwrap();

    let mut input_files = HashMap::new();
    input_files.insert(("1", "1"), "inputs/day1_1.in.txt");
    input_files.insert(("1", "2"), "inputs/day1_1.in.txt");
    input_files.insert(("2", "1"), "inputs/day2_1.in.txt");
    input_files.insert(("2", "2"), "inputs/day2_1.in.txt");
    input_files.insert(("3", "1"), "inputs/day3_1.in.txt");
    input_files.insert(("3", "2"), "inputs/day3_1.in.txt");
    input_files.insert(("4", "1"), "inputs/day4_1.in.txt");
    input_files.insert(("4", "2"), "inputs/day4_1.in.txt");
    input_files.insert(("5", "1"), "inputs/day5_1.in.txt");
    input_files.insert(("5", "2"), "inputs/day5_1.in.txt");
    input_files.insert(("6", "1"), "inputs/day6_1.in.txt");
    input_files.insert(("6", "2"), "inputs/day6_1.in.txt");
    input_files.insert(("7", "1"), "inputs/day7_1.in.txt");
    input_files.insert(("7", "2"), "inputs/day7_1.in.txt");
    input_files.insert(("8", "1"), "inputs/day8_1.in.txt");
    input_files.insert(("8", "2"), "inputs/day8_1.in.txt");
    input_files.insert(("9", "1"), "inputs/day9_1.in.txt");
    input_files.insert(("9", "2"), "inputs/day9_1.in.txt");
    input_files.insert(("10", "1"), "inputs/day10_1.in.txt");
    input_files.insert(("10", "2"), "inputs/day10_1.in.txt");

    let input = if let Some(filepath) = input_files.get(&(day, part)) {
        let mut f = File::open(filepath).expect("file open");
        let mut input = String::new();
        f.read_to_string(&mut input).expect("file read");
        input
    } else {
        "".to_string()
    };

    match (day, part) {
        ("1", "1") => println!("{}", day1::part1(&input)),
        ("1", "2") => println!("{}", day1::part2(&input)),
        ("2", "1") => println!("{}", day2::part1(&input)),
        ("2", "2") => println!("{}", day2::part2(&input)),
        ("3", "1") => println!("{}", day3::part1(&input)),
        ("3", "2") => println!("{}", day3::part2(&input)),
        ("4", "1") => println!("{}", day4::part1(&input)),
        ("4", "2") => println!("{}", day4::part2(&input)),
        ("6", "1") => println!("{}", day6::part1(&input)),
        ("6", "2") => println!("{}", day6::part2(&input)),
        ("7", "1") => println!("{}", day7::part1(&input)),
        ("7", "2") => println!("{}", day7::part2(&input)),
        ("8", "1") => println!("{}", day8::part1(&input)),
        ("8", "2") => println!("{}", day8::part2(&input)),
        ("9", "1") => println!("{}", day9::part1(&input)),
        ("9", "2") => println!("{}", day9::part2(&input)),
        ("10", "1") => println!("{}", day10::part1(&input)),
        ("10", "2") => println!("{}", day10::part2(&input)),
        ("11", "1") => println!("{:?}", day11::part1(5153)),
        ("11", "2") => println!("{:?}", day11::part2(5153)),
        (_, _) => unimplemented!(),
    }
}
