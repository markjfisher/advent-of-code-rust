pub fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

pub fn part1(input: &[u32]) -> u64 {
    part1_expanded(input)
        .iter()
        .enumerate()
        .filter(|(_, &x)| x != -1)
        .map(|(i, &x)| i as u64 * x as u64)
        .sum::<u64>()
}

pub fn part1_expanded(digits: &[u32]) -> Vec<i32> {
    // the vector is the length of the sum of the digits
    let total = digits.iter().sum::<u32>();

    let mut expanded: Vec<i32> = vec![-1; total as usize];
    let mut current_id: i32 = 0;
    let mut pos: u32 = 0;

    let mut is_data = true;
    for digit in digits {
        if is_data {
            for _ in 0..*digit {
                expanded[pos as usize] = current_id;
                pos += 1;
            }
            current_id += 1;
        } else {
            pos += *digit;
        }
        is_data = !is_data;
    }

    // println!("{}", format_sequence(&expanded));

    // we have the expanded values in a vector with positions for their values, e.g.
    // 00...111...2...333.44.5555.6666.777.888899
    // now we need to move values to the start from the end. The total number of moves is the number of blanks

    let mut current_back_index = expanded.len() - 1;
    let mut current_front_index = 0;
    while current_front_index < current_back_index {
        while expanded[current_back_index] == -1 {
            current_back_index -= 1;
        }
        while expanded[current_front_index] != -1 {
            current_front_index += 1;
        }
        expanded[current_front_index] = expanded[current_back_index];
        expanded[current_back_index] = -1;
        current_back_index -= 1;
        current_front_index += 1;
        // println!("{}", format_sequence(&expanded));
    }

    expanded
}

pub fn part2(input: &[u32]) -> u64 {
    let mut blocks = part2_expanded(input);

    // Start from the end, try to move each non-empty block. Space will be merged down as we go
    for i in (0..blocks.len()).rev() {
        if blocks[i].id != -1 {
            try_move_block(i, &mut blocks);
        }
    }

    // Flatten the blocks into individual values, using 0 for empty spaces. this allows us to find the sum of the index products easily
    let flattened: Vec<u64> = blocks
        .iter()
        .flat_map(|block| {
            std::iter::repeat(if block.id == -1 { 0 } else { block.id as u64 })
                .take(block.length as usize)
        })
        .collect();

    flattened
        .iter()
        .enumerate()
        .map(|(i, &id)| i as u64 * id)
        .sum()
}

// convert input to a series of blocks either having data (an id) or empty (-1)
fn part2_expanded(input: &[u32]) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut current_id = 0;
    for (i, &digit) in input.iter().enumerate() {
        let block = Block {
            id: if i % 2 == 0 { current_id } else { -1 },
            length: digit,
        };
        blocks.push(block);
        if i % 2 == 1 {
            current_id += 1;
        }
    }

    blocks
}

fn _format_sequence(values: &[i32]) -> String {
    values
        .iter()
        .map(|&x| {
            if x == -1 {
                '.'
            } else {
                char::from_digit(x as u32, 10).unwrap()
            }
        })
        .collect()
}

// a block has either an id -1 for empty or a number for its data block id.
#[derive(Debug, Clone)]
pub struct Block {
    id: i32,
    length: u32,
}

impl Block {
    pub fn new(id: i32, length: u32) -> Self {
        Block { id, length }
    }
}

pub fn format_blocks(blocks: &[Block]) -> String {
    let mut result = String::new();

    for block in blocks {
        if block.id == -1 {
            result.push_str(&".".repeat(block.length as usize));
        } else {
            let digit = char::from_digit(block.id as u32, 10).unwrap();
            result.push_str(&digit.to_string().repeat(block.length as usize));
        }
    }

    result
}

pub fn try_move_block(block_index: usize, blocks: &mut Vec<Block>) -> bool {
    let block = &blocks[block_index];
    let block_length = block.length;
    let block_id = block.id;

    // Find first fitting gap
    for i in 0..block_index {
        if blocks[i].id == -1 && blocks[i].length >= block_length {
            // Found a suitable gap
            let gap_length = blocks[i].length;

            // Set the gap's id to the moving block's id and adjust length
            blocks[i].id = block_id;
            blocks[i].length = block_length;

            // If there was extra space, insert a new gap after
            if gap_length > block_length {
                blocks.insert(i + 1, Block::new(-1, gap_length - block_length));
            }

            // Convert the original block to a gap
            blocks[block_index + if gap_length > block_length { 1 } else { 0 }].id = -1;

            // Merge any adjacent gaps - this isn't needed as we only iterate once and don't need to create larger spaces
            // Had the challenge required us to keep merging down, we would need this
            // merge_gaps(blocks);

            return true;
        }
    }

    false
}

pub fn merge_gaps(blocks: &mut Vec<Block>) {
    let mut i = 0;
    while i < blocks.len() - 1 {
        if blocks[i].id == -1 && blocks[i + 1].id == -1 {
            // merge adjacent gaps
            blocks[i].length += blocks[i + 1].length;
            blocks.remove(i + 1);
        } else {
            i += 1;
        }
    }
}
