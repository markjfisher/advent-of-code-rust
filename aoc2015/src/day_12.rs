// part 1 was uber quick on cmdline!
// jq . < /home/markf/dev/personal/rust/advent-of-code-rust/aoc2015/input/2015/day12.txt | grep -o -- '-*[0-9]*' | awk '{SUM+=$0}END{print SUM}'

use serde_json::Value;

#[aoc(day12, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let v = serde_json::from_slice(input).unwrap();
    sum(v, true)
}

#[aoc(day12, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let v = serde_json::from_slice(input).unwrap();
    sum(v, false)
}

fn sum(v: Value, include_red: bool) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(v) => v.into_iter().map(|e| sum(e, include_red)).sum(),
        Value::Object(ref v) => {
            let mut max = 0;
            for v in v.values() {
                if (v == "red") && !include_red {
                    return 0;
                }
                max += sum(v.clone(), include_red);
            }
            return max;
        }
    }
}