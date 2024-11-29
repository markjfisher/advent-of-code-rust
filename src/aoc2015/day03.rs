use std::collections::HashSet;

type SantaPosition = (i32, i32);

pub fn part1(input: &str) -> usize {
    let mut houses: HashSet<SantaPosition> = HashSet::new();
    let mut santa_position = (0, 0);
    houses.insert(santa_position);

    input.chars().for_each(|c| {
        match c {
            '>' => santa_position.0 += 1,
            '<' => santa_position.0 -= 1,
            '^' => santa_position.1 += 1,
            'v' => santa_position.1 -= 1,
            _ => unreachable!(),
        }
        houses.insert(santa_position);
    });
    houses.len()
}

pub fn part2(input: &str) -> usize {
    let mut houses: HashSet<SantaPosition> = HashSet::new();
    let mut santa_position = (0, 0);
    let mut robot_position = (0, 0);
    let mut is_santa_move = true;
    houses.insert(santa_position);

    input.chars().for_each(|c| {
        let current_pos = if is_santa_move { &mut santa_position } else { &mut robot_position };
        match c {
            '>' => current_pos.0 += 1,
            '<' => current_pos.0 -= 1,
            '^' => current_pos.1 += 1,
            'v' => current_pos.1 -= 1,
            _ => unreachable!(),
        }
        houses.insert(*current_pos);
        is_santa_move = !is_santa_move;
    });
    houses.len()
}
