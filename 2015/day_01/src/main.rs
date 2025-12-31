use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let mut floor: i32 = 0;

    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)?;

    for (index, c) in contents.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            println!(
                " part 2's answer: Position: {} +1 , Character: {}, Floor: {}",
                index, c, floor
            );
        }
    }

    println!("part one's answer: {}", floor);

    Ok(())
}
