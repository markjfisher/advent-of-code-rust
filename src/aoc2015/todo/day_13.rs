// This is very similar to day 09 with locations, except the costs are different in each direction.

// Fortunately for part 2, I didn't merge the values to make simpler edges between the attendees
// so inserting the host was a matter of making another attendee with a score of 0 if not found
// in the cost map.

// However, the circular combinations list could be made more efficient, there are plenty
// of duplicate calculations going forward and backward, despite trying to limit them initially
// so this requires some work.

// Also there's a lot of creating Attendee objects flying around, I don't know if this could be
// kept as a singular list with references in the map to them, maybe with Rc<> ??

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Attendee(String);

impl Attendee {
    pub fn new(s: &str) -> Attendee {
        Attendee(s.to_string())
    }

    pub fn from(l: &Attendee) -> Attendee {
        Attendee::new(l.0.to_string().as_str())
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (HashSet<Attendee>, HashMap<(Attendee, Attendee), i32>) {
    let mut cost_map = HashMap::new();
    let mut attendees: HashSet<Attendee> = HashSet::new();

    // Reading the data is a bit hideous, and could be improved with some Regex.
    input.lines().for_each(|l| {
        let l = l.trim();
        let mut split = l.split(" ");
        let att_1 = split.next().unwrap();
        split.next(); // would
        let change = split.next().unwrap(); // gain/lose
        let diff: i32 = split.next().unwrap().parse().unwrap(); // value before change affects
        let diff = if change == "lose" { -diff } else { diff };
        for _ in 0..6 { split.next(); }
        let att_2 = split.next().unwrap().split(".").next().unwrap(); // name without "."

        // HashSet will remove duplicates naturally
        attendees.insert(Attendee::new(att_1));
        attendees.insert(Attendee::new(att_2));

        let att_pair = (Attendee::new(att_1), Attendee::new(att_2));
        cost_map.insert(att_pair, diff);
    });

    (attendees, cost_map)
}

#[aoc(day13, part1)]
pub fn part1(input: &(HashSet<Attendee>, HashMap<(Attendee, Attendee), i32>)) -> i32 {
    let (attendees, cost_map) = input;
    let perms = perms_of(attendees);
    perms.iter().map(|perm| {
        cost_of(perm, cost_map)
    }).max().unwrap()
}

#[aoc(day13, part2)]
pub fn part2(input: &(HashSet<Attendee>, HashMap<(Attendee, Attendee), i32>)) -> i32 {
    let (attendees, cost_map) = input;

    let mut extended = HashSet::new();
    let host = Attendee::new("Host");
    extended.insert(host);
    // I couldn't get Extend:extend to work here...
    attendees.iter().for_each(|a| {
        extended.insert(Attendee::from(a));
    });
    let perms = perms_of(&extended);
    perms.iter().map(|perm| {
        cost_of(perm, cost_map)
    }).max().unwrap()
}

fn cost_of(permutation: &Vec<&Attendee>, cost_map: &HashMap<(Attendee, Attendee), i32>) -> i32 {
    let mut extended = permutation.clone();
    extended.push(permutation.first().unwrap());
    let happiness = extended.windows(2).map(|w| {
        let (att_1, att_2) = (w[0], w[1]);
        let att_pair_1 = (Attendee::from(&att_1), Attendee::from(&att_2));
        let att_pair_2 = (Attendee::from(&att_2), Attendee::from(&att_1));
        // Simple change for part 2 was to default cost to 0
        let c1 = cost_map.get(&att_pair_1).unwrap_or(&0);
        let c2 = cost_map.get(&att_pair_2).unwrap_or(&0);
        c1 + c2
    }).sum();
    happiness
}

// This could be better with generics as there are 2 similar functions in day9 and here
pub fn perms_of(attendees: &HashSet<Attendee>) -> Vec<Vec<&Attendee>> {
    attendees.iter()
        .permutations(attendees.len()).filter(|v| v.first().unwrap().0 <= v.last().unwrap().0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input_data() {
        let (attendees, cost_map) = input_generator(&create_test_data());
        assert_eq!(attendees.len(), 3);
        assert_eq!(cost_map.len(), 6);

        assert!(attendees.contains(&Attendee::new("Alice")));
        assert!(attendees.contains(&Attendee::new("Bob")));
        assert!(attendees.contains(&Attendee::new("Carol")));

        let att_pair_1 = (Attendee::new("Alice"), Attendee::new("Bob"));
        let att_pair_1b = (Attendee::new("Bob"), Attendee::new("Alice"));
        let att_pair_2 = (Attendee::new("Alice"), Attendee::new("Carol"));
        let att_pair_2b = (Attendee::new("Carol"), Attendee::new("Alice"));
        let att_pair_3 = (Attendee::new("Bob"), Attendee::new("Carol"));
        let att_pair_3b = (Attendee::new("Carol"), Attendee::new("Bob"));

        assert_eq!(cost_map.get(&att_pair_1).unwrap(), &54);
        assert_eq!(cost_map.get(&att_pair_1b).unwrap(), &83);
        assert_eq!(cost_map.get(&att_pair_2).unwrap(), &-79);
        assert_eq!(cost_map.get(&att_pair_2b).unwrap(), &-62);
        assert_eq!(cost_map.get(&att_pair_3).unwrap(), &-7);
        assert_eq!(cost_map.get(&att_pair_3b).unwrap(), &60);
    }

    #[test]
    fn can_solve_test_data() {
        let data = input_generator(&create_full_test_data());
        let result = part1(&data);
        assert_eq!(result, 330);
    }

    fn create_test_data() -> String {
        String::from(
            r#"Alice would gain 54 happiness units by sitting next to Bob.
            Alice would lose 79 happiness units by sitting next to Carol.
            Bob would gain 83 happiness units by sitting next to Alice.
            Bob would lose 7 happiness units by sitting next to Carol.
            Carol would lose 62 happiness units by sitting next to Alice.
            Carol would gain 60 happiness units by sitting next to Bob."#
        )
    }

    fn create_full_test_data() -> String {
        String::from(
            r#"Alice would gain 54 happiness units by sitting next to Bob.
            Alice would lose 79 happiness units by sitting next to Carol.
            Alice would lose 2 happiness units by sitting next to David.
            Bob would gain 83 happiness units by sitting next to Alice.
            Bob would lose 7 happiness units by sitting next to Carol.
            Bob would lose 63 happiness units by sitting next to David.
            Carol would lose 62 happiness units by sitting next to Alice.
            Carol would gain 60 happiness units by sitting next to Bob.
            Carol would gain 55 happiness units by sitting next to David.
            David would gain 46 happiness units by sitting next to Alice.
            David would lose 7 happiness units by sitting next to Bob.
            David would gain 41 happiness units by sitting next to Carol."#
        )
    }
}