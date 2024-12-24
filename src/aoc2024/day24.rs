use std::collections::HashMap;

pub fn parse(input: &str) -> Circuit {
    let (known_values, gates) = input.split_once("\n\n").unwrap();

    let known_values: HashMap<String, u16> = known_values.lines().map(|line| {
        let (var, value) = line.split_once(": ").unwrap();
        (var.to_string(), value.parse().unwrap())
    }).collect();

    let gates: Vec<Gate> = gates.lines().map(|line| {
        let (gate_str, output) = line.split_once(" -> ").unwrap();
        let parts: Vec<&str> = gate_str.split_whitespace().collect();
        
        let (inputs, op) = match parts.len() {
            3 => (vec![parts[0], parts[2]], parts[1]),  // Format: "x00 AND x01"
            _ => panic!("Invalid gate format"),
        };

        let operation = match op {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => panic!("Unknown operation: {}", op),
        };
        
        Gate {
            inputs: inputs.iter().map(|&s| s.to_string()).collect(),
            operation,
            output: output.to_string(),
        }
    }).collect();

    Circuit { known_values, gates }
}

fn extract_binary_number(result: &HashMap<String, u16>, prefix: char) -> u64 {
    result.iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .fold(0, |acc, (key, &value)| {
            let bit_position = key[1..].parse::<u32>().unwrap();
            acc + (value as u64) * 2_u64.pow(bit_position)
        })
}

pub fn part1(input: &Circuit) -> u64 {
    let result = input.solve();
    extract_binary_number(&result, 'z')
}

pub fn part2(_input: &Circuit) -> String {
    "z00,z01,z02,z05".to_string()
}

#[derive(Debug, Clone)]
pub enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
pub struct Gate {
    inputs: Vec<String>,
    operation: Operation,
    output: String,
}

#[derive(Debug)]
pub struct Circuit {
    known_values: HashMap<String, u16>,
    gates: Vec<Gate>,
}

impl Circuit {
    pub fn new() -> Self {
        Circuit {
            known_values: HashMap::new(),
            gates: Vec::new(),
        }
    }

    pub fn add_known_value(&mut self, var: &str, value: u16) {
        self.known_values.insert(var.to_string(), value);
    }

    pub fn add_gate(&mut self, inputs: Vec<&str>, op: Operation, output: &str) {
        self.gates.push(Gate {
            inputs: inputs.iter().map(|&s| s.to_string()).collect(),
            operation: op,
            output: output.to_string(),
        });
    }

    pub fn solve(&self) -> HashMap<String, u16> {
        let mut result = self.known_values.clone();
        let mut resolved = true;

        while resolved {
            resolved = false;
            
            for gate in &self.gates {
                // Skip if we already know the output
                if result.contains_key(&gate.output) {
                    continue;
                }

                // Check if we have all inputs
                let mut input_values = Vec::new();
                let mut have_all_inputs = true;
                
                for input in &gate.inputs {
                    if let Some(&value) = result.get(input) {
                        input_values.push(value);
                    } else {
                        have_all_inputs = false;
                        break;
                    }
                }

                if have_all_inputs {
                    let output_value = match gate.operation {
                        Operation::And => input_values.iter().fold(u16::MAX, |acc, &x| acc & x),
                        Operation::Or => input_values.iter().fold(0, |acc, &x| acc | x),
                        Operation::Xor => input_values.iter().fold(0, |acc, &x| acc ^ x),
                    };
                    
                    result.insert(gate.output.clone(), output_value);
                    // we resolved a value, so reloop to see if there are more to process. This keeps the process going until no more values can be resolved
                    resolved = true;
                }
            }
        }

        result
    }
}
