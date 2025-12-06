use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use std::thread;

pub mod file_parser;
use crate::file_parser::FileParser;

mod food;
use crate::food::IDRange;

mod food_finder;
use crate::food_finder::FoodFinder;

pub fn solve(input_file: &str) -> (u64, u64) {
    let mut fresh_food_list = FoodFinder::new();

    let input_lines: Vec<&str> = input_file.lines().collect();

    let mut parts = input_lines.as_slice().split(|line| line.is_empty());

    let fresh_food_ranges_iter = parts.next().unwrap();
    let needed_food_iter = parts.next().unwrap();

    for line in fresh_food_ranges_iter {
        fresh_food_list.insert_id_range(line);
    }
    fresh_food_list.sort_and_merge();

    let mut food_hits = 0u64;
    for food_id in needed_food_iter {
        if fresh_food_list.is_fresh(&food_id.parse::<u64>().unwrap()) {
            food_hits += 1;
        }
    }
    (food_hits, fresh_food_list.number_of_fresh_ids())
}

pub fn solve_pt1(input_file: &str) -> u64 {
    let mut fresh_food_list = FoodFinder::new();

    let input_lines: Vec<&str> = input_file.lines().collect();

    let mut parts = input_lines.as_slice().split(|line| line.is_empty());

    let fresh_food_ranges_iter = parts.next().unwrap();
    let needed_food_iter = parts.next().unwrap();

    for line in fresh_food_ranges_iter {
        fresh_food_list.insert_id_range(line);
    }

    let mut food_hits = 0u64;
    for food_id in needed_food_iter {
        if fresh_food_list.is_fresh(&food_id.parse::<u64>().unwrap()) {
            food_hits += 1;
        }
    }
    food_hits
}

pub fn solve_pt2(input_file: &str) -> u64 {
    let mut fresh_food_list = FoodFinder::new();

    let input_lines: Vec<&str> = input_file.lines().collect();

    let mut parts = input_lines.as_slice().split(|line| line.is_empty());

    let fresh_food_ranges_iter = parts.next().unwrap();
    let needed_food_iter = parts.next().unwrap();

    for line in fresh_food_ranges_iter {
        fresh_food_list.insert_id_range(line);
    }

    fresh_food_list.sort_and_merge();

    fresh_food_list.number_of_fresh_ids()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let file = FileParser::new("data/example.txt");
        let (part_1, part_2) = solve(file.get_str());
        assert_eq!(part_1, 3);
        assert_eq!(part_2, 14);
    }

    #[test]
    fn example_pts() {
        let my_file = FileParser::new("data/example.txt");
        assert_eq!(solve_pt1(my_file.get_str()), 3);
        assert_eq!(solve_pt2(my_file.get_str()), 14);
    }

    #[test]
    fn actual() {
        let file = FileParser::new("data/input.txt");
        let (part_1, part_2) = solve(file.get_str());
        assert_eq!(part_1, 885);
        assert_eq!(part_2, 348115621205535);
    }

    #[test]
    fn actual_pts() {
        let my_file = FileParser::new("data/input.txt");
        assert_eq!(solve_pt1(my_file.get_str()), 885);
        assert_eq!(solve_pt2(my_file.get_str()), 348115621205535);
    }
}
