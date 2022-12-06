#![allow(dead_code)]
pub fn day04(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.split(",").collect();
        let first_range: Vec<usize> = split
            .first()
            .unwrap()
            .split("-")
            .map(|t| t.parse::<usize>().unwrap())
            .collect();
        let second_range: Vec<usize> = split
            .last()
            .unwrap()
            .split("-")
            .map(|t| t.parse::<usize>().unwrap())
            .collect();

        if first_range.get(0).unwrap() <= second_range.get(0).unwrap()
            && first_range.get(1).unwrap() >= second_range.get(1).unwrap()
        {
            count = count + 1;
        } else if second_range.get(0).unwrap() <= first_range.get(0).unwrap()
            && second_range.get(1).unwrap() >= first_range.get(1).unwrap()
        {
            count = count + 1;
        }
    }
    println!("{count}");
}

pub fn day04_p2(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.split(",").collect();
        let first_range: Vec<usize> = split
            .first()
            .unwrap()
            .split("-")
            .map(|t| t.parse::<usize>().unwrap())
            .collect();
        let second_range: Vec<usize> = split
            .last()
            .unwrap()
            .split("-")
            .map(|t| t.parse::<usize>().unwrap())
            .collect();

        if overlap(first_range.get(0).unwrap(), first_range.get(1).unwrap(),second_range.get(0).unwrap(),second_range.get(1).unwrap()){
            count = count + 1;
        }
    }
    println!("{count}");
}

fn overlap(a1: &usize, b1: &usize, a2: &usize, b2: &usize) -> bool{
    !(b1 < a2 || b2 < a1)
}
