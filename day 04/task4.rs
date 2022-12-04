use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines = BufReader::new(&file).lines().map(|r| r.unwrap());

    // Logic
    struct Assignment {
        start: i64,
        end: i64,
    }
    impl Assignment {
        fn parse(text: &str) -> Self {
            let parts: Vec<&str> = text.split("-").collect();
            Assignment {
                start: parts[0].parse().unwrap(),
                end: parts[1].parse().unwrap(),
            }
        }

        fn contains(&self, other: &Assignment) -> bool {
            self.start <= other.start && self.end >= other.end
        }

        fn overlaps(&self, other: &Assignment) -> bool {
            self.start <= other.end && self.end >= other.start
        }
    }

    let mut containment_count = 0;
    let mut overlap_count = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(",").collect();
        let a1 = Assignment::parse(parts[0]);
        let a2 = Assignment::parse(parts[1]);
        if a1.contains(&a2) || a2.contains(&a1) {
            containment_count += 1;
        }
        if a1.overlaps(&a2) {
            overlap_count += 1;
        }
    }

    println!("Containment count: {}", containment_count);
    println!("Overlap count: {}", overlap_count);
}