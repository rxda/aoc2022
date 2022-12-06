pub fn day06(input: &str) {
    let chars = input.chars().collect::<Vec<char>>();
    let mut result = 0;
    for (i, group) in chars.windows(4).enumerate() {
        if !have_same(group){
            result = i+4;
            break;
        }
    }
    println!("{result}");
}

pub fn day06_p2(input: &str) {
    let chars = input.chars().collect::<Vec<char>>();
    let mut result = 0;
    for (i, group) in chars.windows(14).enumerate() {
        if !have_same(group){
            result = i+14;
            break;
        }
    }
    println!("{result}");
}

fn have_same(window : &[char])->bool{
    for (i, item) in window.iter().enumerate() {
        if i< window.len() && window[i+1..].contains(item){
            return true;
        }
    }
    false
}