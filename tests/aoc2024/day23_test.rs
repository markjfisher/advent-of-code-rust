use aoc::aoc2024::day23::*;
use assert_unordered::assert_eq_unordered;
use aoc::util::hash::*;

const EXAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 7);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 456);
}

#[test]
fn parse_test() {
    let input = parse(EXAMPLE);
    // 16 unique names: aq,cg,co,de,ka,kh,qp,ta,tb,tc,td,ub,vc,wh,wq,yn
    assert_eq!(input.len(), 16);

    // looking at tX connections:
    // ta-co
    // ta-ka
    // de-ta
    // kh-ta
    // ^^^^^^^^^^ ta
    // cg-tb
    // tb-ka
    // tb-wq
    // tb-vc
    // ^^^^^^^^^^ tb
    // kh-tc
    // wh-tc
    // tc-td
    // co-tc
    // ^^^^^^^^^^ tc
    // tc-td
    // wh-td
    // td-qp
    // td-yn
    // ^^^^^^^^^^ td

    assert_eq_unordered!(input.get("ta").unwrap(), &FastSet::from_iter(vec!["co", "ka", "de", "kh"]));
    assert_eq_unordered!(input.get("tb").unwrap(), &FastSet::from_iter(vec!["cg", "ka", "wq", "vc"]));
    assert_eq_unordered!(input.get("tc").unwrap(), &FastSet::from_iter(vec!["kh", "wh", "td", "co"]));
    assert_eq_unordered!(input.get("td").unwrap(), &FastSet::from_iter(vec!["tc", "wh", "qp", "yn"]));

    // de connections:
    // de-cg
    // de-co
    // de-ta
    // ka-de
    // ^^^^^^^^^^ de
    assert_eq_unordered!(input.get("de").unwrap(), &FastSet::from_iter(vec!["cg", "co", "ta", "ka"]));

    // ka connections:
    // ka-co
    // tb-ka
    // ta-ka
    // ka-de
    // ^^^^^^^^^^ ka
    assert_eq_unordered!(input.get("ka").unwrap(), &FastSet::from_iter(vec!["co", "tb", "ta", "de"]));

    // ka-co
    // ta-co
    // de-co
    // co-tc
    // ^^^^^^^^^^ co
    assert_eq_unordered!(input.get("co").unwrap(), &FastSet::from_iter(vec!["ka", "ta", "de", "tc"]));

    // kh-tc
    // qp-kh
    // kh-ub
    // kh-ta
    // ^^^^^^^^^^ kh
    assert_eq_unordered!(input.get("kh").unwrap(), &FastSet::from_iter(vec!["tc", "qp", "ub", "ta"]));

}
