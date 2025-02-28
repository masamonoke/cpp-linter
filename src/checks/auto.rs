use colored::Colorize;
use regex::Regex;

pub async fn find_auto_hint_hide(lines: &Vec<String>) -> Vec<String> {
    let regex = Regex::new(r"auto\s*\*?(\w+)\s*=\s*(new\s*(\w+))|(.*<(.*)>)").unwrap();
    let mut ret = vec![];
    for (line_num, line) in lines.into_iter().enumerate() {
        if line.contains("auto") && regex.find(&line).is_none() {
            ret.push(format!("auto hid type at line {}: {}", line_num, line.trim_start().bold()));
        }
    }

    return ret;
}


#[cfg(test)]
mod tests {
    use crate::checks::pointer::find_non_prefixed_pointer;

    #[tokio::test]
    async fn test_auto_type_erasure() {
        let test_str = std::fs::read_to_string("tests/auto.cpp").unwrap();
        for line in test_str.lines().filter(|s| *s != "") {
            let v = line.split("// ").collect::<Vec<&str>>();
            let test = v[0].to_string();
            let res: usize = v[1].parse().unwrap();
            assert_eq!(res, find_non_prefixed_pointer(&vec![test.clone()], "p".to_owned()).await.len(), "{}", test);
        }
    }
}
