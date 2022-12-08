use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

const RADIX: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|r| r.unwrap()).collect();

    // Logic;
    let rows = lines.len();
    let cols = lines[0].len();
    let mut heights: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
    for row in 0..rows {
        let line: Vec<i32> = lines[row].chars().map(|c| c.to_digit(RADIX).unwrap() as i32).collect();
        for col in 0..cols {
            heights[row][col] = line[col];
        }
    }
    let heights = heights;

    // Part 1

    let mut visible: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    fn check_line(heights: &Vec<Vec<i32>>, visible: &mut Vec<Vec<bool>>, start_row: usize, start_col: usize, delta_row: i32, delta_col: i32) {
        let mut threshold = -1;
        let mut row = start_row as i32;
        let mut col = start_col as i32;
        let rows = visible.len() as i32;
        let cols = visible[0].len() as i32;
        while row >= 0 && row < rows && col >= 0 && col < cols {
            if heights[row as usize][col as usize] > threshold {
                threshold = heights[row as usize][col as usize];
                visible[row as usize][col as usize] = true;
            }
            row = row as i32 + delta_row;
            col = col as i32 + delta_col;
        }
    }

    // Check horizontally
    for row in 0..rows {
        check_line(&heights, &mut visible, row, 0, 0, 1);
        check_line(&heights, &mut visible, row, cols - 1, 0, -1);
    }
    // Check vertically
    for col in 0..cols {
        check_line(&heights, &mut visible, 0, col, 1, 0);
        check_line(&heights, &mut visible, rows - 1, col, -1, 0);
    }

    // Count up visible trees
    let mut visible_trees = 0;
    for row in 0..rows {
        for col in 0..cols {
            if visible[row][col] {
                visible_trees += 1;
            }
        }
    }

    println!("Number of visible trees: {}", visible_trees);

    // Part 2

    fn count_visible(heights: &Vec<Vec<i32>>, start_row: i32, start_col: i32, delta_row: i32, delta_col: i32) -> i32 {
        let mut count = 0;
        let rows = heights.len() as i32;
        let cols = heights[0].len() as i32;
        let mut row = start_row + delta_row;
        let mut col = start_col + delta_col;
        let limit = heights[start_row as usize][start_col as usize];
        while row >= 0 && row < rows && col >= 0 && col < cols {
            count += 1;
            if heights[row as usize][col as usize] >= limit {
                break;
            }
            row += delta_row;
            col += delta_col;
        }
        return count;
    }

    let mut best_scenic_score = 0;
    for row in 0..rows as i32 {
        for col in 0..cols as i32 {
            let left = count_visible(&heights, row, col, 0, -1);
            let right = count_visible(&heights, row, col, 0, 1);
            let up = count_visible(&heights, row, col, -1, 0);
            let down = count_visible(&heights, row, col, 1, 0);
            let scenic_score = left * right * up * down;
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }

    println!("Best scenic score: {}", best_scenic_score);
}