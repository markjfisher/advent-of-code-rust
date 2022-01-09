use std::collections::HashSet;

type SantaPosition = (i32, i32);

#[aoc(day3, part1)]
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

#[aoc(day3, part2)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // > delivers presents to 2 houses: one at the starting location, and one to the east.
    fn example1() {
        assert_eq!(part1(">"), 2);
    }

    #[test]
    // ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    fn example2() {
        assert_eq!(part1("^>v<"), 4);
    }

    #[test]
    // ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.
    fn example3() {
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    // ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
    fn example4() {
        assert_eq!(part2("^v"), 3);
    }

    #[test]
    // ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
    fn example5() {
        assert_eq!(part2("^>v<"), 3);
    }

    #[test]
    // ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
    fn example6() {
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}