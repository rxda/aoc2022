use std::collections::{HashMap, HashSet};

pub fn day03(input: &String) {
    let mut priorities_sum: usize = 0;
    for line in input.lines() {
        let center = line.len() / 2;
        // let line = line.to_string();
        let left = &line[0..center];
        let right = &line[center..line.len()];
        let mut left_map = HashMap::new();
        left.chars().for_each(|c| {
            if left_map.contains_key(&c) {
                left_map.insert(c, left_map.get(&c).unwrap() + 1);
            } else {
                left_map.insert(c, 1);
            }
        });

        let mut right_set = HashSet::new();
        right.chars().for_each(|c| {
            if !right_set.contains(&c) {
                match left_map.get(&c) {
                    Some(count) => {
                        priorities_sum = priorities_sum + get_property(&c) as usize;
                    }
                    None => (),
                };
            }
            right_set.insert(c);
        })
    }
    println!("{priorities_sum}")
}

pub fn get_property(c: &char) -> u8 {
    let degit = *c as u8;
    if degit >= b'a' && degit <= b'z' {
        return degit - (b'a' - 1);
    }
    if degit >= b'A' && degit <= b'Z' {
        return degit - (b'A' - 27);
    }
    0
}
