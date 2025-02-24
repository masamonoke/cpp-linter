use crate::anonymous_namespace::find_anonymous_namespace;

pub fn check(lines: Vec<String>) {
    find_anonymous_namespace(&lines);
}
