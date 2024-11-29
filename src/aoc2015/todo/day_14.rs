use std::cmp::min;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reindeer {
    name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
}

impl Reindeer {
    pub fn new(name: &str, speed: i32, fly_time: i32, rest_time: i32) -> Reindeer {
        Reindeer { name: name.to_string(), speed, fly_time, rest_time }
    }

    pub fn distance_travelled(&self, time: i32) -> i32 {
        let cycle_time = self.fly_time + self.rest_time;
        let whole_cycles = time / cycle_time;
        let remainder = time - whole_cycles * cycle_time;
        let extra = min(remainder, self.fly_time);
        self.speed * (whole_cycles * self.fly_time + extra)
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reindeer> {
    let re_parse_reindeer = Regex::new(r"^(\D+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$").unwrap();
    input.lines().map(|line| {
        match re_parse_reindeer.captures(line.trim()) {
            Some(cap) => {
                let name = cap.get(1).unwrap().as_str();
                let speed: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
                let fly_time: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
                let rest_time: i32 = cap.get(4).unwrap().as_str().parse().unwrap();
                Reindeer::new(name, speed, fly_time, rest_time)
            }
            None => unreachable!()
        }
    }).collect()
}

#[aoc(day14, part1)]
pub fn part1(reindeers: &Vec<Reindeer>) -> i32 {
    reindeers.iter().map(|r|{
        r.distance_travelled(2503)
    }).max().unwrap()
}

#[aoc(day14, part2)]
pub fn part2(reindeers: &Vec<Reindeer>) -> i32 {
    move_reindeers(reindeers, 2503)
}

pub fn move_reindeers(reindeers: &Vec<Reindeer>, time: i32) -> i32 {
    let mut scores: HashMap<&Reindeer, i32> = HashMap::new();
    for t in 1..=time {
        let reindeer_to_distance_at_t: Vec<(&Reindeer, i32)> = reindeers.iter().map(|r| {
            (r, r.distance_travelled(t))
        }).collect();
        let furthest_distance: i32 = reindeer_to_distance_at_t.iter().map(|(_r, d)| *d).max().unwrap();
        let reindeers_at_furthest: Vec<&Reindeer> = reindeer_to_distance_at_t.iter().filter(|(_r, d)| *d == furthest_distance).map(|(r, _d)| *r).collect();
        reindeers_at_furthest.iter().for_each(|r| {
            let s = scores.entry(*r).or_insert(0);
            *s += 1;
        });
    }
    *scores.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_distance_travelled() {
        let comet = Reindeer::new("comet", 14, 10, 127);
        let dancer = Reindeer::new("dancer", 16, 11, 162);

        assert_eq!(comet.distance_travelled(12), 140);
        assert_eq!(dancer.distance_travelled(12), 176);

        assert_eq!(comet.distance_travelled(1000), 1120);
        assert_eq!(dancer.distance_travelled(1000), 1056);
    }

    #[test]
    fn can_parse_input() {
        let input = create_input_data();
        let rs = input_generator(&input);
        let comet = Reindeer::new("Comet", 14, 10, 127);
        let dancer = Reindeer::new("Dancer", 16, 11, 162);
        assert_eq!(rs.len(), 2);
        assert_eq!(rs[0], comet);
        assert_eq!(rs[1], dancer);
    }

    #[test]
    fn can_move_reindeer() {
        let rs = input_generator(&create_input_data());
        assert_eq!(move_reindeers(&rs, 1), 1);
        assert_eq!(move_reindeers(&rs, 139), 139);
        assert_eq!(move_reindeers(&rs, 140), 139);
        assert_eq!(move_reindeers(&rs, 1000), 689);
    }

    fn create_input_data() -> String {
        String::from(
            r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
               Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#
        )
    }
}