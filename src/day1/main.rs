use std::collections::HashMap;
use std::env;
use std::fs;

const DAY: u32 = 1;
const YEAR: u32 = 2016;


#[derive(Copy, Clone)]
enum Rotate {
    LEFT, 
    RIGHT,
}

fn read_file_string(filename: &str) -> String {
    let data_folder = env::var("ADVENT_HOME").expect("ADVENT_HOME is not set");
    let file_path = format!("{}/{}/data/day{}/{}", data_folder, YEAR, DAY, filename);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");    
    contents
}

#[derive(Copy, Clone)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn turn(dir: Direction, rotate: Rotate) -> Direction {
    let index = dir as isize;

    let next = match rotate {
        Rotate::RIGHT => {
            let n = index+1;
            if n >= 4 {
                0
            } else {
                n
            }
        },
        Rotate::LEFT => {
            let n = index-1;
            if n < 0 {
                3
            } else {
                n
            }
        }
    };

    return match next {
        0 => Direction::NORTH,
        1 => Direction::EAST,
        2 => Direction::SOUTH,
        3 => Direction::WEST,
        _ => Direction::NORTH,
    }
}

fn main() {
    let data = read_file_string("data.txt");
    let parts = data.trim().split(", ");

    let mut x:isize = 0;
    let mut y:isize = 0;
    let mut direction = Direction::NORTH;
    let mut count: usize = 0;
    let mut map: HashMap<String, isize> = HashMap::new();
    let mut first = -1;
    for part in parts {
        count += 1;
        let dir = &part[..1];
        let dist = &part[1..];
        let dir_enum = match dir {
            "L" => Rotate::LEFT,
            _ => Rotate::RIGHT
        };
        let distance: isize = dist.parse().unwrap_or_default();
        
        direction = turn(direction, dir_enum);

        for idx in 0..distance {
            match direction {
                Direction::NORTH => {y += 1;},
                Direction::SOUTH => {y -= 1;},
                Direction::EAST => {x += 1;},
                Direction::WEST => {x -= 1;},
            }
            let key = format!("{}-{}",x,y);
            if(map.contains_key(&key) && first < 0) {
                first = isize::abs(x)+isize::abs(y);
            } else {
                map.insert(key, 1);
            }
        }
        
    
    }
    println!("Part 1 = {}", isize::abs(x)+isize::abs(y));
    println!("Part 2 = {}", first);    
}
