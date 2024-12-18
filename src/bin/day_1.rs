use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_1").unwrap();
    let (mut left_list, mut right_list) = process_input(&input);
    left_list.sort();
    right_list.sort();
    
    let total_distance: u32 = left_list.iter().zip(&right_list).map(|(lhs, rhs)| lhs.abs_diff(*rhs)).sum();
    let similarity_score: u32 = left_list.iter().map(|lhs| *lhs * right_list.iter().filter(|rhs| **rhs == *lhs).count() as u32).sum();
    
    println!("Advent of Code 2024 - Day 1\n[Part 1] Total Distance: {}\n[Part 2] Similarity Score: {}", total_distance, similarity_score);
}

fn process_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines().map(|l| {
        let mut split = l.split_whitespace();
        (split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap().parse::<u32>().unwrap())
    }).unzip()
}