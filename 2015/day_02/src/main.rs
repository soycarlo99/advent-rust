use std::cmp::min;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in contents.lines() {
        //part 1
        let parts: Vec<&str> = line.split('x').collect();

        let l: i32 = parts[0].parse().unwrap();
        let w: i32 = parts[1].parse().unwrap();
        let h: i32 = parts[2].parse().unwrap();

        let surface_area = (2 * l * w) + (2 * w * h) + (2 * h * l);

        let smallest_side = min(l * w, min(w * h, h * l));

        let total = surface_area + smallest_side;

        total_paper += total;

        // Part 2
        let mut dims = [l, w, h];
        dims.sort();
        println!("{:?}", dims);

        let ribbon_wrap = 2 * dims[0] + 2 * dims[1];

        let ribbon_bow = l * w * h;

        let ribbon = ribbon_wrap + ribbon_bow;
        total_ribbon += ribbon;

        println!("{}x{}x{} requires {} square feet", l, w, h, total);
        println!("Tottal ribbon: {}", total_ribbon);
    }

    println!("\nTotal wrapping paper needed: {} square feet", total_paper);
}
