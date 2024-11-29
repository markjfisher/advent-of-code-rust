use crate::util::hash::*;

use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Location(String);

impl Location {
    pub fn new(s: &str) -> Location {
        Location(s.to_string())
    }

    // I'm not sure about this. should I just be using the refs instead of copying them around?
    pub fn from(l: &Location) -> Location {
        Location::new(l.0.to_string().as_str())
    }
}

pub fn parse(input: &str) -> (i32, i32) {
    let mut cost_map = FastMap::default();
    let mut locations: FastSet<Location> = FastSet::default();

    input.lines().for_each(|l| {
        let l = l.trim();
        let mut split = l.split(" ");
        let loc_1 = split.next().unwrap();
        split.next(); // to
        let loc_2 = split.next().unwrap();
        split.next(); // =
        let cost: i32 = split.next().unwrap().parse().unwrap();

        // HashSet will remove duplicates naturally
        locations.insert(Location::new(loc_1));
        locations.insert(Location::new(loc_2));

        // alphabetical order of pair for map
        let loc_pair = if loc_1 <= loc_2 { (Location::new(loc_1), Location::new(loc_2)) } else { (Location::new(loc_2), Location::new(loc_1)) };
        cost_map.insert(loc_pair, cost as i32);

    });

    let perms = perms_of(&locations);
    let (min, max) = perms.iter()
        .map(|perm| cost_of(perm, &cost_map))
        .fold((i32::MAX, i32::MIN), |(min, max), cost| {
            (min.min(cost), max.max(cost))
        });

    (min, max)
}

pub fn part1(input: &(i32, i32)) -> i32 {
    input.0
}

pub fn part2(input: &(i32, i32)) -> i32 {
    input.1
}

fn cost_of(permutation: &Vec<&Location>, cost_map: &FastMap<(Location, Location), i32>) -> i32 {
    permutation.windows(2).map(|w| {
        let (loc_1, loc_2) = (w[0], w[1]);
        let loc_pair = ordered_location_pair(loc_1, loc_2);
        cost_map.get(&loc_pair).unwrap()
    }).sum()
}

fn ordered_location_pair(loc_1: &Location, loc_2: &Location) -> (Location, Location) {
    if loc_1.0 <= loc_2.0 { (Location::from(loc_1), Location::from(loc_2)) } else { (Location::from(loc_2), Location::from(loc_1)) }
}


pub fn perms_of(locations: &FastSet<Location>) -> Vec<Vec<&Location>> {
    locations.iter()
        .permutations(locations.len()).filter(|v| v.first().unwrap().0 <= v.last().unwrap().0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perms() {
        let data: FastSet<_> = [
            Location::new("a"),
            Location::new("b"),
            Location::new("c"),
            Location::new("d"),
            Location::new("e"),
        ].into_iter().collect();
        let permutations = perms_of(&data);
        assert_eq!(permutations.len(), 60);
    }

    #[test]
    fn can_parse_input_data() {
        let (min, max) = parse(&create_test_data());
        assert_eq!(min, 605);
        assert_eq!(max, 982);
    }

    #[test]
    fn can_do_part1_on_test_data() {
        let data = parse(&create_test_data());
        assert_eq!(part1(&data), 605);
    }

    #[test]
    fn can_do_part2_on_test_data() {
        let data = parse(&create_test_data());
        assert_eq!(part2(&data), 982);
    }

    fn create_test_data() -> String {
        String::from(
            r#"London to Dublin = 464
                London to Belfast = 518
                Dublin to Belfast = 141"#
        )
    }
}