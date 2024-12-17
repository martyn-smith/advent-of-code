fn count_xmas(x: (usize, usize), board: &Vec<Vec<char>>) -> usize {
    let right = [
        (Some(x.0), Some(x.1)),
        (Some(x.0), Some(x.1 + 1)),
        (Some(x.0), Some(x.1 + 2)),
        (Some(x.0), Some(x.1 + 3)),
    ]
    .into_iter()
    .filter_map(|x| get(x, board))
    .collect::<String>();
    let left = [
        (Some(x.0), Some(x.1)),
        (Some(x.0), x.1.checked_sub(1)),
        (Some(x.0), x.1.checked_sub(2)),
        (Some(x.0), x.1.checked_sub(3)),
    ]
    .into_iter()
    .filter_map(|x| get(x, board))
    .collect::<String>();
    let down = [
        (Some(x.0), Some(x.1)),
        (Some(x.0 + 1), Some(x.1)),
        (Some(x.0 + 2), Some(x.1)),
        (Some(x.0 + 3), Some(x.1)),
    ]
    .into_iter()
    .filter_map(|x| get(x, board))
    .collect::<String>();
    let up = [
        (Some(x.0), Some(x.1)),
        (x.0.checked_sub(1), Some(x.1)),
        (x.0.checked_sub(2), Some(x.1)),
        (x.0.checked_sub(3), Some(x.1)),
    ]
    .into_iter()
    .filter_map(|x| get(x, board))
    .collect::<String>();
    let up_right = [
        (Some(x.0), Some(x.1)),
        (x.0.checked_sub(1), Some(x.1 + 1)),
        (x.0.checked_sub(2), Some(x.1 + 2)),
        (x.0.checked_sub(3), Some(x.1 + 3)),
    ]
    .into_iter()
    .filter_map(|x| get(x, board))
    .collect::<String>();
    let down_right = [
        (Some(x.0), Some(x.1)),
        (Some(x.0 + 1), Some(x.1 + 1)),
        (Some(x.0 + 2), Some(x.1 + 2)),
        (Some(x.0 + 3), Some(x.1 + 3)),
    ]
    .into_iter()
    .filter_map(|x| get(x, board))
    .collect::<String>();
    let up_left = [
        (Some(x.0), Some(x.1)),
        (x.0.checked_sub(1), x.1.checked_sub(1)),
        (x.0.checked_sub(2), x.1.checked_sub(2)),
        (x.0.checked_sub(3), x.1.checked_sub(3)),
    ]
    .into_iter()
    .filter_map(|x| get(x, board))
    .collect::<String>();
    let down_left = [
        (Some(x.0), Some(x.1)),
        (Some(x.0 + 1), x.1.checked_sub(1)),
        (Some(x.0 + 2), x.1.checked_sub(2)),
        (Some(x.0 + 3), x.1.checked_sub(3)),
    ]
    .into_iter()
    .filter_map(|x| get(x, board))
    .collect::<String>();
    [
        left, right, up, down, up_right, down_right, up_left, down_left,
    ]
    .into_iter()
    .filter(|s| s == "XMAS")
    .count()
}

fn count_x_mas(a: (usize, usize), board: &Vec<Vec<char>>) -> usize {
    let up_left = [
        (Some(a.0 + 1), a.1.checked_sub(1)),
        (Some(a.0), Some(a.1)),
        (a.0.checked_sub(1), Some(a.1 + 1)),
    ]
    .into_iter()
    .filter_map(|x| get(x, board))
    .collect::<String>();
    let down_right = [
        (a.0.checked_sub(1), a.1.checked_sub(1)),
        (Some(a.0), Some(a.1)),
        (Some(a.0 + 1), Some(a.1 + 1)),
    ]
    .into_iter()
    .filter_map(|x| get(x, board))
    .collect::<String>();
    if (up_left == "MAS" || up_left == "SAM") && (down_right == "MAS" || down_right == "SAM") {
        1
    } else {
        0
    }
}

fn get(pos: (Option<usize>, Option<usize>), board: &Vec<Vec<char>>) -> Option<char> {
    let y = pos.0?;
    let x = pos.1?;
    let row = board.get(y)?;
    row.get(x).copied()
}

pub fn get_input() -> Vec<Vec<char>> {
    include_str!("../../data/4.txt")
        .lines()
        .map(|l| {
            l //.strip('\n')
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part_1(input: &Vec<Vec<char>>) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, r)| {
            r.iter()
                .enumerate()
                .filter_map(|(j, &c)| {
                    if c == 'X' {
                        Some(count_xmas((i, j), &input))
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn part_2(input: &Vec<Vec<char>>) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, r)| {
            r.iter()
                .enumerate()
                .filter_map(|(j, &c)| {
                    if c == 'A' {
                        Some(count_x_mas((i, j), &input))
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}
