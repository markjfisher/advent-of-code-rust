pub fn parse(input: &str) -> (u64, u64) {
    let (range_lines, ingredient_lines) = input.split_once("\n\n").unwrap();

    // range_lines are in the format "3-5", "10-14", ... on separate lines
    let mut ranges: Vec<(u64, u64)> = range_lines
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    // ingredient_lines are in the format "1", "5", "8", ... on separate lines
    let ingredients: Vec<u64> = ingredient_lines
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut in_range_count = 0;

    for ingredient in ingredients {
        let mut in_range = false;
        for (start, end) in &ranges {
            if ingredient >= *start && ingredient <= *end {
                in_range = true;
                break;
            }
        }
        if in_range {
            in_range_count += 1;
        }
    }

    // Now merge all the initial ranges and get total covered length.
    // It would help if I'd read the question properly here and not assumed it
    // was only the count of ranges that had fresh ingredients!
    // That was a wasted 10 mins.

    // sort ranges by start
    ranges.sort_by_key(|(s, _)| *s);

    // merge overlapping or touching ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            // if this range overlaps or touches the last one, merge them
            if start <= last.1.saturating_add(1) {
                if end > last.1 {
                    last.1 = end;
                }
            } else {
                // disjoint range, start a new merged segment
                merged.push((start, end));
            }
        } else {
            // first range
            merged.push((start, end));
        }
    }

    // sum the lengths of the merged ranges
    let total_length: u64 = merged
        .iter()
        .map(|(s, e)| e - s + 1)
        .sum();

    (in_range_count, total_length)
}

pub fn part1(input: &(u64, u64)) -> u64 {
    input.0
}

pub fn part2(input: &(u64, u64)) -> u64 {
    input.1
}
