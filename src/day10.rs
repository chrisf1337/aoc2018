use crate::na::{Point2, Vector2};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp,
    collections::{HashMap, HashSet},
};

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"position=<( *-?\d+), ( *-?\d+)> velocity=<( *-?\d+), ( *-?\d+)>").unwrap();
}

type Pt2i = Point2<i32>;
type Vec2i = Vector2<i32>;

fn bounding_box(grid: &HashMap<Vec2i, Vec<Pt2i>>) -> (Pt2i, Pt2i) {
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut max_y = std::i32::MIN;
    for positions in grid.values() {
        for pos in positions {
            min_x = cmp::min(min_x, pos.x);
            min_y = cmp::min(min_y, pos.y);
            max_x = cmp::max(max_x, pos.x);
            max_y = cmp::max(max_y, pos.y);
        }
    }
    (Pt2i::new(min_x, min_y), Pt2i::new(max_x, max_y))
}

fn update_grid(grid: &mut HashMap<Vec2i, Vec<Pt2i>>) {
    for (vel, positions) in grid {
        for pos in positions {
            *pos += vel;
        }
    }
}

fn print_grid(grid: &HashMap<Vec2i, Vec<Pt2i>>) {
    let mut points: HashSet<Pt2i> = HashSet::new();;
    for pts in grid.values() {
        points.extend(pts.iter());
    }
    let bbox = bounding_box(grid);
    for r in bbox.0.y..=bbox.1.y {
        for c in bbox.0.x..=bbox.1.x {
            if points.contains(&Pt2i::new(c, r)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

pub fn part1(input: &str) -> u32 {
    let points = input
        .lines()
        .map(|line| {
            let caps = LINE_REGEX.captures(line).unwrap();
            (
                Pt2i::new(
                    caps[1].trim().parse().unwrap(),
                    caps[2].trim().parse().unwrap(),
                ),
                Vec2i::new(
                    caps[3].trim().parse().unwrap(),
                    caps[4].trim().parse().unwrap(),
                ),
            )
        })
        .collect::<Vec<(Pt2i, Vec2i)>>();

    let mut grid = HashMap::new();
    for (pos, vel) in points {
        grid.entry(vel).or_insert(vec![]).push(pos);
    }

    let mut min_bbox_area = std::i64::MAX;
    for i in 0..=10557 {
        update_grid(&mut grid);
        let (bbox_min, bbox_max) = bounding_box(&grid);
        let bbox = (bbox_max.x - bbox_min.x + 1, bbox_max.y - bbox_min.y + 1);
        if bbox.0 as i64 * bbox.1 as i64 <= min_bbox_area {
            min_bbox_area = bbox.0 as i64 * bbox.1 as i64;
            println!("new {} {}", i, min_bbox_area);
        }
    }
    print_grid(&grid);
    0
}

pub fn part2(input: &str) -> u32 {
    0
}
