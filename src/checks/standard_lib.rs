use regex::Regex;
use colored::Colorize;

pub fn find_std(lines: &Vec<String>, exceptions: Vec<String>) -> Vec<String> {
    let regex = Regex::new(r"std::(\w+)").unwrap();

    let mut ret = vec![];
    lines.iter().enumerate().for_each(|(line, s)| {
        regex.captures_iter(s).for_each(|cap| {
            let after_std = &cap[1];
            if !exceptions.contains(&after_std.to_string()) {
                let message = format!("Found use of {} at line {}", format!("std::{}", after_std).red().bold(), line + 1);
                ret.push(message);
            }
        })
    });

    return ret;
}
