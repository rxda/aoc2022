#![allow(dead_code)]

use std::collections::LinkedList;
fn get_dir_sizes(terminal_output: &mut LinkedList<&str>, dir_sizes: &mut Vec<i32>) -> i32 {
    let mut size = 0;
    loop {
        match terminal_output.pop_front() {
            None => break,
            Some("$ cd ..") => break,
            Some(s) => {
                if s.starts_with("$ cd") {
                    size += get_dir_sizes(terminal_output, dir_sizes);
                } else if !s.starts_with("dir") && !s.starts_with("$ ls") {
                    size += s.split(" ").collect::<Vec<&str>>()[0]
                        .parse::<i32>()
                        .unwrap();
                }
            }
        };
    }
    dir_sizes.push(size);
    size
}

fn get_dir_size(i: &mut usize, lines: &[&str], dir_sizes: &mut Vec<usize>) -> usize {
    let mut size = 0;

    while *i < lines.len() {
        let index = *i;
        *i += 1;
        let parts: Vec<&str> = lines[index].split(" ").collect();
        match parts.get(0) {
            Some(&"$") => match parts.get(1) {
                Some(&"cd") => {
                    if parts[2] == ".." {
                        break;
                    } else {
                        size += get_dir_size(i, &lines, dir_sizes);
                    }
                }
                _ => (),
            },
            None => break,
            Some(&"dir") => (),
            Some(file_size) => size += file_size.parse::<usize>().unwrap(),
        }
    }
    dir_sizes.push(size);
    size
}

pub fn day07(input: &str) {
    let mut terminal_output: LinkedList<&str> = input.split('\n').collect();
    let mut dir_sizes: Vec<i32> = Vec::new();
    let outmost_size = get_dir_sizes(&mut terminal_output, &mut dir_sizes);
    let limited_dir_sizes_sum = dir_sizes
        .iter()
        .fold(0, |sum, e| return if *e > 100_000 { sum } else { sum + e });
    let smallest_dir_to_delete = dir_sizes
        .iter()
        .filter(|e| 70_000_000 - outmost_size + *e >= 30_000_000)
        .min()
        .unwrap();

    println!("{}", limited_dir_sizes_sum);
    println!("{}", smallest_dir_to_delete);
}

pub fn day07_self(input: &str) {
    let mut dir_sizes: Vec<usize> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let outmost_size = get_dir_size(&mut (0 as usize), &lines, &mut dir_sizes);
    let limited_dir_sizes_sum = dir_sizes
        .iter()
        .fold(0, |sum, e| return if *e > 100_000 { sum } else { sum + e });
    let smallest_dir_to_delete = dir_sizes
        .iter()
        .filter(|e| 70_000_000 - outmost_size + *e >= 30_000_000)
        .min()
        .unwrap();
    println!("{}", limited_dir_sizes_sum);
    println!("{}", smallest_dir_to_delete);
}


#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn day01_test() {
        let input =
            fs::read_to_string("input/day07.txt").expect("Should have been able to read the file");
        super::day07(&input);
        super::day07_self(&input);
    }
}
