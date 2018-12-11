use std::cmp::max;

fn hundreds_digit(i: i32) -> i32 {
    if i < 100 {
        0
    } else {
        (i / 100) % 10
    }
}

fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    hundreds_digit((rack_id * y + serial) * rack_id) - 5
}

fn area_power_level(grid: &[[i32; 300]], x: usize, y: usize, size: usize) -> i32 {
    let mut sum = 0;
    for x in x..x + size {
        for y in y..y + size {
            sum += grid[x][y];
        }
    }
    sum
}

fn area_power_level_prev(grid: &[[i32; 300]], x: usize, y: usize, size: usize, prev: i32) -> i32 {
    let mut prev_row = 0;
    for x in x..x + size {
        prev_row += grid[x][y - 1];
    }
    let mut next_row = 0;
    for x in x..x + size {
        next_row += grid[x][y + size - 1];
    }
    prev - prev_row + next_row
}

pub fn part1(serial: i32) -> (usize, usize) {
    let mut grid = [[0i32; 300]; 300];
    for x in 1..=300 {
        for y in 1..=300 {
            grid[x - 1][y - 1] = power_level(x as i32, y as i32, serial);
        }
    }

    let mut max_power = std::i32::MIN;
    let mut max_power_coord = (0, 0);
    for x in 0..297 {
        for y in 0..297 {
            let power_lv = area_power_level(&grid, x, y, 3);
            if power_lv > max_power {
                max_power = power_lv;
                max_power_coord = (x, y);
            }
        }
    }

    (max_power_coord.0 + 1, max_power_coord.1 + 1)
}

pub fn part2(serial: i32) -> (usize, usize, usize) {
    let mut grid = [[0i32; 300]; 300];
    for x in 1..=300 {
        for y in 1..=300 {
            grid[x - 1][y - 1] = power_level(x as i32, y as i32, serial);
        }
    }

    let mut max_power = std::i32::MIN;
    let mut max_power_coord = (0, 0, 0);
    for size in 1..300 {
        for x in 1..=300 - size {
            let mut power_lv = area_power_level(&grid, x, 0, size);
            if power_lv > max_power {
                max_power = power_lv;
                max_power_coord = (x, 0, size);
            }
            for y in 1..=300 - size {
                power_lv = area_power_level_prev(&grid, x, y, size, power_lv);
                if power_lv > max_power {
                    max_power = power_lv;
                    max_power_coord = (x, y, size);
                }
            }
        }
    }

    (
        max_power_coord.0 + 1,
        max_power_coord.1 + 1,
        max_power_coord.2,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
