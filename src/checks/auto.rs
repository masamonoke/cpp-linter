use colored::Colorize;
use regex::Regex;

pub async fn find_auto_hint_hide(lines: &Vec<String>) -> Vec<String> {
    let regex = Regex::new(r"auto\s*(\w+)\s*=\s*(new\s*(\w+))|(.*<(\w+)>)").unwrap();
    let mut ret = vec![];
    for (line_num, line) in lines.into_iter().enumerate() {
        if line.contains("auto") && regex.find(&line).is_none() {
            ret.push(format!("auto hid type at line {}: {}", line_num, line.trim_start().bold()));
        }
    }

    return ret;
}

