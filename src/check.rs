use std::fs::read_to_string;
use serde::Deserialize;
use strum::{EnumIter, IntoEnumIterator};

use crate::anonymous_namespace::find_anonymous_namespace;

#[derive(Deserialize)]
struct CheckOptions {
    anonymous_namespace: bool
}

#[derive(Deserialize)]
pub struct Config {
    check_options: CheckOptions,
}

impl Config {
    fn new() -> Option<Self> {
        let toml_content = read_to_string("linter.toml");

        match  toml_content {
            Ok(toml_content) => {
                let config: Result<Config, toml::de::Error> = toml::from_str(&toml_content);
                match config {
                    Ok(config) => {
                        return Some(config);
                    },
                    Err(e) => println!("{}", e)
                }
            }
            Err(_) => {}
        }

        return None;
    }

    fn checks(&self) -> Vec<Check> {
        let mut checks = vec![];

        if self.check_options.anonymous_namespace {
            checks.push(Check::AnonymousNamespace);
        }

        return checks;
    }

    fn all_enabled() -> Vec<Check> {
        let mut checks = vec![];
        for check in Check::iter() {
            checks.push(check);
        }

        checks
    }
}

#[derive(EnumIter)]
pub enum Check {
    AnonymousNamespace
}

pub fn check(lines: Vec<String>) {
    let config = Config::new();
    let checks;

    match config {
        Some(config) => {
            checks = config.checks();
        },
        None => checks = Config::all_enabled()
    }

    checks.into_iter().for_each(|check| {
        match check {
            Check::AnonymousNamespace => find_anonymous_namespace(&lines)
        }
    })
}
