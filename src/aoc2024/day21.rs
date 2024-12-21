use std::collections::{HashMap, VecDeque};


pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(_input: &[&str]) -> u32 {
    123
}

pub fn part2(_input: &[&str]) -> u32 {
    456
}

pub fn complexity(input: &str) -> u32 {
    // Create and execute the robot chain
    let mut chain = RobotChain::new(input);
    chain.add_robot(NumpadRobot::new())
        .add_robot(DirectionRobot::new())
        .add_robot(DirectionRobot::new());
    
    let result = chain.execute();
    
    // Extract the number from the input string (everything except the last 'A')
    let number: u32 = input[..input.len()-1].parse().unwrap();

    println!("result: {}, number: {}", result, number);
    
    // Return the length of the result multiplied by the input number
    (result.len() as u32) * number
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Numpad keys
    Key0,
    Key1, Key2, Key3,
    Key4, Key5, Key6,
    Key7, Key8, Key9,
    KeyA,
    // Direction pad keys
    Up, Down, Left, Right,
    DirA,
}

// Base trait defining common behavior
pub trait Robot: std::any::Any {
    fn current(&self) -> Key;
    fn key_positions(&self) -> &HashMap<Key, (i32, i32)>;
    fn adjacency(&self) -> &HashMap<Key, Vec<(Key, char)>>;
    fn set_key(&mut self, key: Key);
    
    fn char_to_key(c: char) -> Key where Self: Sized {
        panic!("char_to_key not implemented for this robot type - attempting to convert {}", c)
    }

    fn press(&mut self, key: Key) -> String {
        let path = self.find_path(key);
        self.set_key(key);
        path
    }

    fn find_path(&self, target: Key) -> String {
        if self.current() == target {
            return String::new();
        }

        // BFS to find shortest path
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        
        queue.push_back((self.current(), String::new()));
        visited.insert(self.current(), String::new());

        while let Some((pos, path)) = queue.pop_front() {
            for &(next, dir) in &self.adjacency()[&pos] {
                if !visited.contains_key(&next) {
                    let new_path = format!("{}{}", path, dir);
                    if next == target {
                        return new_path;
                    }
                    visited.insert(next, new_path.clone());
                    queue.push_back((next, new_path));
                }
            }
        }

        unreachable!()
    }
}

// Common implementation details
#[derive(Debug)]
struct RobotImpl {
    current: Key,
    key_positions: HashMap<Key, (i32, i32)>,
    adjacency: HashMap<Key, Vec<(Key, char)>>,
}

impl RobotImpl {
    fn build_adjacency(
        key_positions: HashMap<Key, (i32, i32)>,
        initial_key: Key,
        is_valid_connection: impl Fn(Key, Key) -> bool
    ) -> Self {
        let mut adjacency = HashMap::new();
        
        // Build adjacency list with movement directions
        for (key, &(col, row)) in &key_positions {
            let mut neighbors = Vec::new();
            
            // Check all possible neighbors
            for (dc, dr, dir) in [(1, 0, '>'), (-1, 0, '<'), (0, -1, '^'), (0, 1, 'v')] {
                let new_col = col + dc;
                let new_row = row + dr;
                
                // Find key at new position
                if let Some(&neighbor) = key_positions.iter()
                    .find(|&(_, &pos)| pos == (new_col, new_row))
                    .map(|(k, _)| k) {
                    if is_valid_connection(*key, neighbor) {
                        neighbors.push((neighbor, dir));
                    }
                }
            }
            
            adjacency.insert(*key, neighbors);
        }

        Self {
            current: initial_key,
            key_positions,
            adjacency,
        }
    }
}

// Concrete robot types
#[derive(Debug)]
pub struct NumpadRobot(RobotImpl);

impl NumpadRobot {
    pub fn new() -> Self {
        let mut key_positions = HashMap::new();
        
        // Define positions (col, row) for each key
        key_positions.insert(Key::Key1, (0, 2));
        key_positions.insert(Key::Key2, (1, 2));
        key_positions.insert(Key::Key3, (2, 2));
        key_positions.insert(Key::Key4, (0, 1));
        key_positions.insert(Key::Key5, (1, 1));
        key_positions.insert(Key::Key6, (2, 1));
        key_positions.insert(Key::Key7, (0, 0));
        key_positions.insert(Key::Key8, (1, 0));
        key_positions.insert(Key::Key9, (2, 0));
        key_positions.insert(Key::Key0, (1, 3));
        key_positions.insert(Key::KeyA, (2, 3));

        Self(RobotImpl::build_adjacency(
            key_positions,
            Key::KeyA,
            |_, _| true // All adjacent keys are connected in numpad
        ))
    }

}

