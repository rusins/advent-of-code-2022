use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;

const QUERY_ROW: i64 = 2000000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|r| r.unwrap()).collect();

    // Logic
    struct Interval {
        start: i64,
        end: i64, // inclusive
    }
    let mut intervals = Vec::<Interval>::new();
    let mut beacons_in_row = HashSet::new();

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
            let interval_start = sensor_x - delta_x;
            let interval_end = sensor_x + delta_x;
            intervals.push(Interval { start: interval_start, end: interval_end });
        }
        if beacon_y == QUERY_ROW {
            beacons_in_row.insert(beacon_x);
        }
    }

    intervals.sort_by(|a, b| a.start.cmp(&b.start));

    let mut occupied = 0;
    let mut interval_end = intervals[0].end;
    let mut i: usize = 1;
    while i < intervals.len() {
        if interval_end < intervals[i].start {
            occupied += intervals[i - 1].end - intervals[i - 1].start + 1;
        } else {
            occupied += intervals[i].start - intervals[i - 1].start;
        }
        if intervals[i].end > interval_end {
            interval_end = intervals[i].end
        }
        i += 1;
    }
    occupied += intervals[i - 1].end - intervals[i - 1].start + 1;
    occupied -= beacons_in_row.len() as i64;

    println!("There are {} positions that cannot contain a beacon in row {}", occupied, QUERY_ROW);
}
