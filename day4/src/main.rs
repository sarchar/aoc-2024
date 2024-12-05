use std::io::{self, Read};

fn check_puzzle(grid: &Vec<String>, check_right: bool, check_down: bool) -> usize {
    let mut count = 0;
    const XMAS: &str = "XMAS";
    
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // check right
            if check_right && grid[y][x..].starts_with(XMAS) {
                count += 1;
            }

            if y <= grid.len() - 4 {
                // check down
                if check_down && grid[y][x..x+1].starts_with(&XMAS[0..1]) && grid[y+1][x..x+1].starts_with(&XMAS[1..2]) && grid[y+2][x..x+1].starts_with(&XMAS[2..3]) && grid[y+3][x..x+1].starts_with(&XMAS[3..4]) {
                    count += 1;
                }

                if x <= grid[y].len() - 4 {
                    // check down-right
                    if grid[y][x..x+1].starts_with(&XMAS[0..1]) && grid[y+1][x+1..x+2].starts_with(&XMAS[1..2]) && grid[y+2][x+2..x+3].starts_with(&XMAS[2..3]) && grid[y+3][x+3..x+4].starts_with(&XMAS[3..4]) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[allow(dead_code)]
fn part1(grid: Vec<String>) {
    let mut count = check_puzzle(&grid, true, true);

    // flip the right across the vertical axis
    let grid = grid.into_iter().map(|s| s.chars().rev().collect::<String>()).collect::<Vec<_>>();
    count += check_puzzle(&grid, true, false);

    // flip the grid upside down
    let grid = grid.into_iter().rev().collect::<Vec<_>>();
    count += check_puzzle(&grid, false, true);

    // flip the grid across the vertical axis again
    let grid = grid.into_iter().map(|s| s.chars().rev().collect::<String>()).collect::<Vec<_>>();
    count += check_puzzle(&grid, false, false);
    
    println!("{}", count);
}

fn part2(grid: Vec<String>) {
    let mut count = 0;
    
    for y in 0..grid.len() - 2 {
        for x in 0..grid[y].len() - 2 {
            if (grid[y][x..].starts_with("M") && grid[y+2][x+2..].starts_with("S"))
                || (grid[y][x..].starts_with("S") && grid[y+2][x+2..].starts_with("M"))
            {
                if (grid[y][x+2..].starts_with("M") && grid[y+2][x..].starts_with("S"))
                    || (grid[y][x+2..].starts_with("S") && grid[y+2][x..].starts_with("M")) {
                    if grid[y+1][x+1..].starts_with("A") {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{}", count);
}

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let grid = input.lines().map(|s| String::from(s)).collect::<Vec<_>>();

    // part1(grid);
    part2(grid);
}