#[derive(Debug)]
pub struct DirectionRobot(RobotImpl);

impl DirectionRobot {
    pub fn new() -> Self {
        let mut key_positions = HashMap::new();
        
        // Define positions (col, row) for direction pad
        key_positions.insert(Key::Up, (1, 0));
        key_positions.insert(Key::Left, (0, 1));
        key_positions.insert(Key::Down, (1, 1));
        key_positions.insert(Key::Right, (2, 1));
        key_positions.insert(Key::DirA, (2, 0));

        Self(RobotImpl::build_adjacency(
            key_positions,
            Key::DirA,
            |key, neighbor| matches!(
                (key, neighbor),
                (Key::DirA, Key::Up) | (Key::DirA, Key::Right) |
                (Key::Up, Key::DirA) | (Key::Up, Key::Down) |
                (Key::Down, Key::Up) | (Key::Down, Key::Left) | (Key::Down, Key::Right) |
                (Key::Left, Key::Down) |
                (Key::Right, Key::Down)
            )
        ))
    }

    fn control_robot(&self, sequence: &str, robot: &mut dyn Robot) -> String {
        let mut result = String::new();

        for c in sequence.chars() {
            let target = if robot.type_id() == std::any::TypeId::of::<NumpadRobot>() {
                NumpadRobot::char_to_key(c)
            } else if robot.type_id() == std::any::TypeId::of::<DirectionRobot>() {
                DirectionRobot::char_to_key(c)
            } else {
                panic!("Unknown robot type")
            };
            
            let path = robot.press(target);
            if !path.is_empty() {
                result.push_str(&path);
            }
            result.push('A');
        }

        result
    }

    pub fn control_numbot(&self, sequence: &str, numpad: &mut NumpadRobot) -> String {
        self.control_robot(sequence, numpad)
    }

    pub fn control_dirbot(&self, sequence: &str, dir_robot: &mut DirectionRobot) -> String {
        self.control_robot(sequence, dir_robot)
    }
}

// Implement Robot trait for both types
impl Robot for NumpadRobot {
    fn current(&self) -> Key { self.0.current }
    fn key_positions(&self) -> &HashMap<Key, (i32, i32)> { &self.0.key_positions }
    fn adjacency(&self) -> &HashMap<Key, Vec<(Key, char)>> { &self.0.adjacency }
    fn set_key(&mut self, key: Key) { self.0.current = key; }

    fn char_to_key(c: char) -> Key where Self: Sized {
        match c {
            '0' => Key::Key0,
            '1' => Key::Key1,
            '2' => Key::Key2,
            '3' => Key::Key3,
            '4' => Key::Key4,
            '5' => Key::Key5,
            '6' => Key::Key6,
            '7' => Key::Key7,
            '8' => Key::Key8,
            '9' => Key::Key9,
            'A' => Key::KeyA,
            _ => panic!("Invalid numpad key: {}", c),
        }
    }
}

impl Robot for DirectionRobot {
    fn current(&self) -> Key { self.0.current }
    fn key_positions(&self) -> &HashMap<Key, (i32, i32)> { &self.0.key_positions }
    fn adjacency(&self) -> &HashMap<Key, Vec<(Key, char)>> { &self.0.adjacency }
    fn set_key(&mut self, key: Key) { self.0.current = key; }

    fn char_to_key(c: char) -> Key where Self: Sized {
        match c {
            '^' => Key::Up,
            'v' => Key::Down,
            '<' => Key::Left,
            '>' => Key::Right,
            'A' => Key::DirA,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

// A Box<T> is a smart pointer to heap-allocated data
// dyn Robot means "any type that implements the Robot trait"
// We need these because we want to store different robot types in the same vector
pub struct RobotChain {
    sequence: String,
    robots: Vec<Box<dyn Robot>>,
}

impl RobotChain {
    pub fn new(sequence: &str) -> Self {
        Self {
            sequence: sequence.to_string(),
            robots: Vec::new(),
        }
    }

    // 'static means the Robot type must have a static lifetime
    // This is needed because we're storing it in our vector
    pub fn add_robot(&mut self, robot: impl Robot + 'static) -> &mut Self {
        self.robots.push(Box::new(robot));
        self
    }

    pub fn execute(&mut self) -> String {
        let mut current_sequence = self.sequence.clone();
        // Create a single controller robot to manage all the others
        let controller = DirectionRobot::new();
        
        for robot in &mut self.robots {
            current_sequence = controller.control_robot(&current_sequence, robot.as_mut());
        }
        current_sequence
    }
}
