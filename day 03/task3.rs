use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::BTreeSet;
use std::collections::BTreeMap;


fn main() {
    let args: Vec<String> =  env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines = BufReader::new(&file).lines().map(|r| r.unwrap());

    // Logic
    let priority_map = {
      let mut map = BTreeMap::new();
        for c in 'a' ..= 'z' {
            map.insert(c, (c as u32) - ('a' as u32) + 1);
        }
        for c in 'A' ..= 'Z' {
            map.insert(c, (c as u32) - ('A' as u32) + 27);
        }
        map
    };

    let mut priority_sum = 0;
    let mut badge_sum = 0;
    let mut group_bag: [BTreeSet<char>; 2] = [BTreeSet::new(), BTreeSet::new()];
    for (line_no, line) in lines.enumerate() {

        let mut first_half: BTreeSet<char> = BTreeSet::new();
        for i in 0 .. line.len() / 2 {
            let c = line.as_bytes()[i] as char;
            first_half.insert(c);
        }
        for i in line.len() / 2 .. line.len() {
            let c = line.as_bytes()[i] as char;
            if first_half.contains(&c) {
                // println!("Bag had duplicate item {}", c);
                priority_sum += priority_map[&c];
                break;
            }
        }

        // Part 2
        let group_member_number = line_no % 3;
        if group_member_number <= 1 {
            group_bag[group_member_number] = BTreeSet::new();
        }
        for i in 0 .. line.len() {
            let c = line.as_bytes()[i] as char;
            if group_member_number <= 1 {
                group_bag[group_member_number].insert(c);
            } else if group_bag[0].contains(&c) && group_bag[1].contains(&c) {
                badge_sum += priority_map[&c];
                break;
            }
        }
    }
    println!("Part 1: {}", priority_sum);
    println!("Part 2: {}", badge_sum);
}