use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;
    let mut nice_count = 0;

    for line in contents.lines() {
        if is_nice(line) {
            nice_count += 1;
        }
    }

    println!("Nice strings: {}", nice_count);
    Ok(())
}

fn is_nice(s: &str) -> bool {
    let mut vowel_count = 0;
    for c in s.chars() {
        if "aeiou".contains(c) {
            vowel_count += 1;
        }
    }

    if vowel_count < 3 {
        return false;
    }

    // Rule 2: At least one double letter
    let has_double = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
    if !has_double {
        return false;
    }

    // Rule 3: Does NOT contain ab, cd, pq, or xy
    if s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy") {
        return false;
    }

    true
}
