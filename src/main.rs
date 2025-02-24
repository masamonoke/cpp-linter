use crate::check::check;
use std::{env, fs::read_to_string};
use anyhow::Result;
use mpe_linter::check;

fn read_lines(filename: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let file = read_to_string(filename)?;
    for line in file.lines() {
        result.push(line.to_string());
    }

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let filename = &args[1];
            let lines = read_lines(&filename);
            match lines {
                Ok(lines) => {
                    check(lines);
                },
                Err(e) => println!("{}", e)
            }
        },
        _ => println!("usage:\nmpe-linter <path/to/cpp/file>")
    }
}
