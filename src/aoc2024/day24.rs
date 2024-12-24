use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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

// initially solved by inspection of the DOT output as svg.
pub fn part2(input: &Circuit) -> String {
    let wrong = find_wrong_outputs(input);
    // input.write_graph("circuit.dot", &wrong).unwrap();
    wrong.join(",")
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

    fn style_node(node: &str, bad_node: bool) -> String {
        if node.contains('_') {
            let parts: Vec<&str> = node.split('_').collect();
            let op = parts[1];
            let attrs = match op {
                "AND" => "shape=invtrapezium,fillcolor=yellow",
                "OR" => "shape=invtriangle,fillcolor=greenyellow",
                "XOR" => "shape=invhouse,fillcolor=lightblue,fontcolor=white",
                _ => unreachable!(),
            };
            format!("{}[label={},style=filled,{}]", node, op, attrs)
        } else if node.starts_with('x') {
            format!("{}[shape=square,style=filled,fillcolor=deepskyblue]", node)
        } else if node.starts_with('y') {
            format!("{}[shape=square,style=filled,fillcolor=dodgerblue]", node)
        } else if node.starts_with('z') {
            if bad_node {
                format!("{}[shape=square,style=filled,fillcolor=red]", node)
            } else {
                format!("{}[shape=square,style=filled,fillcolor=purple,fontcolor=white]", node)
            }
        } else if bad_node {
            format!("{}[shape=square,style=filled,fillcolor=red]", node)
        } else {
            format!("{}[shape=square,style=filled,fillcolor=lightgrey]", node)
        }
    }

    pub fn write_graph(&self, filename: &str, bad_outputs: &[String]) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        writeln!(file, "digraph 24 {{")?;

        // Write all known values and outputs as nodes
        for key in self.known_values.keys() {
            writeln!(file, "  {}", Self::style_node(key, bad_outputs.contains(key)))?;
        }
        for gate in &self.gates {
            writeln!(file, "  {}", Self::style_node(&gate.output, bad_outputs.contains(&gate.output)))?;
        }

        // Write gate connections
        for gate in &self.gates {
            let operator = format!("{}_{}_{}", 
                gate.inputs[0],
                match gate.operation {
                    Operation::And => "AND",
                    Operation::Or => "OR",
                    Operation::Xor => "XOR",
                },
                gate.inputs[1]
            );
            
            writeln!(file, "  {}", Self::style_node(&operator, false))?;
            writeln!(file, "  {} -> {};", operator, gate.output)?;
            writeln!(file, "  {} -> {};", gate.inputs[0], operator)?;
            writeln!(file, "  {} -> {};", gate.inputs[1], operator)?;
        }

        writeln!(file, "}}")?;
        Ok(())
    }
}

// After inspecting the DOT output, this logic was built up to match the inspections
pub fn find_wrong_outputs(circuit: &Circuit) -> Vec<String> {
    let mut wrong = Vec::new();
    let mut highest_z = "z00".to_string();

    // Find highest z value first
    for gate in &circuit.gates {
        if gate.output.starts_with('z') {
            if gate.output > highest_z {
                highest_z = gate.output.clone();
            }
        }
    }

    // Apply rules to find wrong outputs
    for gate in &circuit.gates {
        // Rule 1: z-output using AND/OR (not XOR) and not the highest z
        if gate.output.starts_with('z') && gate.output != highest_z {
            match gate.operation {
                Operation::And | Operation::Or => {
                    wrong.push(gate.output.clone());
                }
                _ => {}
            }
        }

        // Rule 2: XOR chains where neither inputs nor output start with x/y/z
        if matches!(gate.operation, Operation::Xor) 
           && !gate.output.starts_with(['x', 'y', 'z'])
           && !gate.inputs.iter().any(|i| i.starts_with(['x', 'y', 'z'])) {
            wrong.push(gate.output.clone());
        }

        // Rule 3: AND chains (except x00) feeding into non-OR operations
        if matches!(gate.operation, Operation::And) 
           && !gate.inputs.contains(&"x00".to_string()) {
            for other_gate in &circuit.gates {
                if other_gate.inputs.iter().any(|i| i == &gate.output)
                   && !matches!(other_gate.operation, Operation::Or) {
                    wrong.push(gate.output.clone());
                }
            }
        }

        // Rule 4: XOR outputs feeding into OR operations
        if matches!(gate.operation, Operation::Xor) {
            for other_gate in &circuit.gates {
                if other_gate.inputs.iter().any(|i| i == &gate.output)
                   && matches!(other_gate.operation, Operation::Or) {
                    wrong.push(gate.output.clone());
                }
            }
        }
    }

    wrong.sort();
    wrong.dedup();
    wrong
}