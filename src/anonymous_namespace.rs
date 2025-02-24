use regex::Regex;

pub fn find_anonymous_namespace(lines: &Vec<String>) {
    let pattern = r"namespace\s*\{|^namespace$";
    let regex = Regex::new(pattern).unwrap();
    lines.iter().enumerate().for_each(|(line, s)| {
        regex.find_iter(&s).for_each(|matched| println!("Anonymous namespace at line {}: {}", line + 1, matched.as_str()))
    });
}

