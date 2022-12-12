use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|r| r.unwrap()).collect();

    // Logic:
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    struct Pos {
        row: usize,
        col: usize,
    }

    let mut heights: Vec<Vec<u8>> = Vec::new();
    let mut start = Pos { row: 0, col: 0 };
    let mut end = Pos { row: 0, col: 0 };
    for (row_no, line) in lines.iter().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (col_no, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Pos { row: row_no, col: col_no };
                    row.push(0);
                }
                'E' => {
                    end = Pos { row: row_no, col: col_no };
                    row.push('z' as u8 - 'a' as u8);
                }
                _ => row.push(c as u8 - 'a' as u8),
            };
        }
        heights.push(row);
    }
    let rows = heights.len();
    let cols = heights[0].len();


    // Breadth first search
    let mut dist: Vec<Vec<i32>> = vec![vec![-1; cols]; rows];
    dist[start.row][start.col] = 0;
    let mut next = VecDeque::from([start]);
    while !next.is_empty() {
        let pos = next.pop_front().unwrap();
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = pos.row as i32 + dr;
            let nc = pos.col as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                let delta_height = heights[nr][nc] as i32 - heights[pos.row][pos.col] as i32;
                if dist[nr][nc] == -1 && delta_height <= 1 {
                    next.push_back(Pos { row: nr, col: nc });
                    dist[nr][nc] = dist[pos.row][pos.col] + 1;
                }
            }
        }
    }

    println!("Min steps to reach goal: {}", dist[end.row][end.col]);

    // Part 2, copy paste with changes
    let mut dist: Vec<Vec<i32>> = vec![vec![-1; cols]; rows];
    dist[end.row][end.col] = 0;
    let mut next = VecDeque::from([end]);
    let mut min_dist = -1;
    while min_dist == -1 {
        let pos = next.pop_front().unwrap();
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = pos.row as i32 + dr;
            let nc = pos.col as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                let delta_height = heights[nr][nc] as i32 - heights[pos.row][pos.col] as i32;
                if dist[nr][nc] == -1 && delta_height >= -1 {
                    dist[nr][nc] = dist[pos.row][pos.col] + 1;
                    if heights[nr][nc] == 0 {
                        min_dist = dist[nr][nc];
                    }
                    next.push_back(Pos { row: nr, col: nc });
                }
            }
        }
    }

    println!("Min steps from any a to end: {}", min_dist);
}