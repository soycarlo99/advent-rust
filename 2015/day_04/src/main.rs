use md5;

fn main() {
    let secret = "iwrupvqb";
    let mut number = 1;

    loop {
        // secret + number
        let input = format!("{}{}", secret, number);

        let hash = md5::compute(input.as_bytes());
        let hash_string = format!("{:x}", hash);

        if hash_string.starts_with("000000") {
            println!("Found it! Number: {}", number);
            println!("Hash: {}", hash_string);
            break;
        }

        number += 1;
    }
}
