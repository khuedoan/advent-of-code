// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
//
// Here is an example engine schematic:
//
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
//
// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

use std::fs;
use std::io::{BufRead, BufReader, Lines};

fn has_adjacent_symbol(input: Vec<String>, x_left: usize, x_right: usize, y: usize) -> bool {
    let left_edge: usize = x_left - 1;
    let left_edge: usize = x_right + 1;
    let top_edge: usize = y - 1;
    let bottom_edge: usize = y + 1;

    false
}

fn main() {
    let input_file = fs::File::open("./input").unwrap();
    let reader = BufReader::new(input_file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut sum = 0;

    for line in reader.lines() {
        // for each line -> y
        // find number n -> xb, xe
        //
        // check if any in the following has
        //                    xb-1,y ; xe+1,y
        // or if y > 0        xb-1,y-1 -> xe+1,y-1
        // or if y < max line xb-1,y+1 -> xe+1,y+1
        // if any match then sum += n
    }

    println!("{sum}");
}


#[cfg(test)]
mod tests {
    #[test]
    fn simple_one_line() {
        let input = vec!["617*......"];

        assert_eq!(crate::has_adjacent_symbol(input, 0, 2, 0), true);
    }
}
