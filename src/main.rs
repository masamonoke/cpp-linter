use std::io::BufRead;
use std::io;
use linter::check::check;
use std::{env, fs::read_to_string};
use anyhow::Result;

fn read_lines(filename: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let file = read_to_string(filename)?;
    for line in file.lines() {
        result.push(line.to_string());
    }

    Ok(result)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            let stdin = io::stdin();
            let mut lines = vec![];
            for line in stdin.lock().lines() {
                match line {
                    Ok(line) => lines.push(line),
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                }
            }
            check(lines).await
        },
        2 => {
            let filename = &args[1];
            let lines = read_lines(&filename);
            match lines {
                Ok(lines) => {
                    check(lines).await
                },
                Err(e) => println!("{}", e)
            }
        },
        _ => println!("usage:\nlinter <path/to/cpp/file>")
    }
}
