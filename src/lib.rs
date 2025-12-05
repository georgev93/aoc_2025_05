use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use std::thread;

mod file_parser;
use crate::file_parser::{FileParser, FileParserTrait};

mod food;
use crate::food::IDRange;

mod food_finder;
use crate::food_finder::FoodFinder;

pub fn solve(input_file: &str) -> (u64, u64) {
    let mut fresh_food_list = FoodFinder::new();

    let input_lines = FileParser::new(input_file).parse_lines();

    let mut parts = input_lines.as_slice().split(|line| line.is_empty());

    let fresh_food_ranges_iter = parts.next().unwrap();
    let needed_food_iter = parts.next().unwrap();

    println!("Recording ranges");
    for line in fresh_food_ranges_iter {
        fresh_food_list.insert_id_range(line);
    }
    println!("Ranges recorded!");
    fresh_food_list.sort_and_merge();
    println!("Ranges sorted!");

    let mut food_hits = 0u64;
    for food_id in needed_food_iter {
        if fresh_food_list.is_fresh(&food_id.parse::<u64>().unwrap()) {
            food_hits += 1;
        }
    }
    // let battery_banks = input_lines.iter().map(|s| BatteryBank::new(s));

    // let result1 = Arc::new(AtomicU64::new(0));
    // let result2 = Arc::new(AtomicU64::new(0));

    // let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(battery_banks.len());
    // for battery_bank in battery_banks {
    //     let result1_clone = Arc::clone(&result1);
    //     let result2_clone = Arc::clone(&result2);
    //     let handle = thread::spawn(move || {
    //         result1_clone.fetch_add(battery_bank.get_high_joltage(2), Ordering::SeqCst);
    //         result2_clone.fetch_add(battery_bank.get_high_joltage(12), Ordering::SeqCst);
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().expect("Thread panicked!");
    // }
    //
    // (
    //     result1.load(Ordering::Relaxed),
    //     result2.load(Ordering::Relaxed),
    // )
    (food_hits, fresh_food_list.number_of_fresh_ids())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (part_1, part_2) = solve("data/example.txt");
        assert_eq!(part_1, 3);
        assert_eq!(part_2, 14);
    }

    #[test]
    fn actual() {
        let (part_1, part_2) = solve("data/input.txt");
        assert_eq!(part_1, 885);
        assert_eq!(part_2, 0);
    }
}
