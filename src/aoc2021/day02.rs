pub fn day02(input: &str) {
    let (mut h, mut d) = (0, 0);
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(op, step)| (op, step.parse::<i32>().unwrap()))
        .for_each(|(op, step)| match op {
            "forward" => h += step,
            "down" => d += step,
            "up" => d -= step,
            _ => (),
        });
    let result = h * d;
    println!("{result}");
}

pub fn day02_p2(input: &str) {
    let (mut h, mut d) = (0, 0);
    let mut aim = 0;
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(op, step)| (op, step.parse::<i32>().unwrap()))
        .for_each(|(op, step)| match op {
            "forward" => {
                h += step;
                d += aim * step;
            }
            "down" => aim += step,
            "up" => aim -= step,
            _ => (),
        });
    let result = h * d;
    println!("{result}");
}
