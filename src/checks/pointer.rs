use colored::Colorize;
use regex::Regex;

pub fn find_non_prefixed_pointer(lines: &Vec<String>, prefix: &str) -> Vec<String> {
    let asterisk_ptr_pattern = r"\b\w+\s*\*\s*(?:p[^A-Z\s]\w*|[^p\s]\w*)\s*=\s*[^;]+";
    let prefixed = format!(r"\s*(?:{}[^A-Z\s]\w*|[^p\s]\w*)\s*", prefix);

    let p1 = format!(r"\b(?:(auto|(std::unique_ptr|std::shared_ptr|std::weak_ptr)(<[^>]+>)?){}=\s*(std::make_unique|std::make_shared|)(<[^>]+>)?\((.*?)\)", prefixed);
    let p2 = format!(r"(std::unique_ptr|std::shared_ptr|std::weak_ptr)<[^>]+>{}=\s*\w+\(\))", prefixed);
    let smart_ptr_pattern = format!(r"{}|{}", p1, p2);
    let regex = Regex::new(format!("({})|({})", asterisk_ptr_pattern, smart_ptr_pattern).as_str()).unwrap();

    let mut ret = vec![];
    lines.iter().enumerate().for_each(|(line, s)| {
        regex
            .find_iter(&s)
            .for_each(|matched| {
                let message = format!("Pointer at line {} not {}-prefixed: {}", line + 1, prefix, matched.as_str().bold());
                ret.push(message);
            })
    });

    return ret;
}

#[cfg(test)]
mod tests {
    use crate::checks::pointer::find_non_prefixed_pointer;

    #[test]
    fn test_ptr_prefix() {
        let test_str = std::fs::read_to_string("tests/pointer_prefix.cpp").unwrap();
        for line in test_str.lines().filter(|s| *s != "") {
            let v = line.split("// ").collect::<Vec<&str>>();
            let test = v[0].to_string();
            let res: usize = v[1].parse().unwrap();
            assert_eq!(res, find_non_prefixed_pointer(&vec![test.clone()], "p").len(), "{}", test);
        }
    }
}
