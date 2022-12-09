use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;

// Change to 2 for part 1
const KNOT_COUNT: usize = 10;

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
        lr: i64,
        ud: i64,
    }

    let mut knots = [Pos { lr: 0, ud: 0 }; KNOT_COUNT];
    let mut visited = HashSet::from([knots[KNOT_COUNT - 1]]);
    for line in lines {
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let distance: i64 = split.next().unwrap().parse::<i64>().unwrap();
        // Move first knot
        for _ in 0..distance {
            match direction {
                "R" => knots[0].lr += 1,
                "L" => knots[0].lr -= 1,
                "D" => knots[0].ud += 1,
                "U" => knots[0].ud -= 1,
                _ => (),
            }

            // Follow tail
            for k in 1..KNOT_COUNT {
                let delta_lr = knots[k - 1].lr - knots[k].lr;
                let delta_ud = knots[k - 1].ud - knots[k].ud;
                if delta_lr.abs() > 1 || delta_ud.abs() > 1 {
                    // Move knot by 1
                    if delta_lr != 0 {
                        knots[k].lr += delta_lr / delta_lr.abs();
                    }
                    if delta_ud != 0 {
                        knots[k].ud += delta_ud / delta_ud.abs();
                    }
                }
            }
            visited.insert(knots[KNOT_COUNT - 1]);
        }
    }

    println!("The tail visited this many unique spaces: {}", visited.len());
}