use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input.txt").expect("could not open input");
    let lines = io::BufReader::new(file).lines();

    let mut bit_counts: Vec<usize> = vec![];
    let mut values: Vec<String> = Vec::new();
    for l_result in lines {
        let line = l_result.expect("could not read line");
        if bit_counts.len() < line.len() {
            bit_counts.resize(line.len(), 0);
        }
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                bit_counts[i] += 1;
            }
        }
        values.push(line);
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for (i, c) in bit_counts.iter().rev().enumerate() {
        assert!(c != &(values.len() >> 1));
        if c > &(values.len() >> 1) {
            // 1 was most common bit
            gamma += 1 << i;
        } else {
            // 1 was least common bit
            epsilon += 1 << i;
        }
    }

    println!(
        "gamma: {}, epislon: {}, product: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let mut remaining_o2_values: Vec<&String> = values.iter().collect();
    let mut remaining_co2_values: Vec<&String> = values.iter().collect();
    for i in 0..bit_counts.len() {
        let o2_c: usize = remaining_o2_values.iter().filter(|v| v[i..(i+1)] == *"1").map(|_| 1).sum();
        let co2_c: usize = remaining_co2_values.iter().filter(|v| v[i..(i+1)] == *"1").map(|_| 1).sum();

        let o2_bit = if o2_c >= (remaining_o2_values.len() - o2_c) { "1" } else { "0" };
        let co2_bit = if co2_c >= (remaining_co2_values.len() - co2_c) { "0" } else { "1" };

        // filter O2 values
        if remaining_o2_values.len() > 1 {
            remaining_o2_values = remaining_o2_values
                .iter()
                .filter(|v| v[i..(i + 1)] == *o2_bit)
                .map(|v| v.to_owned())
                .collect();
        }

        // filter CO2 values
        if remaining_co2_values.len() > 1 {
            remaining_co2_values = remaining_co2_values
                .iter()
                .filter(|v| v[i..(i + 1)] == *co2_bit)
                .map(|v| v.to_owned())
                .collect();
        }
    }

    assert!(remaining_o2_values.len() == 1);
    assert!(remaining_co2_values.len() == 1);
    let o2 = usize::from_str_radix(remaining_o2_values[0], 2).expect("could not parse binary");
    let co2 = usize::from_str_radix(remaining_co2_values[0], 2).expect("could not parse binary");
    println!("O2: {}, CO2: {}, product: {}", o2, co2, o2 * co2);
}
