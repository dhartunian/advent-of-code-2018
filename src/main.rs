use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day-3-input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.lines();
    let mut hashy : HashMap<(u32, u32), u32> = HashMap::new();
    for line in lines {
        let words : Vec<&str> = line.split(' ').collect();
        // ["#2", "@", "518,811:", "15x18"]
        let start_coords : Vec<u32> = words[2].split_at(words[2].len()-1)
                                   .0
                                   .split(',')
                                   .map(|s| s.parse().unwrap())
                                   .collect();
        let size : Vec<u32> = words[3].split('x')
                                      .map(|s| s.parse().unwrap())
                                      .collect();
        for x in start_coords[0]..start_coords[0]+size[0] {
            for y in start_coords[1]..start_coords[1]+size[1] {
                hashy.entry((x,y)).and_modify(|x| { *x += 1 }).or_insert(1);
            }
        }
    }
    let mut counter = 0;
    for v in hashy.values() {
        if *v >= 2 {
            counter += 1;
        }
    }
    println!("{}", counter);
    Ok(())
}

fn _day_2_part_2() -> std::io::Result<()> {
    let mut file = File::open("inputs/day-2-input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.lines();
    for line1 in lines {
        let contents2 = contents.clone();
        let lines2 = contents2.lines();
        for line2 in lines2 {
            let diff = char_difference(&line1.to_string(),
                                       &line2.to_string());
            if diff {
                println!("{}", line1);
                println!("{}", line2);
            }
        }
    }
    Ok(())
}

fn char_difference(word1 : &String, word2 : &String) -> bool {
    return word1.chars()
                .zip(word2.chars())
                .filter(|x| {
                    match x {
                        (y,z) => {
                            y != z
                        }
                    }
                })
                .count() == 1;
}

fn _day2_part_1() -> std::io::Result<()> {
    let mut file = File::open("inputs/day-2-input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.lines();
    let mut count_with_two: i32 = 0;
    let mut count_with_three: i32 = 0;
    for line in lines {
        let counts = get_letter_counts(line);
        let mut already_counted_twos = false;
        let mut already_counted_threes = false;
        for (_key, value) in counts.iter() {
            if *value == 2 && !already_counted_twos {
                count_with_two += 1;
                already_counted_twos = true;
            }
            if *value == 3 && !already_counted_threes {
                count_with_three += 1;
                already_counted_threes = true;
            }
        }
    }
    println!("checksum: {}", count_with_three * count_with_two);
    Ok(())
}

fn get_letter_counts(word: &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();
    for letter in word.chars() {
        match counts.get(&letter) {
            Some(&n) => {
                //println!("Found some of {}", letter);
                counts.insert(letter, n + 1);
            }
            None => {
                //println!("Found none of {}", letter);
                counts.insert(letter, 1);
            }
        };
    }
    return counts;
}

fn _day1_part_2() -> std::io::Result<()> {
    let mut file = File::open("inputs/day-1-input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut frequencies = HashSet::new();

    let lines = contents.lines().cycle();
    let mut sum: i32 = 0;
    for line in lines {
        frequencies.insert(sum);
        let (sign, number) = line.split_at(1);
        let n : i32 = number.parse().unwrap();
        if sign == "+" {
            sum += n;
        } else if sign == "-" {
            sum -= n;
        }
        if frequencies.contains(&sum) {
            println!("Found repeat: {}", sum);
            break;
        }
    }
    Ok(())
}

fn _day1() -> std::io::Result<()> {
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

