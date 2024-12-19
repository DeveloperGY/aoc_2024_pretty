use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_3").unwrap();
    let mul_calls = process_muls_input(&input);
    let flagged_mul_calls = process_flagged_muls_input(&input);

    let mul_total = mul_calls
        .into_iter()
        .map(|(lhs, rhs)| lhs * rhs)
        .sum::<u32>();
    let flagged_mul_total = flagged_mul_calls
        .into_iter()
        .map(|(lhs, rhs)| lhs * rhs)
        .sum::<u32>();

    println!(
        "Advent of Code 2024 - Day 3\n[Part 1] All Mul Total: {}\n[Part 2] Enabled Mul Total: {}",
        mul_total, flagged_mul_total
    );
}

fn process_muls_input(input: &str) -> Vec<(u32, u32)> {
    let should_include_first_section = input.starts_with("mul(");
    input
        // Find the starts of all the mul calls
        .split("mul(")
        // Skip the first one if it didn't start with mul
        .skip(if should_include_first_section { 0 } else { 1 })
        // Filter out the ones that don't have a closing parenthesis
        .filter(|args_str| args_str.contains(')'))
        // Keep everything before the closing parenthesis
        .map(|args_str| args_str.split_once(')').unwrap().0)
        // ensure we have two arguments and they're not too long
        .filter(|arg_str| arg_str.contains(',') && arg_str.chars().count() <= 7)
        // Split into two arguments
        .map(|arg_str| arg_str.split_once(',').unwrap())
        // Filter out arguments that are too long
        .filter(|(lhs, rhs)| lhs.chars().count() <= 3 && rhs.chars().count() <= 3)
        // Parse the arguments
        .filter_map(|(lhs, rhs)| Some((lhs.parse::<u32>().ok()?, rhs.parse::<u32>().ok()?)))
        .collect()
}

fn process_flagged_muls_input(input: &str) -> Vec<(u32, u32)> {
    input
        // Split into enabled sections
        .split("do()")
        // Remove the disabled part of each enabled section
        .map(|enabled_code| {
            enabled_code
                .split_once("don't()")
                .unwrap_or((enabled_code, ""))
                .0
        })
        // Process each enabled part of each section
        .flat_map(process_muls_input)
        .collect()
}
