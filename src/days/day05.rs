pub fn day05(input: &String) {
    let part: Vec<&str> = input.split("\n\n").collect();
    let mut stack = read_to_stack_vec(part.get(0).unwrap());
    move_crate(part.get(1).unwrap(), &mut stack);
    let mut last = String::from("");
    for mut s in stack {
        last.push(s.pop().unwrap());
    }
    println!("{:?}", last);
}

pub fn day05_p2(input: &String) {
    let part: Vec<&str> = input.split("\n\n").collect();
    let mut stack = read_to_stack_vec(part.get(0).unwrap());
    move_create_once(part.get(1).unwrap(), &mut stack);
    let mut last = String::from("");
    for mut s in stack {
        last.push(s.pop().unwrap());
    }
    println!("{:?}", last);
}

fn move_crate(input: &str, stack: &mut Vec<Vec<char>>) {
    for line in input.lines() {
        let line_part: Vec<&str> = line.split(" ").collect();
        let num = line_part.get(1).unwrap().parse::<usize>().unwrap();
        let from = line_part.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = line_part.get(5).unwrap().parse::<usize>().unwrap() - 1;
        for _ in 0..num {
            let pop = stack.get_mut(from).unwrap().pop().unwrap();
            stack.get_mut(to).unwrap().push(pop);
        }
    }
}

fn move_create_once(input: &str, stack: &mut Vec<Vec<char>>) {
    for line in input.lines() {
        let line_part: Vec<&str> = line.split(" ").collect();
        let num = line_part.get(1).unwrap().parse::<usize>().unwrap();
        let from = line_part.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = line_part.get(5).unwrap().parse::<usize>().unwrap() - 1;
        let from_stack = stack.get_mut(from).unwrap();

        let mut pop_stack = Vec::new();
        for _ in 0..num {
            pop_stack.push(from_stack.pop());
        }
        let to_stack = stack.get_mut(to).unwrap();
        pop_stack.reverse();
        pop_stack.iter().for_each(|item|to_stack.push(item.unwrap()));
    }
}

fn read_to_stack_vec(input: &str) -> Vec<Vec<char>> {
    // 1,5,9
    let mut result = Vec::new();
    for (line_index, line) in input.lines().filter(|s| s.contains('[')).enumerate() {
        if line_index == 0 {
            for _ in 0..(line.len() + 1) / 4 {
                result.push(Vec::new());
            }
        }

        for (i, c) in line.chars().enumerate() {
            if c != '[' && c != ']' && c != ' ' {
                let column_index = i / 4;
                result.get_mut(column_index).unwrap().push(c);
            }
        }
    }

    for i in 0..result.len() {
        result.get_mut(i).unwrap().reverse();
    }
    result
}
