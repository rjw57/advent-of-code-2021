use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("./input.txt").expect("could not open input");
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|v| v.expect("failed to read"))
        .collect();

    count_fish(&lines[0], 80);
    count_fish(&lines[0], 256);
}

fn count_fish(ln: &str, days: u32) {
    let initial: Vec<usize> = ln.split(",").map(|v| v.parse().expect("bad num")).collect();

    // Counts of fish with specified timers.
    let mut fish_timers: Vec<i64> = vec![0; 9];
    initial.iter().for_each(|&v| fish_timers[v] += 1);

    for _ in 1..=days {
        let n_breed = fish_timers[0];
        for i in 1..(fish_timers.len()) { fish_timers[i-1] = fish_timers[i]; }
        fish_timers[6] += n_breed;
        fish_timers[8] = n_breed;
    }
    println!("Fish count: {}", fish_timers.iter().sum::<i64>());
}
