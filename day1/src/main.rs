use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Solution for Day {}", 1);
    let filename: String = "./src/input.txt".to_string();
    let mut total_calibration_value: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        println!("Processing lines");
        for line in lines {
            total_calibration_value += calculate_calibration(line.unwrap());
        }
    }
    println!("Total calibration value: {}", total_calibration_value);
}

fn calculate_calibration(line: String) -> u32 {
    let mut st: u32 = 0;
    let mut end: u32 = 0;
    let mut start_set: bool = false;
    for c in line.chars() {
        if c.is_numeric() {
            end = c.to_digit(10).unwrap();
            if !start_set {
                st = end;
                start_set = true;
            }
        }
    }
    let calibration_value: u32 = st * 10 + end;
    println!(
        "Calibration value for line {} is {}",
        line, calibration_value
    );
    calibration_value
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
