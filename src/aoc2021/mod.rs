pub mod day01;
pub mod day02;
mod day03;


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::aoc2021::day01;
    use crate::aoc2021::day02;
    use crate::aoc2021::day03;

    #[test]
    fn day01_test() {
        let input = fs::read_to_string("2021input/day01.txt").expect("Should have been able to read the file");
        day01::day01(&input);
    }

    #[test]
    fn day01_p2_test() {
        let input = fs::read_to_string("2021input/day01.txt").expect("Should have been able to read the file");
        day01::day01_p2(&input);
    }

    #[test]
    fn day02_test() {
        let input = fs::read_to_string("2021input/day02.txt").expect("Should have been able to read the file");
        day02::day02(&input);
    }

    #[test]
    fn day02_p2_test() {
        let input = fs::read_to_string("2021input/day02.txt").expect("Should have been able to read the file");
        day02::day02_p2(&input);
    }

    #[test]
    fn day03_test() {
        let input = fs::read_to_string("2021input/day03.txt").expect("Should have been able to read the file");
        day03::day03(&input);
    }

    // #[test]
    // fn day03_p2_test() {
    //     let input = fs::read_to_string("2021input/day02.txt").expect("Should have been able to read the file");
    //     day03::day03_p2(&input);
    // }


}