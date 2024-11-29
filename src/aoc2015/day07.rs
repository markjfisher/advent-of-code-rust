use std::collections::HashMap;

use regex::Captures;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Signal {
    Assign(String),
    And(String, String),
    Or(String, String),
    LShift(String, usize),
    RShift(String, usize),
    Not(String),
}

pub fn parse(input: &str) -> HashMap<String, Signal> {
    let mut signals = HashMap::new();

    let re_assign = Regex::new(r"^(\w+) -> (\D+)$").unwrap();
    let re_and = Regex::new(r"^(\w+) AND (\w+) -> (\D+)$").unwrap();
    let re_or = Regex::new(r"^(\w+) OR (\w+) -> (\D+)$").unwrap();
    let re_not = Regex::new(r"^NOT (\w+) -> (\D+)$").unwrap();
    let re_lshift = Regex::new(r"^(\w+) LSHIFT (\d+) -> (\D+)$").unwrap();
    let re_rshift = Regex::new(r"^(\w+) RSHIFT (\d+) -> (\D+)$").unwrap();

    for line in input.lines() {
        let line = line.trim();
        if parsed_by(&re_assign, line, &mut |cap| {
            let name = cap.get(2).unwrap().as_str().to_string();
            signals.insert(name, Signal::Assign(
                cap.get(1).unwrap().as_str().to_string()
            ));
        }) { continue; }

        if parsed_by(&re_and, line, &mut |cap| {
            let name = cap.get(3).unwrap().as_str().to_string();
            signals.insert(name, Signal::And(
                cap.get(1).unwrap().as_str().to_string(),
                cap.get(2).unwrap().as_str().to_string()
            ));
        }) { continue; }

        if parsed_by(&re_or, line, &mut |cap| {
            let name = cap.get(3).unwrap().as_str().to_string();
            signals.insert(name, Signal::Or(
                cap.get(1).unwrap().as_str().to_string(),
                cap.get(2).unwrap().as_str().to_string()
            ));
        }) { continue; }

        if parsed_by(&re_not, line, &mut |cap| {
            let name = cap.get(2).unwrap().as_str().to_string();
            signals.insert(name, Signal::Not(
                cap.get(1).unwrap().as_str().to_string()
            ));
        }) { continue; }

        if parsed_by(&re_lshift, line, &mut |cap| {
            let name = cap.get(3).unwrap().as_str().to_string();
            signals.insert(name, Signal::LShift(
                cap.get(1).unwrap().as_str().to_string(),
                cap.get(2).unwrap().as_str().parse().unwrap()
            ));
        }) { continue; }

        if parsed_by(&re_rshift, line, &mut |cap| {
            let name = cap.get(3).unwrap().as_str().to_string();
            signals.insert(name, Signal::RShift(
                cap.get(1).unwrap().as_str().to_string(),
                cap.get(2).unwrap().as_str().parse().unwrap()
            ));
        }) { continue; }

        unreachable!("found a line that couldn't be parsed: {}", line);
    };
    signals
}

fn parsed_by<'t, F>(re: &Regex, line: &'t str, f: &mut F) -> bool
    where F: FnMut(Captures<'t>)
{
    return match re.captures(line) {
        Some(cap) => {
            f(cap);
            true
        }
        None => false,
    };
}

fn get_val(signals: &HashMap<String, Signal>, cache: &mut HashMap<String, u16>, s: &String) -> u16 {
    if cache.contains_key(s) { return *cache.get(s).unwrap(); }

    let v: Result<u16, _> = s.parse();
    if v.is_ok() { return v.unwrap(); }

    let r = match signals.get(s) {
        Some(&Signal::Assign(ref s)) => get_val(signals, cache, s),
        Some(&Signal::And(ref s1, ref s2)) => get_val(signals, cache, s1) & get_val(signals, cache, s2),
        Some(&Signal::Or(ref s1, ref s2)) => get_val(signals, cache, s1) | get_val(signals, cache, s2),
        Some(&Signal::LShift(ref s, v)) => get_val(signals, cache, s) << v,
        Some(&Signal::RShift(ref s, v)) => get_val(signals, cache, s) >> v,
        Some(&Signal::Not(ref s)) => !get_val(signals, cache, s),
        _ => unreachable!("unknown signal: {}", s),
    };
    cache.insert(s.clone(), r);
    r
}

pub fn part1(signals: &HashMap<String, Signal>) -> u16 {
    solve(signals, "a", None)
}

pub fn part2(signals: &HashMap<String, Signal>) -> u16 {
    // part 1 gives 16076, which we plug into b value as starting value
    solve(signals, "a", Some(16076))
}

pub fn solve(signals: &HashMap<String, Signal>, s: &str, b: Option<u16>) -> u16 {
    let mut cache = HashMap::new();
    if b.is_some() { cache.insert("b".to_string(), b.unwrap()); }
    get_val(signals, &mut cache, &s.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_and_input() {
        let map = parse("lf AND lq -> ls");
        let signal = map.get("ls").unwrap();
        let expected = Signal::And("lf".to_string(), "lq".to_string());
        assert_eq!(*signal, expected)
    }

    #[test]
    fn can_parse_assign_input() {
        let map = parse("100 -> x");
        let signal = map.get("x").unwrap();
        let expected = Signal::Assign("100".to_string());
        assert_eq!(*signal, expected)
    }

    #[test]
    fn can_parse_lshift_input() {
        let map = parse("ip LSHIFT 15 -> it");
        let signal = map.get("it").unwrap();
        let expected = Signal::LShift("ip".to_string(), 15);
        assert_eq!(*signal, expected)
    }

    #[test]
    fn can_parse_not_input() {
        let map = parse("NOT fx -> fy");
        let signal = map.get("fy").unwrap();
        let expected = Signal::Not("fx".to_string());
        assert_eq!(*signal, expected)
    }

    #[test]
    fn can_parse_or_input() {
        let map = parse("et OR fe -> ff");
        let signal = map.get("ff").unwrap();
        let expected = Signal::Or("et".to_string(), "fe".to_string());
        assert_eq!(*signal, expected)
    }

    #[test]
    fn can_parse_rshift_input() {
        let map = parse("ip RSHIFT 15 -> it");
        let signal = map.get("it").unwrap();
        let expected = Signal::RShift("ip".to_string(), 15);
        assert_eq!(*signal, expected)
    }

    #[test]
    fn can_do_test_data() {
        let input = create_test_data();
        let signals = parse(&input);
        assert_eq!(solve(&signals, "d", None), 72);
        assert_eq!(solve(&signals, "e", None), 507);
        assert_eq!(solve(&signals, "f", None), 492);
        assert_eq!(solve(&signals, "g", None), 114);
        assert_eq!(solve(&signals, "h", None), 65412);
        assert_eq!(solve(&signals, "i", None), 65079);
        assert_eq!(solve(&signals, "x", None), 123);
        assert_eq!(solve(&signals, "y", None), 456);
    }

    fn create_test_data() -> String {
        String::from(
            "123 -> x
            456 -> y
            x AND y -> d
            x OR y -> e
            x LSHIFT 2 -> f
            y RSHIFT 2 -> g
            NOT x -> h
            NOT y -> i")
    }
}