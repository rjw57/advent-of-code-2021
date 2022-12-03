use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input.txt").expect("could not open input");
    let lines = io::BufReader::new(file).lines();

    let mut hpos_1 = 0;
    let mut depth_1 = 0;
    let mut hpos_2 = 0;
    let mut depth_2 = 0;
    let mut aim = 0;

    for line in lines {
        let ln_str = line.expect("could not read line");
        let (cmd, arg) = ln_str.split_once(" ").expect("could not parse line");
        let amount: i32 = arg.parse().expect("could not parse argument");
        match cmd {
            "forward" => {
                hpos_1 += amount;
                hpos_2 += amount;
                depth_2 += aim * amount;
            }
            "up" => {
                depth_1 -= amount;
                aim -= amount;
            }
            "down" => {
                depth_1 += amount;
                aim += amount;
            }
            &_ => {
                panic!("unknown command");
            }
        }
    }

    println!(
        "pt1, hpos: {}, depth: {}, product: {}",
        hpos_1,
        depth_1,
        hpos_1 * depth_1
    );

    println!(
        "pt2, hpos: {}, depth: {}, product: {}",
        hpos_2,
        depth_2,
        hpos_2 * depth_2
    );
}
