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
    let mut found_digit: bool = false;
    
    //one two three four five six seven eight nine 
    println!("Line {}", line);
    for (i, c) in line.to_lowercase().chars().enumerate() {
        if c.is_numeric()  {
            end = c.to_digit(10).unwrap();
            found_digit = true;
        } else if c == 'o' {
            if let Some(substring) = line.get(i..i + 3) {
                if substring == "one" {
                    end = 1;
                    found_digit = true;
                }
            }
        } else if c == 't' {
            if let Some(substring) = line.get(i..i + 3) {
                if substring == "two" {
                    end = 2;
                    found_digit = true;
                }
            } 
            if let Some(substring) = line.get(i..i + 5) {
                if substring == "three" {
                    end = 3;
                    found_digit = true;
                }
            }
        } else if c == 'f' {
            if let Some(substring) = line.get(i..i + 4) {
                if substring == "four" {
                    end = 4;
                    found_digit = true;
                } else if substring == "five" {
                    end = 5;
                    found_digit = true;
                }
            }
        } else if c == 's' {
            if let Some(substring) = line.get(i..i + 3) {
                if substring == "six" {
                    end = 6;
                    found_digit = true;
                } 
            } 
            if let Some(substring) = line.get(i..i + 5) {
                if substring == "seven" {
                    end = 7;
                    found_digit = true;
                } 
            }
        } else if c == 'e' {
            if let Some(substring) = line.get(i..i + 5) {
                if substring == "eight" {
                    end = 8;
                    found_digit = true;
                } 
            } 
        } else if c == 'n' {
            if let Some(substring) = line.get(i..i + 4) {
                if substring == "nine" {
                    end = 9;
                    found_digit = true;
                } 
            } 
        } 
        if found_digit && !start_set {
            st = end;
            start_set = true;
        }
        found_digit = false;
    }
    let calibration_value: u32 = st * 10 + end;
    println!("Calibration value for line {} is {}", line, calibration_value);
    calibration_value
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}