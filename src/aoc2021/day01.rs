pub fn day01(input: &str) {
    let nums: Vec<usize> = input
        .lines()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    let mut sum = 0;
    nums.windows(2).for_each(|w| {
        if w[1] > w[0] {
            sum += 1;
        }
    });
    println!("{sum}");
}

pub fn day01_p2(input: &str) {
    let nums: Vec<usize> = input
        .lines()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    let mut sum = 0;
    nums.windows(4).for_each(|w| {
        let sum1 = w[0] + w[1] + w[2];
        let sum2 = w[1] + w[2] + w[3];
        if sum2 > sum1 {
            sum += 1;
        }
    });
    println!("{sum}");
}
