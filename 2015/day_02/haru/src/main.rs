use std::fs;
use std::str::FromStr;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut slack_total = 0;
    let mut total = 0;
    for lines in contents.lines() {
        let mut vect: Vec<usize> = vec![];
        for l in lines.split("x") {
            let a = usize::from_str(l).unwrap();
            vect.push(a);
        }
        let (x, y, z) = (vect[0], vect[1], vect[2]);

        let max = *vect.iter().max().unwrap();
        let slack = match max {
            _a if x == max => y * z,
            _b if y == max => x * z,
            _c if z == max => x * y,
            _ => 0,
        };
        slack_total += slack;
        let current = 2 * x * y + 2 * x * z + 2 * y * z;
        total += total;
        println!(
            "x {} y {} z {}, current surface {}, current slack {}",
            x, y, z, current, slack
        );
    }
    println!("this is the amount {}", total + slack_total)
}
