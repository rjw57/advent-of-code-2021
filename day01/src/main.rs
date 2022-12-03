use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input.txt").expect("could not open input");
    let lines = io::BufReader::new(file).lines();

    let (mut prev, mut window_prev): (i32, i32) = (0, 0);
    let (mut n_increases, mut n_window_increases): (i32, i32) = (0, 0);
    let mut window: Vec<i32> = vec![0,0,0];
    for (i, ln) in lines.enumerate() {
        let v: i32 = ln
            .expect("could not read line")
            .parse()
            .expect("failed to parse");
        window[i % 3] = v;
        let window_sum = window.iter().sum();
        if (i > 0) && (v > prev) {
            n_increases += 1;
        }
        if (i >= 3) && (window_sum > window_prev) {
            n_window_increases += 1;
        }
        (prev, window_prev) = (v, window_sum);
    }

    println!("Number of increases: {}.", n_increases);
    println!("Number of window increases: {}.", n_window_increases);
}
