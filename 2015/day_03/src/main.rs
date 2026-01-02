use std::collections::HashSet;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    // Santa delivers alone

    let mut visited_part1 = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    visited_part1.insert((x, y));

    for c in contents.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => {}
        }
        visited_part1.insert((x, y));
    }

    println!(
        "Part 1 - Houses visited by Santa alone: {}",
        visited_part1.len()
    );

    // Santa and Robot

    let mut visited_part2 = HashSet::new();
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;

    visited_part2.insert((0, 0));

    for (i, c) in contents.chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '^' => santa_y += 1,
                'v' => santa_y -= 1,
                '>' => santa_x += 1,
                '<' => santa_x -= 1,
                _ => {}
            }
            visited_part2.insert((santa_x, santa_y));
        } else {
            match c {
                '^' => robo_y += 1,
                'v' => robo_y -= 1,
                '>' => robo_x += 1,
                '<' => robo_x -= 1,
                _ => {}
            }
            visited_part2.insert((robo_x, robo_y));
        }
    }

    println!(
        "Part 2 - Houses visited by Santa and Robo-Santa: {}",
        visited_part2.len()
    );

    Ok(())
}
