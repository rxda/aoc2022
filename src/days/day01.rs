pub fn day01(input: &String) {
    let blocks = input.split("\n\n");
    let mut max: usize = 0;
    for block in blocks {
        let lines = block.split("\n");
        let sum = lines
            .into_iter()
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .sum::<usize>();
        if sum > max {
            max = sum;
        }
    }
    println!("{max}");
}

pub fn day01_p2(input: &String) {
    let blocks = input.split("\n\n");
    let mut sum_list = Vec::new();
    for block in blocks {
        let lines = block.split("\n");
        let sum = lines
            .into_iter()
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .sum::<usize>();
        sum_list.push(sum);
    }
    sum_list.sort();
    sum_list.reverse();
    println!("{:?}", sum_list.get(0).unwrap() + sum_list.get(1).unwrap() + sum_list.get(2).unwrap());
}