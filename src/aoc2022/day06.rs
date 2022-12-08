#![allow(dead_code)]
pub fn day06(input: &str) {
    window_same(input,4);
}

pub fn day06_p2(input: &str) {
    window_same(input,14);
}

fn window_same(input: &str, window_size: usize) {
    let chars = input.chars().collect::<Vec<char>>();
    let mut result = 0;
    for (i, group) in chars.windows(window_size).enumerate() {
        if !have_same(group) {
            result = i + 4;
            break;
        }
    }
    println!("{result}");
}

fn have_same(window: &[char]) -> bool {
    for (i, item) in window.iter().enumerate() {
        if i < window.len() && window[i + 1..].contains(item) {
            return true;
        }
    }
    false
}
