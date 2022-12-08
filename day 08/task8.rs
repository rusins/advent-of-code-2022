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
    let mut heights: Vec<Vec<u32>> = vec![vec![0; cols]; rows];
    for row in 0..rows {
        let line: Vec<u32> = lines[row].chars().map(|c| c.to_digit(RADIX).unwrap() as u32).collect();
        for col in 0..cols {
            heights[row][col] = line[col];
        }
    }
    let heights = heights;

    let mut visible: Vec<Vec<u32>> = vec![vec![false; cols]; rows];
    fn check_line()
}