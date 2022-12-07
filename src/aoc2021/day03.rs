use std::{collections::HashMap, f32::consts::E};

pub fn day03(input: &str) {
    let mut line_size = 0;
    let column_size = input.lines().next().unwrap().len();
    let mut column = vec![0; column_size];
    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                column[i] = column.get(i).unwrap() + 1;
            }
        });
        line_size += 1;
    });

    let center = line_size / 2;

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    column.iter().for_each(|n| {
        if *n > center {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    });

    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
    let result = gamma * epsilon;
    println!("{result}");
}
