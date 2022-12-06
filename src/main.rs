mod days;
use std::fs;

use days::{day01, day02, day03, day04};

fn main() {
    // let input = fs::read_to_string("input/day01.txt").expect("Should have been able to read the file");
    // day01::day01(&input);
    // day01::day01_p2(&input);

    // let input = fs::read_to_string("input/day02.txt").expect("Should have been able to read the file");
    // day02::day02(&input);
    // day02::day02_p2(&input);

    // let input = fs::read_to_string("input/day03.txt").expect("Should have been able to read the file");
    // day03::day03(&input);
    // day03::day03_p2(&input);

    let input = fs::read_to_string("input/day04.txt").expect("Should have been able to read the file");
    // day04::day04(&input);
    day04::day04_p2(&input);
}
