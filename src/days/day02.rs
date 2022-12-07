#![allow(dead_code)]
pub fn day02(input: &String) {
    let mut score = 0;
    for line in input.lines() {
        let round: Vec<char> = line.chars().collect();
        let first = round.get(0).unwrap();
        let second = round.get(2).unwrap();
        match second {
            'X' => score = score + 1,
            'Y' => score = score + 2,
            'Z' => score = score + 3,
            _ => (),
        }
        match (first, second) {
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => score = score + 3,
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => score = score + 6,
            _ => (),
        }
    }
    println!("{score}");
}

pub fn day02_p2(input: &String) {
    let mut score = 0;
    for line in input.lines() {
        let round: Vec<char> = line.chars().collect();
        let first = round.get(0).unwrap();
        let second = round.get(2).unwrap();
        match second {
            'X' => {
                score = score + 0;
                match first {
                    'A' => score = score + 3,  // z
                    'B' => score = score + 1,  // x
                    'C' => score = score + 2,  // y
                    _ => (),
                }
            }
            'Y' => {
                score = score + 3;
                match first {
                    'A' => score = score + 1,  // x
                    'B' => score = score + 2,  // y
                    'C' => score = score + 3,  // z
                    _ => (),
                }
            },
            'Z' => {
                score = score + 6;
                match first {
                    'A' => score = score + 2,  // y
                    'B' => score = score + 3,  // z
                    'C' => score = score + 1,  // x
                    _ => (),
                }
            },
            _ => (),
        }
    }
    println!("{score}");
}
