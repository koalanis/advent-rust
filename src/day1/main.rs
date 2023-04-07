use std::env;
use std::fs;


const DAY: u32 = 1;
const YEAR: u32 = 2016;
 

    fn read_file_string(filename: &str) -> String {
        println!("{}", filename);
        let data_folder = env::var("ADVENT_HOME").expect("ADVENT_HOME is not set");
        let file_path = format!("{}/{}/data/day{}/{}", data_folder, YEAR, DAY, filename);
        println!("file_path={}",file_path);
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        println!("{}", contents);
        
        contents
    }

enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn main() {
    println!("Hello, world!");
    let data = read_file_string("data.txt");
    let parts = data.trim().split(", ");

    let mut count_x:i32 = 0;
    let mut count_y:i32 = 0;
    let mut direction = Direction::NORTH;

    for part in parts {
        let dir = &part[..1];
        let dist = &part[1..2];
        println!("{} {}", part, inst);
        let blocks = inst.to_string().parse::<u32>().expect("");
        

    }
    println!("{}", count);
}
