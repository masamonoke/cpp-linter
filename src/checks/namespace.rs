use colored::Colorize;
use regex::Regex;

pub async fn find_anonymous_namespace(lines: &Vec<String>) -> Vec<String> {
    let pattern = r"namespace\s*\{|^namespace\s*$";
    let regex = Regex::new(pattern).unwrap();
    let mut ret = vec![];
    lines.iter().enumerate().for_each(|(line, s)| {
        regex.find_iter(&s).for_each(|matched| {
            let message = format!("Anonymous namespace at line {}: {}", line + 1, matched.as_str().bold());
            ret.push(message);
        })
    });

    return ret;
}

#[cfg(test)]
mod tests {
    use crate::checks::namespace::find_anonymous_namespace;

    #[tokio::test]
    async fn test_anonymous_namespace() {
        let test_str = std::fs::read_to_string("tests/namespace.cpp").unwrap();
        for line in test_str.lines().filter(|s| *s != "") {
            let v = line.split("// ").collect::<Vec<&str>>();
            if v.len() < 2 {
                continue;
            }
            let test = v[0].to_string();
            let res: usize = v[1].parse().unwrap();
            assert_eq!(res, find_anonymous_namespace(&vec![test.clone()]).await.len(), "{}", test);
        }
    }
}

