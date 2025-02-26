use std::fs::read_to_string;
use serde::Deserialize;
use strum::{EnumIter, IntoEnumIterator};

use crate::checks::{auto::find_auto_hint_hide, namespace::find_anonymous_namespace, pointer::find_non_prefixed_pointer, standard_lib::find_std};

#[derive(Deserialize)]
struct CheckOptions {
    anonymous_namespace: bool,
    pointer_prefix: String,
    std_lib_enabled: bool,
    std_lib: Vec<String>,
    auto_hint_hide: bool
}

#[derive(Deserialize)]
struct Config {
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

    fn checks(self) -> Vec<Check> {
        let mut checks = vec![];

        if self.check_options.anonymous_namespace {
            checks.push(Check::AnonymousNamespace);
        }

        if !self.check_options.pointer_prefix.is_empty() {
            checks.push(Check::PointerPrefix { prefix: self.check_options.pointer_prefix })
        }

        if self.check_options.std_lib_enabled && !self.check_options.std_lib.is_empty() {
            checks.push(Check::StandardLibrary { exceptions: self.check_options.std_lib })
        }

        if self.check_options.auto_hint_hide {
            checks.push(Check::AutoHintHide)
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
    AnonymousNamespace,
    PointerPrefix { prefix: String },
    StandardLibrary { exceptions: Vec<String> },
    AutoHintHide
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

    let mut line_messages = vec![];
    checks.into_iter().for_each(|check| {
        match check {
            Check::AnonymousNamespace => line_messages.append(&mut find_anonymous_namespace(&lines)),
            Check::PointerPrefix { prefix } => line_messages.append(&mut find_non_prefixed_pointer(&lines, &prefix)),
            Check::StandardLibrary { exceptions } => line_messages.append(&mut find_std(&lines, exceptions)),
            Check::AutoHintHide => line_messages.append(&mut find_auto_hint_hide(&lines))
        }
    });

    line_messages.into_iter().for_each(|message| println!("{}", message));
}
