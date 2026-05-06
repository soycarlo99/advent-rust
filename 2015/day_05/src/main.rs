use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut nice_count = 0;

    for line in contents.lines() {
        if is_nice(line) {
            nice_count += 1;
        }
    }

    println!("Nice strings: {}", nice_count);
}

fn has_repeated_pair(s: &str) -> bool {
    s.as_bytes()
        .windows(2)
        .enumerate()
        .any(|(i, pair)| s.as_bytes()[i + 2..].windows(2).any(|later| later == pair))
}

fn has_split_pair(s: &str) -> bool {
    let it1 = s.chars();
    // let it2 = s.chars().skip(1);
    let it3 = s.chars().skip(2);

    it1.zip(it3).any(|(a, b)| a == b)
}

fn is_nice(s: &str) -> bool {
    has_repeated_pair(s) && has_split_pair(s)
}

//PART 1
//
// fn has_double_letter(s: &str) -> bool {
//     let it1 = s.chars();
//     let it2 = s.chars().skip(1);
//     it1.zip(it2).any(|(a, b)| a == b)
// }
//
// fn is_nice(s: &str) -> bool {
//     let vowel_count = s.chars().filter(|c| "aeiou".contains(*c)).count();
//     if vowel_count < 3 {
//         return false;
//     }
//
//     if !has_double_letter(s) {
//         return false;
//     }
//
//     // rule 3: does NOT contain "ab", "cd", "pq", or "xy"
//     if ["ab", "cd", "pq", "xy"].iter().any(|bad| s.contains(bad)) {
//         return false;
//     }
//
//     true
// }
