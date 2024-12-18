#![feature(array_windows)]

use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_2").unwrap();
    let reports = process_input(&input);
    
    let safe_report_count = reports.iter().filter(|report| is_report_safe(report)).count();
    let tolerable_report_count = reports.iter().filter(|report| is_report_tolerable(report)).count();
    println!("Advent of Code 2024 - Day 2\n[Part 1] Safe Reports: {}\n[Part 2] Tolerable Reports: {}", safe_report_count, tolerable_report_count);
}

fn process_input(input: &str) -> Vec<Vec<u32>> {
    // A report is a list of reactor levels
    // Each line is a space-separated list of reactor levels
    // Thus this returns a list of reports
    input.lines().map(|line| {
        line.split_whitespace().map(|word| word.parse::<u32>().unwrap()).collect()
    }).collect()
}

fn is_report_safe(reactor_levels: &[u32]) -> bool {
    if reactor_levels.len() <= 1 {
        // A report of at most one reactor level will always be safe
        return true;
    } else if reactor_levels[0] == reactor_levels[1] {
        // Reactor levels must not remain stagnant!
        return false;
    }
    
    let is_increasing = reactor_levels[1] > reactor_levels[0];
    // Test if every consecutive pair of reactor levels is safe (magnitude changes by 1-3, and is consistently increasing or decreasing)
    reactor_levels.array_windows().all(|&[lhs, rhs]| (1..4).contains(&lhs.abs_diff(rhs)) && if is_increasing {lhs < rhs} else {lhs > rhs})
}

fn is_report_tolerable(reactor_levels: &[u32]) -> bool {
    if is_report_safe(reactor_levels) {
        // Report is already safe, no need to remove any reactor levels
        return true;
    }

    (0..reactor_levels.len()).any(|i| {
        // Remove the i-th reactor level from the report
        let potential_tolerable_report: Vec<_> = reactor_levels[0..i].iter().chain(&reactor_levels[i + 1..]).copied().collect();
        // Test if the resulting report is safe
        is_report_safe(&potential_tolerable_report)
    })
}