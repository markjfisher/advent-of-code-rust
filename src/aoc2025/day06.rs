pub fn parse(input: &str) -> (u64, u64) {
    // For part 1, we can simply parse the numbers as they are.
    // However, the numbers are in a particular formation deliberately for part 2!
    //  123 328  51 64 
    //   45 64  387 23 
    //    6 98  215 314
    //  *   +   *   +
    let mut lines = input.lines();
    let mut lines_text = Vec::new();
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut symbols_line = String::new();

    while let Some(line) = lines.next() {
        lines_text.push(line.to_string());
        if line.starts_with('*') || line.starts_with('+') {
            symbols = line.split_whitespace().map(|s| s.chars().next().unwrap()).collect();
            if !symbols_line.is_empty() {
                panic!("Found too many symbols lines");
            }
            symbols_line = line.to_string();
        } else {
            numbers.push(line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
        }
    }
    let total_columns = numbers[0].len();
    let total_rows = numbers.len();

    let mut result: u64 = 0;
    // take the numbers column by column and apply the symbols to the numbers in that column
    for current_column in 0..total_columns {
        let current_symbol = symbols[current_column];
        let mut current_column_result = if current_symbol == '*' { 1 } else { 0 };
        for current_row in 0..total_rows {
            match current_symbol {
                '*' => current_column_result *= numbers[current_row][current_column],
                '+' => current_column_result += numbers[current_row][current_column],
                _ => panic!("Invalid symbol"),
            }
        }
        result += current_column_result;
    }

    // for part 2, we will use the symbols_line exactly as given to calculate the starting columns of numbers
    // and work out the rectangles of the data that we need to use when perfoming the calculations.
    
    // calculate the column index of symbols from the symbols_line
    let mut symbol_column_indices = Vec::new();
    let mut current_column_index = 0;
    for c in symbols_line.chars() {
        if c == '*' || c == '+' {
            symbol_column_indices.push(current_column_index);
        }
        current_column_index += 1;
    }

    let mut p2_sum = 0u64;
    for block in 0..symbols.len() {
        let current_symbol = symbols[block];
        let mut nums_in_block = Vec::new();
        let current_column_index = symbol_column_indices[block];
        let current_column_end_index = if block + 1 < symbol_column_indices.len() { symbol_column_indices[block + 1] - 1 } else { lines_text[0].len() };

        for c in current_column_index..current_column_end_index {
            // accumulate the characters and convert to a number
            let mut num = String::new();
            for r in 0..total_rows {
                let ch = lines_text[r].chars().nth(c).unwrap();
                num.push(ch);
            }
            // we can trim the number to remove space around it, as numbers in columns may start with a space
            nums_in_block.push(num.trim().parse::<u64>().unwrap());
        }

        // fold all the numbers in nums_in_block using the current_symbol
        let mut result_for_current_block = if current_symbol == '*' { 1 } else { 0 };
        for n in nums_in_block.iter() {
            match current_symbol {
                '*' => result_for_current_block *= n,
                '+' => result_for_current_block += n,
                _ => panic!("Invalid symbol"),
            }
        }
        p2_sum += result_for_current_block;
    }

    (result, p2_sum)
}

pub fn part1(input: &(u64, u64)) -> u64 {
    input.0
}

pub fn part2(input: &(u64, u64)) -> u64 {
    input.1
}
