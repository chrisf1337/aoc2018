use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn mdist(&self, p: Point) -> i32 {
        i32::abs(self.x - p.x) + i32::abs(self.y - p.y)
    }

    fn closest_point(&self, points: &[Point]) -> Option<Point> {
        let mut min_mdist = std::i32::MAX;
        let mut closest = points[0];
        let mut dupe = false;
        for &p in points {
            let mdist = self.mdist(p);
            if mdist < min_mdist {
                min_mdist = self.mdist(p);
                closest = p;
                dupe = false;
            } else if mdist == min_mdist {
                dupe = true;
            }
        }
        if !dupe {
            Some(closest)
        } else {
            None
        }
    }
}

lazy_static! {
    static ref point_regex: Regex = Regex::new(r"(\d+), (\d+)").unwrap();
}

fn parse_point(line: &str) -> Point {
    let caps = point_regex.captures(line).unwrap();
    Point {
        x: caps[1].parse().unwrap(),
        y: caps[2].parse().unwrap(),
    }
}

fn bounding_box_points(
    (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y }): (Point, Point),
) -> Vec<Point> {
    let mut bbox_points = vec![];
    for x in min_x..max_x {
        bbox_points.push(Point::new(x, max_y));
    }
    for y in (min_y..max_y).rev() {
        bbox_points.push(Point::new(max_x, y));
    }
    for x in (min_x..max_x).rev() {
        bbox_points.push(Point::new(x, min_y));
    }
    for y in min_y..max_y {
        bbox_points.push(Point::new(min_x, y));
    }
    bbox_points
}

fn bounding_box(points: &[Point]) -> (Point, Point) {
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for &Point { x, y } in points {
        min_x = cmp::min(x, min_x);
        min_y = cmp::min(y, min_y);
        max_x = cmp::max(x, max_x);
        max_y = cmp::max(y, max_y);
    }

    (Point::new(min_x, min_y), Point::new(max_x, max_y))
}

fn fill_points(
    (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y }): (Point, Point),
) -> Vec<Point> {
    let mut points = vec![];
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            points.push(Point { x, y })
        }
    }
    points
}

pub fn part1(input: &str) -> u32 {
    let points = input
        .lines()
        .map(|l| parse_point(l))
        .collect::<Vec<Point>>();

    let mut infinite_points = HashSet::new();
    let bbox = bounding_box(&points);
    for p in bounding_box_points(bbox) {
        if let Some(input_point) = p.closest_point(&points) {
            infinite_points.insert(input_point);
        }
    }

    let mut n_closest = HashMap::new();
    for p in fill_points(bbox) {
        if let Some(p) = p.closest_point(&points) {
            if !infinite_points.contains(&p) {
                *n_closest.entry(p).or_insert(0) += 1;
            }
        }
    }

    println!("{:#?}", n_closest);

    let mut max_n_closest = 0;
    for &n in n_closest.values() {
        if n > max_n_closest {
            max_n_closest = n;
        }
    }

    max_n_closest
}

pub fn part2(input: &str) -> u32 {
    let points = input
        .lines()
        .map(|l| parse_point(l))
        .collect::<Vec<Point>>();
    let cutoff = 10000;
    let mut region_size = 0;
    for p in fill_points((Point::new(-cutoff, -cutoff), Point::new(cutoff, cutoff))) {
        let sum = points.iter().map(|&pt| pt.mdist(p)).sum::<i32>();
        if sum < cutoff {
            region_size += 1;
        }
    }

    region_size
}
