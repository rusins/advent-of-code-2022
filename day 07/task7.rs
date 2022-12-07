use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

const PART1_SIZE_THRESHOLD: i64 = 100000;
const TOTAL_DISK_SIZE: i64 = 70000000;
const SIZE_REQUIRED: i64 = 30000000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = match args.len() {
        2 => &args[1],
        _ => panic!("Missing file name argument!"),
    };
    let file = File::open(&Path::new(file_name)).unwrap();
    let lines: Vec<String> = BufReader::new(&file).lines().map(|r| r.unwrap()).collect();

    // Logic
    enum FsObject {
        Dir { children: Vec<usize>, parent: usize },
        Fil { size: i64 },
    }

    let mut nodes: Vec<FsObject> = vec![FsObject::Dir { children: Vec::new(), parent: 0 }];
    let mut names: Vec<String> = vec![String::from("/")];

    // Returns index of newly created directory
    fn create_dir(nodes: &mut Vec<FsObject>, names: &mut Vec<String>, current_dir: usize, dir_name: String) -> usize {
        println!("Creating dir {} in {}", dir_name, names[current_dir]);
        let dir = FsObject::Dir { children: Vec::new(), parent: current_dir };
        let dir_pos = nodes.len();
        nodes.push(dir);
        names.push(dir_name);
        if let FsObject::Dir { parent: _, children } = &mut nodes[current_dir] {
            children.push(dir_pos);
        }
        return dir_pos;
    }

    fn create_file(nodes: &mut Vec<FsObject>, names: &mut Vec<String>, current_dir: usize, file_name: String, file_size: i64) {
        println!("Creating file {} in {}", file_name, names[current_dir]);
        let file = FsObject::Fil { size: file_size };
        let file_pos = nodes.len();
        nodes.push(file);
        names.push(file_name);
        if let FsObject::Dir { parent: _, children } = &mut nodes[current_dir] {
            children.push(file_pos);
        }
    }

    fn size(nodes: &Vec<FsObject>, node: usize) -> i64 {
        match &nodes[node] {
            FsObject::Fil { size } => *size,
            FsObject::Dir { children, .. } => {
                let mut total: i64 = 0;
                for i in children {
                    total += size(&nodes, *i);
                }
                total
            }
        }
    }

    // Parse input
    let mut current_dir: usize = 0;
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        match split[0] {
            "$" => match split[1] {
                "cd" => match split[2] {
                    "/" => current_dir = 0,
                    ".." => if let FsObject::Dir { children: _, parent } = &nodes[current_dir] {
                        current_dir = *parent;
                    },
                    dir_name => if let FsObject::Dir { children, parent: _ } = &nodes[current_dir] {
                        // Directory to move to
                        let mut child_node = 0;
                        for child in children {
                            if dir_name.eq(&names[*child]) {
                                child_node = *child;
                                break;
                            }
                        }
                        if child_node == 0 {
                            println!("Dir {} did not exist in {}, so creating it.", dir_name, names[current_dir]);
                            child_node = create_dir(&mut nodes, &mut names, current_dir, String::from(dir_name));
                        }
                        current_dir = child_node;
                    }
                },
                "ls" => (),
                _ => panic!("Unknown command encountered"),
            },
            "dir" => {
                let dir_name = split[1];
                create_dir(&mut nodes, &mut names, current_dir, String::from(dir_name));
            }
            file_size => {
                let file_name = split[1];
                create_file(&mut nodes, &mut names, current_dir, String::from(file_name), file_size.parse().unwrap())
            }
        }
    }

    // Iterate over directories and calculate the answer to part 1
    let mut part1_size_sum: i64 = 0;
    for i in 0..nodes.len() {
        if let FsObject::Dir { children: _, parent: _ } = &nodes[i] {
            let size = size(&nodes, i);
            if size <= PART1_SIZE_THRESHOLD {
                part1_size_sum += size;
            }
        }
    }
    println!("Part 1 answer: {}", part1_size_sum);

    // Iterate over files and calculate total space used on drive
    let mut used_space: i64 = 0;
    for i in 0..nodes.len() {
        if let FsObject::Fil { size } = &nodes[i] {
            used_space += size;
        }
    }

    let extra_space_needed = SIZE_REQUIRED - (TOTAL_DISK_SIZE - used_space);
    println!("Need {} more space for the update!", extra_space_needed);

    // Iterate over folders and pick the smallest one that's over the needed threshold
    let mut best_size = TOTAL_DISK_SIZE;
    for i in 0..nodes.len() {
        if let FsObject::Dir {parent: _, children: _} = &nodes[i] {
            let size = size(&nodes, i);
            if size >= extra_space_needed && size < best_size {
                best_size = size;
            }
        }
    }
    println!("Part 2 answer: {}", best_size);
}