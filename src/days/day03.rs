#![allow(dead_code)]
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
                    Some(_) => {
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

pub fn day03_p2(input: &String) {
    let mut priorities_sum: usize = 0;
    let mut i = 0;
    let lines: Vec<&str> = input.lines().collect();
    while i < lines.len() {
        // let mut char_map: HashMap<char, usize> = HashMap::new();
        // lines.get(i).unwrap().chars().into_iter().for_each(|c| {
        //     char_map.insert(c, 1);
        // });
        // lines.get(i + 1).unwrap().chars().into_iter().for_each(|c| {
        //     if char_map.contains_key(&c) && *char_map.get(&c).unwrap() == 1 {
        //         char_map.insert(c, 2);
        //     }
        // });
        // lines.get(i + 2).unwrap().chars().into_iter().for_each(|c| {
        //     if char_map.contains_key(&c) && *char_map.get(&c).unwrap() == 2 {
        //         char_map.insert(c, 3);
        //         return;
        //     }
        // });
        // for (c, count) in char_map {
        //     if count == 3 {
        //         priorities_sum = priorities_sum + get_property(&c) as usize;
        //         break;
        //     }
        // }

        let line1_set: HashSet<char> = lines.get(i).unwrap().chars().collect();
        let line2_set: HashSet<char> = lines.get(i + 1).unwrap().chars().collect();
        let line3_set: HashSet<char> = lines.get(i + 2).unwrap().chars().collect();
        for ch in line1_set.intersection(&line2_set) {
            if line3_set.contains(ch) {
                priorities_sum = priorities_sum + get_property(&ch) as usize;
                break;
            }
        }
        i = i + 3;
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
