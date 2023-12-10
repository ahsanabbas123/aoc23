use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Solution for Day {}", 2);
    let filename: String = "./src/input.txt".to_string();
    let mut possible_games_sum: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        println!("Processing lines");
        for line in lines {
            possible_games_sum += check_game_possible(line.unwrap());
        }
    }
    println!("Total possible games: {}", possible_games_sum);
}

fn check_game_possible(line: String) -> u32 {
    // regex
    let game_re = Regex::new(r"Game (\d+):(.*)").unwrap();
    let count_re = Regex::new(r"(\d+) (\w+)").unwrap();

    if let Some(cap) = game_re.captures(line.as_str()) {
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;
        let mut min_red: u32 = 0;
        let parts: Vec<&str> = cap[2].split(';').collect();
        for part in parts {
            let mut color_counts: HashMap<String, u32> = HashMap::new();
            for count_cap in count_re.captures_iter(part) {
                let count: u32 = count_cap[1].parse().unwrap();
                let color = count_cap[2].to_string();
                *color_counts.entry(color).or_insert(0) += count;
            }
            min_green = max(color_counts.get("green").cloned().unwrap_or(0), min_green);
            min_blue = max(color_counts.get("blue").cloned().unwrap_or(0), min_blue);
            min_red = max(color_counts.get("red").cloned().unwrap_or(0), min_red);
        }
        return min_green * min_blue * min_red;
    }
    return 0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
