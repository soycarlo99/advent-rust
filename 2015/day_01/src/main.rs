use std::fs;

fn main() {
    let mut floor: i32 = 0;

    let contents = fs::read_to_string("input.txt").unwrap();

    for (index, c) in contents.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            println!(
                "Position: {} +1 , Character: {}, Floor: {}",
                index, c, floor
            );
        }
    }

    println!("floor {}", floor);
}
