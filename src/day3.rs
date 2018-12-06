use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Claim {
    pub id: u32,
    pub offset: (u32, u32),
    pub extent: (u32, u32),
}

lazy_static! {
    static ref claim_regex: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

fn parse_claim(claim_str: &str) -> Claim {
    let caps = claim_regex.captures(claim_str).unwrap();
    Claim {
        id: caps[1].parse::<u32>().unwrap(),
        offset: (
            caps[2].parse::<u32>().unwrap(),
            caps[3].parse::<u32>().unwrap(),
        ),
        extent: (
            caps[4].parse::<u32>().unwrap(),
            caps[5].parse::<u32>().unwrap(),
        ),
    }
}

fn build_grid(claims: &[Claim]) -> HashMap<(u32, u32), u32> {
    let mut grid = HashMap::new();
    for &Claim {
        offset: (offset_x, offset_y),
        extent: (extent_x, extent_y),
        ..
    } in claims
    {
        for x in offset_x..offset_x + extent_x {
            for y in offset_y..offset_y + extent_y {
                *grid.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    grid
}

pub fn part1(input: &str) -> i32 {
    let claims = input.lines().map(parse_claim).collect::<Vec<Claim>>();
    let grid = build_grid(&claims);
    let mut n = 0;
    for &v in grid.values() {
        if v >= 2 {
            n += 1;
        }
    }
    n
}

fn claim_overlaps(
    &Claim {
        offset: (offset_x, offset_y),
        extent: (extent_x, extent_y),
        ..
    }: &Claim,
    grid: &HashMap<(u32, u32), u32>,
) -> bool {
    for x in offset_x..offset_x + extent_x {
        for y in offset_y..offset_y + extent_y {
            if grid[&(x, y)] >= 2 {
                return false;
            }
        }
    }
    true
}

pub fn part2(input: &str) -> u32 {
    let claims = input.lines().map(parse_claim).collect::<Vec<Claim>>();
    let grid = build_grid(&claims);
    for claim in &claims {
        if claim_overlaps(claim, &grid) {
            return claim.id;
        }
    }
    unreachable!()
}
