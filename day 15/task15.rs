use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

const QUERY_ROW: i64 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|r| r.unwrap()).collect();

    // Logic
    let mut intervals = Vec::<i64>::new();
    let mut interval_end = Vec::<i64>::new();
    let mut beacons_in_row: i64 = 0;

    for line in lines {
        // 200IQ parsing with magic constants
        let parsed: Vec<&str> = line.split(|c| c == '=' || c == ',' || c == ':').collect();
        let sensor_x: i64 = parsed[1].parse().unwrap();
        let sensor_y: i64 = parsed[3].parse().unwrap();
        let beacon_x: i64 = parsed[5].parse().unwrap();
        let beacon_y: i64 = parsed[7].parse().unwrap();

        let dist = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
        let delta_y = (QUERY_ROW - sensor_y).abs();
        if delta_y <= dist {
            let delta_x = (dist - delta_y).abs();

        }
    }
}
