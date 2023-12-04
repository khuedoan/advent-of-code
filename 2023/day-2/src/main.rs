// Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
//
// For example, the record of a few games might look like this:
//
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//
// In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.
//
// The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
//
// In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration.
// However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once.
// If you add up the IDs of the games that would have been possible, you get 8.

use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct GameSubset {
    red: i32,
    green: i32,
    blue: i32,
}

impl From<String> for GameSubset {
    fn from(input_string: String) -> Self {
        let mut subset = GameSubset {
            red: 0,
            green: 0,
            blue: 0,
        };

        for set in input_string.split(",") {
            let tokens: Vec<_> = set.trim().split(" ").collect();

            match tokens[1] {
                "red" => subset.red += tokens[0].parse::<i32>().unwrap(),
                "green" => subset.green += tokens[0].parse::<i32>().unwrap(),
                "blue" => subset.blue += tokens[0].parse::<i32>().unwrap(),
                _ => {},
            };
        }

        subset
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    subsets: Vec<GameSubset>,
}

impl Game {
    fn is_valid(&self) -> bool {
        !self.subsets.iter().any(|set| set.red > 12 || set.green > 13 || set.blue > 14)
    }
}

impl From<String> for Game {
    fn from(input_string: String) -> Self {
        let sections = input_string.split(":").collect::<Vec<&str>>();

        Game {
            id: sections[0]
                .trim()
                .split(" ")
                .collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap(),
            subsets: sections[1]
                .trim()
                .split(";")
                .map(|s| GameSubset::from(s.to_string()))
                .collect(),
        }
    }
}

fn main() {
    let input_file = fs::File::open("./input").unwrap();
    let reader = BufReader::new(input_file);
    let mut id_sum = 0;

    for line in reader.lines() {
        let game = Game::from(line.unwrap());

        if game.is_valid() {
            id_sum += game.id;
        }
    }

    println!("{id_sum}");
}


#[cfg(test)]
mod tests {
    #[test]
    fn subset_from_string() {
        let input = "1 red, 2 green, 6 blue";
        let subset = crate::GameSubset::from(input.to_string());
        assert_eq!(subset.red, 1);
        assert_eq!(subset.green, 2);
        assert_eq!(subset.blue, 6);
    }

    #[test]
    fn game_from_string() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = crate::Game::from(input.to_string());
        assert_eq!(game.id, 1);

        assert_eq!(game.subsets[0].red, 4);
        assert_eq!(game.subsets[0].green, 0);
        assert_eq!(game.subsets[0].blue, 3);

        assert_eq!(game.subsets[1].red, 1);
        assert_eq!(game.subsets[1].green, 2);
        assert_eq!(game.subsets[1].blue, 6);

        assert_eq!(game.subsets[2].red, 0);
        assert_eq!(game.subsets[2].green, 2);
        assert_eq!(game.subsets[2].blue, 0);
    }
}
