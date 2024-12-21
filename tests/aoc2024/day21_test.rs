use aoc::aoc2024::day21::*;

const EXAMPLE: &str = "";

fn assert_path_counts(path: &str, up: usize, down: usize, left: usize, right: usize) {
    assert_eq!(path.chars().filter(|&c| c == '^').count(), up, "wrong number of up moves");
    assert_eq!(path.chars().filter(|&c| c == 'v').count(), down, "wrong number of down moves");
    assert_eq!(path.chars().filter(|&c| c == '<').count(), left, "wrong number of left moves");
    assert_eq!(path.chars().filter(|&c| c == '>').count(), right, "wrong number of right moves");
}

#[test]
fn test_numpad_robot_movement() {
    let mut robot = NumpadRobot::new();
    
    // Starts at A
    let path = robot.find_path(Key::Key7);
    assert_path_counts(&path, 3, 0, 2, 0);
    robot.set_key(Key::Key7);
    
    let path = robot.find_path(Key::Key3);
    assert_path_counts(&path, 0, 2, 0, 2);
    robot.set_key(Key::Key3);
    
    let path = robot.find_path(Key::Key5);
    assert_path_counts(&path, 1, 0, 1, 0);
}

#[test]
fn test_direction_robot_movement() {
    let mut robot = DirectionRobot::new();
    
    // Starts at A
    let path = robot.find_path(Key::Up);
    assert_path_counts(&path, 0, 0, 1, 0);
    robot.set_key(Key::Up);
    
    let path = robot.find_path(Key::Left);
    assert_path_counts(&path, 0, 1, 1, 0);
    robot.set_key(Key::Left);
    
    let path = robot.find_path(Key::DirA);
    assert_path_counts(&path, 1, 0, 0, 2);
}

#[test]
fn test_code_example() {
    let mut dr1 = DirectionRobot::new();
    let dr2 = DirectionRobot::new();
    let mut nr = NumpadRobot::new();
    
    let result = dr1.control_numbot("029A", &mut nr);
    assert_eq!(result, "<A^A>^^AvvvA");

    let result = dr2.control_dirbot(&result, &mut dr1);
    assert_eq!(result, "v<<A>>^A<A>AvA<^AA>A<vAAA>^A");

}

#[test]
fn test_direction_sequence() {
    let direction_robot = DirectionRobot::new();
    let mut numpad_robot = NumpadRobot::new();
    
    let result = direction_robot.control_numbot("97A", &mut numpad_robot);
    assert_eq!(result, "^^^A<<A>>vvvA");

}

#[test]
fn test_control_chain_of_3() {
    let dr3 = DirectionRobot::new();
    let mut dr2 = DirectionRobot::new();
    let mut dr1 = DirectionRobot::new();
    let mut nr = NumpadRobot::new();

    let dir1_instructions = dr2.control_numbot("97A", &mut nr);
    assert_eq!(dir1_instructions, "^^^A<<A>>vvvA");
    
    let dir2_instructions = dr2.control_dirbot(&dir1_instructions, &mut dr1);
    assert_eq!(dir2_instructions, "<AAA>A<v<AA>^>AvAA<AAA^>A");

    let dir3_instructions = dr3.control_dirbot(&dir2_instructions, &mut dr2);
    assert_eq!(dir3_instructions, "<v<A>^>AAAvA<^>A<v<A>A<A>^>AAvA<^A>vA<^>A<vA^>AA<v<A>^>AAA<A>vA<^>A");
}

#[test]
fn test_robot_chain() {
    let mut chain = RobotChain::new("97A");
    chain.add_robot(NumpadRobot::new())
        .add_robot(DirectionRobot::new())
        .add_robot(DirectionRobot::new());
        
    let result = chain.execute();
    assert_eq!(result, "<v<A>^>AAAvA<^>A<v<A>A<A>^>AAvA<^A>vA<^>A<vA^>AA<v<A>^>AAA<A>vA<^>A");
}

#[test]
fn test_simple_dirbot_control() {
    let dr2 = DirectionRobot::new();
    let mut dr1 = DirectionRobot::new();
    
    let dir2_instructions = dr2.control_dirbot("<^", &mut dr1);
    assert_eq!(dir2_instructions, "<v<A>^A");
}

#[test]
fn test_complexity() {
    // finding 80x29, should be 68x29
    assert_eq!(complexity("029A"), 1972);
}

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 123);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 456);
}