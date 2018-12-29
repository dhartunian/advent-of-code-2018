use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() -> std::io::Result<()> {
    let mut file = File::open("inputs/day-4-input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut asleep_for : HashMap<u32, i32> = HashMap::new();
    let mut asleep_hourly_counts : HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let mut current_guard = 0;
    let mut asleep_at : u32 = 0;
    let mut awake_at : u32;
    let mut contents_vec = Vec::from_iter(contents.lines());
    contents_vec.sort();
    for line in contents_vec.iter() {
        let words : Vec<&str> = line.split(' ').collect();
        if words.len() == 6 {
            current_guard = words[3].split_at(1).1.parse().unwrap();
        } else {
            if words[2] == "falls" {
                asleep_at = words[1].split(':').nth(1).unwrap()
                                    .split_at(2).0.parse().unwrap();
            } else {
                awake_at = words[1].split(':').nth(1).unwrap()
                                   .split_at(2).0.parse().unwrap();
                let asleep_for_segment = awake_at as i32 - asleep_at as i32;
                asleep_for.entry(current_guard)
                          .and_modify(|x| { *x += asleep_for_segment})
                          .or_insert(asleep_for_segment);
                let guards_hours = asleep_hourly_counts.entry(current_guard)
                                                       .or_insert(HashMap::new());
                for x in asleep_at..awake_at {
                    guards_hours.entry(x).and_modify(|x| { *x += 1}).or_insert(1);
                }

            }
        }
    }
    let mut largest_guard : u32 = 0;
    let mut largest_guard_asleep_time : u32 = 0;
    let mut largest_guard_asleep_minute : u32 = 0;
    for (guard_id,guard_hours) in asleep_hourly_counts.iter() {
        for (minute, length) in guard_hours.iter() {
            if *length > largest_guard_asleep_time {
                largest_guard_asleep_time = *length;
                largest_guard_asleep_minute = *minute;
                largest_guard = *guard_id;
            }
        }
    }
    println!("The Answer: {}", largest_guard * largest_guard_asleep_minute);

/*     let mut largest_guard : u32 = 0;
    let mut largest_guard_asleep_time : i32 = 0;
    for (k,v) in asleep_for.iter() {
        if *v > largest_guard_asleep_time {
            largest_guard_asleep_time = *v;
            largest_guard = *k;
        }
    }
    println!("Largest Guard: {}", largest_guard);
    println!("Largest Guard Asleep Time: {}", largest_guard_asleep_time);
    let counts = asleep_hourly_counts.get(&largest_guard).unwrap();
    let result = counts.iter().max_by(|&(_, item), &(_, item2)| item.cmp(item2));
    println!("{:?}", result.unwrap());

    println!("The Answer: {}", result.unwrap().0 * largest_guard); */
    Ok(())
}

fn _day_4_part_1() -> std::io::Result<()> {
    let mut file = File::open("inputs/day-4-input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut asleep_for : HashMap<u32, i32> = HashMap::new();
    let mut asleep_hourly_counts : HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let mut current_guard = 0;
    let mut asleep_at : u32 = 0;
    let mut awake_at : u32;
    let mut blah = contents.lines();
    let mut vblah = Vec::from_iter(blah);
    vblah.sort();
    for line in vblah.iter() {
        let words : Vec<&str> = line.split(' ').collect();
        if words.len() == 6 {
            current_guard = words[3].split_at(1).1.parse().unwrap();
        } else {
            if words[2] == "falls" {
                asleep_at = words[1].split(':').nth(1).unwrap()
                                    .split_at(2).0.parse().unwrap();
            } else {
                awake_at = words[1].split(':').nth(1).unwrap()
                                   .split_at(2).0.parse().unwrap();
                let asleep_for_segment = awake_at as i32 - asleep_at as i32;
                asleep_for.entry(current_guard)
                          .and_modify(|x| { *x += asleep_for_segment})
                          .or_insert(asleep_for_segment);
                let guards_hours = asleep_hourly_counts.entry(current_guard)
                                                       .or_insert(HashMap::new());
                for x in asleep_at..awake_at {
                    guards_hours.entry(x).and_modify(|x| { *x += 1}).or_insert(1);
                }

            }
        }
    }
    let mut largest_guard : u32 = 0;
    let mut largest_guard_asleep_time : i32 = 0;
    for (k,v) in asleep_for.iter() {
        if *v > largest_guard_asleep_time {
            largest_guard_asleep_time = *v;
            largest_guard = *k;
        }
    }
    println!("Largest Guard: {}", largest_guard);
    println!("Largest Guard Asleep Time: {}", largest_guard_asleep_time);
    let counts = asleep_hourly_counts.get(&largest_guard).unwrap();
    let result = counts.iter().max_by(|&(_, item), &(_, item2)| item.cmp(item2));
    println!("{:?}", result.unwrap());

    println!("The Answer: {}", result.unwrap().0 * largest_guard);
    Ok(())
}

fn _day_3_part_2() -> std::io::Result<()> {
    let mut file = File::open("inputs/day-3-input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut hashy : HashMap<(u32, u32), u32> = HashMap::new();
    for line in contents.lines() {
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
    for line in contents.lines() {
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
        let id = words[0].split_at(0).1;
        let mut is_overlapping = false;
        for x in start_coords[0]..start_coords[0]+size[0] {
            for y in start_coords[1]..start_coords[1]+size[1] {
                match hashy.get(&(x,y)) {
                    Some(x) => {
                        if *x > 1 {
                            is_overlapping = true;
                        }
                    },
                    None => {
                        //pass
                    }
                };
            }
        }
        if is_overlapping == false {
            println!("{}", id);
        }
    }
    Ok(())
}

fn _day_3_part_1() -> std::io::Result<()> {
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

