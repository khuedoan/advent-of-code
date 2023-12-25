use std::fs;
use std::io::{BufRead, BufReader};

fn first_digit(val: std::str::Chars<'_>) -> Option<u32> {
    for ch in val {
        if let Some(res) = ch.to_digit(10) {
            return Some(res)
        }
    }

    None
}

fn main() {
    // Each line originally contained a specific calibration value.
    // On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number
    //
    // For example:
    //
    // 1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet
    //
    // In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
    // Consider your entire calibration document. What is the sum of all of the calibration values?
    let input_file = fs::File::open("./input/day1").unwrap();
    let reader = BufReader::new(input_file);
    let mut sum = 0;

    for line in reader.lines() {
        let values = line.unwrap();
        let first = first_digit(values.chars()).unwrap();
        let last = first_digit(values.chars().rev().collect::<String>().chars()).unwrap();
        let calibration_value = first * 10 + last;
        sum += calibration_value;
    }

    println!("{sum}");
}
