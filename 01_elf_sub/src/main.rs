use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn part_one(measurements_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut total = 0;
    let mut last = -1;

    read_file(measurements_file)
        .unwrap()
        .lines()
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .for_each(|x| {
            if last < x && last > 0 {
                total += 1;
            }
            last = x;
        });

    println!("Part 1: {}", total);

    Ok(())
}

const WINDOW_SIZE: usize = 3;
fn part_two(measurements_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut total: i32 = 0;
    let mut last: i32 = -1;

    // read measurements
    read_file(measurements_file)
        .unwrap()
        .as_str()
        // line by line
        .lines()
        // translating from Strings to i32
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect_vec()
        // splitting into sliding windows
        .windows(WINDOW_SIZE)
        .collect_vec()
        .iter()
        .for_each(|x| {
            // Get the sum of measurements in the window
            let mut sum: i32 = 0;
            for i in 0..WINDOW_SIZE {
                sum += x[i]
            }

            // Check if window shows an aggregate increase
            if last < sum && last > 0 {
                total += 1;
            }
            last = sum;
        });

    println!("Part 2: {}", total);

    Ok(())
}

fn read_file(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut data = String::new();
    File::open(filename)?.read_to_string(&mut data)?;
    Ok(data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = format!("{}/resources/measurements.txt", env!("CARGO_MANIFEST_DIR"));
    part_one(&filepath)?;
    part_two(&filepath)?;

    Ok(())
}
