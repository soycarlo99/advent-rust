use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    for line in contents.lines() {
        let (prefix, suffix) = line.split_once(" -> ").unwrap();
        println!("{prefix} - {suffix}");
        for prefix in prefix.lines() {}
    }
}
