fn day08(input: &str) {
    let mut matrix = Vec::<Vec<u8>>::new();
    input.lines().for_each(|l| {
        let line_vec: Vec<u8> = l
            .chars()
            .into_iter()
            .map(|c| c.to_string().parse::<u8>().unwrap())
            .collect();
        matrix.push(line_vec);
    });

    let row_num = matrix.len();
    let column_num = matrix.get(0).unwrap().len();
    let mut visible_num = 2 * (row_num + column_num - 2);

    for x in 1..row_num - 1 {
        for y in 1..column_num - 1 {
            let visible = check_inside_visible((x, y), &matrix);
            println!("{visible}");
        }
    }
    println!("{visible_num}")
}

fn day08_p2(input: &str) {
    let mut matrix = Vec::<Vec<u8>>::new();
    input.lines().for_each(|l| {
        let line_vec: Vec<u8> = l
            .chars()
            .into_iter()
            .map(|c| c.to_string().parse::<u8>().unwrap())
            .collect();
        matrix.push(line_vec);
    });

    let row_num = matrix.len();
    let column_num = matrix.get(0).unwrap().len();
    let mut max_visibility = 0;
    for x in 1..row_num - 1 {
        for y in 1..column_num - 1 {
            let visiblity = cal_visiblity((x, y), &matrix);
            if max_visibility < visiblity {
                max_visibility = visiblity;
            }
        }
    }
    println!("{max_visibility}")
}

fn check_inside_visible(inside_pos: (usize, usize), matrix: &Vec<Vec<u8>>) -> bool {
    let inside = *matrix.get(inside_pos.0).unwrap().get(inside_pos.1).unwrap();
    // left, right, up, down
    let mut visible = [true, true, true, true];

    // left, right
    for (i, num) in matrix.get(inside_pos.0).unwrap().iter().enumerate() {
        if inside <= *num {
            if i < inside_pos.1 {
                visible[0] = false
            }

            if i > inside_pos.1 {
                visible[1] = false
            }
        }
        if !visible[0] && !visible[1] {
            break;
        }
    }
    if visible[0] || visible[1] {
        return true;
    }

    for j in 0..matrix.len() {
        let num = matrix.get(j).unwrap().get(inside_pos.1).unwrap();
        if inside <= *num {
            if j < inside_pos.0 {
                visible[2] = false
            }

            if j > inside_pos.0 {
                visible[3] = false
            }
        }
        if !visible[2] && !visible[3] {
            break;
        }
    }

    visible[2] || visible[3]
}

fn cal_visiblity(pos: (usize, usize), matrix: &Vec<Vec<u8>>) -> u64 {
    let value = *matrix.get(pos.0).unwrap().get(pos.1).unwrap();
    // left, right, up, down
    let mut visible = [0, 0, 0, 0];

    let line = matrix.get(pos.0).unwrap();
    for i in (0..pos.1).rev() {
        visible[0] += 1;
        if line[i] >= value {
            break;
        }
    }

    for i in pos.1 + 1..line.len() {
        visible[1] += 1;
        if line[i] >= value {
            break;
        }
    }

    let column: Vec<u8> = matrix
        .iter()
        .map(|line| *line.get(pos.1).unwrap())
        .collect();

    for i in (0..pos.0).rev() {
        visible[2] += 1;
        if column[i] >= value {
            break;
        }
    }

    for i in pos.0 + 1..column.len() {
        visible[3] += 1;
        if column[i] >= value {
            break;
        }
    }

    for (i, e) in visible.into_iter().enumerate() {
        if e == 0 {
            visible[i] = 1;
        }
    }

    return visible[0] as u64 * visible[1] as u64 * visible[2] as u64 * visible[3] as u64;
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn day08_test() {
        let input =
            fs::read_to_string("input/day08.txt").expect("Should have been able to read the file");
        super::day08_p2(&input);
    }
}
