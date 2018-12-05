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

    let mut f = File::open(input_files[&(day, part)]).expect("file open");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("file read");

    match (day, part) {
        ("1", "1") => println!("{}", day1::part1(&input)),
        ("1", "2") => println!("{}", day1::part2(&input)),
        ("2", "1") => println!("{}", day2::part1(&input)),
        ("2", "2") => println!("{}", day2::part2(&input)),
        ("3", "1") => println!("{}", day3::part1(&input)),
        ("3", "2") => println!("{}", day3::part2(&input)),
        ("4", "1") => println!("{}", day4::part1(&input)),
        ("4", "2") => println!("{}", day4::part2(&input)),
        ("5", "1") => println!("{}", day5::part1(&input)),
        ("5", "2") => println!("{}", day5::part2(&input)),
        (_, _) => unimplemented!(),
    }
}
