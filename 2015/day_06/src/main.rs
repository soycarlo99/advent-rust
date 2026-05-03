use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    // Create 1000x1000 grid, all lights start OFF (false)
    let mut grid = vec![vec![false; 1000]; 1000];

    for line in contents.lines() {
        process_instruction(line, &mut grid);
    }

    // Count how many lights are on
    let mut count = 0;
    for row in &grid {
        for &light in row {
            if light {
                count += 1;
            }
        }
    }

    println!("Lights on: {}", count);
}

fn process_instruction(line: &str, grid: &mut Vec<Vec<bool>>) {
    if line.starts_with("turn on") {
        let coords = parse_coords(&line[8..]); // Skip "turn on "
        turn_on(grid, coords);
    } else if line.starts_with("turn off") {
        let coords = parse_coords(&line[9..]); // Skip "turn off "
        turn_off(grid, coords);
    } else if line.starts_with("toggle") {
        let coords = parse_coords(&line[7..]); // Skip "toggle "
        toggle(grid, coords);
    }
}

fn parse_coords(s: &str) -> (usize, usize, usize, usize) {
    let parts: Vec<&str> = s.split(" through ").collect();

    let start: Vec<usize> = parts[0].split(',').map(|n| n.parse().unwrap()).collect();

    let end: Vec<usize> = parts[1].split(',').map(|n| n.parse().unwrap()).collect();

    (start[0], start[1], end[0], end[1])
}

fn turn_on(grid: &mut Vec<Vec<bool>>, coords: (usize, usize, usize, usize)) {
    let (x1, y1, x2, y2) = coords;
    for x in x1..=x2 {
        for y in y1..=y2 {
            grid[x][y] = true;
        }
    }
}

fn turn_off(grid: &mut Vec<Vec<bool>>, coords: (usize, usize, usize, usize)) {
    let (x1, y1, x2, y2) = coords;
    for x in x1..=x2 {
        for y in y1..=y2 {
            grid[x][y] = false;
        }
    }
}

fn toggle(grid: &mut Vec<Vec<bool>>, coords: (usize, usize, usize, usize)) {
    let (x1, y1, x2, y2) = coords;
    for x in x1..=x2 {
        for y in y1..=y2 {
            grid[x][y] = !grid[x][y];
        }
    }
}
