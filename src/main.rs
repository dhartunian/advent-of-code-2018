use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day-1-input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let lines = contents.lines();
    let mut sum: i32 = 0;
    for line in lines {
        let (sign, number) = line.split_at(1);
        let n : i32 = number.parse().unwrap();
        if sign == "+" {
            sum += n;
        } else if sign == "-" {
            sum -= n;
        }
    }
    println!("{}", sum);
    Ok(())
}

