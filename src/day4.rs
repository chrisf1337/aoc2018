use chrono::{NaiveDateTime, Timelike};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Entry {
    pub time: NaiveDateTime,
    pub ty: EntryType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum EntryType {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

impl EntryType {
    fn is_begins_shift(&self) -> bool {
        match self {
            EntryType::BeginsShift(_) => true,
            _ => false,
        }
    }
}

lazy_static! {
    static ref timestamp_regex: Regex = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\]").unwrap();
    static ref shift_regex: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
}

fn parse_entry(line: &str) -> Entry {
    let time_str = &timestamp_regex.captures(line).unwrap()[1];
    let time = NaiveDateTime::parse_from_str(time_str, "%F %R").unwrap();
    if let Some(caps) = shift_regex.captures(line) {
        Entry {
            time,
            ty: EntryType::BeginsShift(caps[1].parse().unwrap()),
        }
    } else if line.contains("falls asleep") {
        Entry {
            time,
            ty: EntryType::FallsAsleep,
        }
    } else if line.contains("wakes up") {
        Entry {
            time,
            ty: EntryType::WakesUp,
        }
    } else {
        unreachable!()
    }
}

fn compile_sleep_history(mut entries: Vec<Entry>) -> HashMap<u32, [u32; 60]> {
    entries.sort_unstable_by_key(|e| e.time);

    let mut sleep_history: HashMap<u32, [u32; 60]> = HashMap::new();
    let mut i = 0;
    while i < entries.len() {
        // get guard id
        let history = if let EntryType::BeginsShift(id) = entries[i].ty {
            sleep_history.entry(id).or_insert([0; 60])
        } else {
            unreachable!()
        };
        i += 1;

        while i < entries.len() && !entries[i].ty.is_begins_shift() {
            // must be falls asleep
            let falls_asleep_sec = entries[i].time.time().minute();
            let wakes_up_sec = entries[i + 1].time.time().minute();
            for s in falls_asleep_sec..wakes_up_sec {
                history[s as usize] += 1;
            }
            i += 2;
        }
    }

    sleep_history
}

pub fn part1(input: &str) -> u32 {
    let entries: Vec<Entry> = input.lines().map(parse_entry).collect();
    let sleep_history = compile_sleep_history(entries);

    let mut max_id = 0;
    let mut max_mins_asleep = 0;
    for (&id, history) in &sleep_history {
        let total_mins_asleep: u32 = history.iter().sum();
        if total_mins_asleep > max_mins_asleep {
            max_mins_asleep = total_mins_asleep;
            max_id = id;
        }
    }

    let history = sleep_history[&max_id];
    let most_freq_min = history
        .iter()
        .enumerate()
        .max_by_key(|(_, &freq)| freq)
        .unwrap()
        .0 as u32;

    max_id * most_freq_min
}

pub fn part2(input: &str) -> u32 {
    let entries: Vec<Entry> = input.lines().map(parse_entry).collect();
    let sleep_history = compile_sleep_history(entries);

    let mut max_id = 0;
    let mut max_min = 0;
    let mut max_freq = 0;
    for (&id, history) in &sleep_history {
        let (min, freq) = history
            .into_iter()
            .enumerate()
            .max_by_key(|(_, &freq)| freq)
            .unwrap();
        if *freq > max_freq {
            max_min = min as u32;
            max_freq = *freq;
            max_id = id;
        }
    }

    max_id * max_min
}
