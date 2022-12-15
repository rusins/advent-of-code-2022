use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;

const SEARCH_BOUND: i64 = 4000000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|r| r.unwrap()).collect();

    // Logic
    fn find_gap_in_row(data: &Vec<(i64, i64, i64, i64)>, query_row: i64) -> Option<i64> {
        struct Interval {
            start: i64,
            end: i64, // inclusive
        }
        let mut intervals = Vec::<Interval>::new();
        let mut beacons_in_row = HashSet::new();

        for d in data {
            let (sensor_x, sensor_y, beacon_x, beacon_y) = d;
            let dist = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
            let delta_y = (query_row - sensor_y).abs();
            if delta_y <= dist {
                let delta_x = (dist - delta_y).abs();
                let interval_start = sensor_x - delta_x;
                let interval_end = sensor_x + delta_x;
                intervals.push(Interval { start: interval_start, end: interval_end });
            }
            if *beacon_y == query_row {
                beacons_in_row.insert(beacon_x);
            }
        }

        intervals.sort_by(|a, b| a.start.cmp(&b.start));

        let mut i = 1;
        let mut interval_end = intervals[0].end;
        while i < intervals.len() {
            if intervals[i].start > interval_end + 1 {
                return Some(intervals[i].start - 1);
            }
            if intervals[i].end > interval_end {
                interval_end = intervals[i].end;
            }
            i += 1;
        }
        None
    }

    let mut data = Vec::<(i64, i64, i64, i64)>::new();

    for line in lines {
        // 200IQ parsing with magic constants
        let parsed: Vec<&str> = line.split(|c| c == '=' || c == ',' || c == ':').collect();
        let sensor_x: i64 = parsed[1].parse().unwrap();
        let sensor_y: i64 = parsed[3].parse().unwrap();
        let beacon_x: i64 = parsed[5].parse().unwrap();
        let beacon_y: i64 = parsed[7].parse().unwrap();

        data.push((sensor_x, sensor_y, beacon_x, beacon_y));
    }


    for i in 1..=SEARCH_BOUND {
        if i % (SEARCH_BOUND / 100) == 0 {
            println!("{}%", i / (SEARCH_BOUND / 100));
        }
        match find_gap_in_row(&data, i) {
            Some(gap) => {
                println!("Found gap x: {}, y: {}", gap, i);
                println!("Beacon tuning frequency: {}", gap * SEARCH_BOUND + i);
                return
            },
            _ => ()
        }
    }
}
