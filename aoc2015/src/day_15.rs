use std::cmp::max;

use regex::Regex;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    pub fn new(name: &str, capacity: i64, durability: i64, flavor: i64, texture: i64, calories: i64) -> Ingredient {
        Ingredient { name: name.to_string(), capacity, durability, flavor, texture, calories }
    }
}

pub fn score_ingredients(ingredients: &Vec<(&Ingredient, i64)>) -> i64 {
    let capacity: i64 = max(ingredients.iter().map(|(i, r)| i.capacity * r).sum(), 0);
    let durability: i64 = max(ingredients.iter().map(|(i, r)| i.durability * r).sum(), 0);
    let flavor: i64 = max(ingredients.iter().map(|(i, r)| i.flavor * r).sum(), 0);
    let texture: i64 = max(ingredients.iter().map(|(i, r)| i.texture * r).sum(), 0);
    capacity * durability * flavor * texture
}

#[aoc_generator(day15)]
    pub fn input_generator(input: &str) -> Vec<Ingredient> {
    // e.g. "Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5"
    let re_parse_ingredient = Regex::new(r"^(\D+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$").unwrap();
    input.lines().map(|line| {
        match re_parse_ingredient.captures(line.trim()) {
            Some(cap) => {
                let name = cap.get(1).unwrap().as_str();
                let capacity: i64 = cap.get(2).unwrap().as_str().parse().unwrap();
                let durability: i64 = cap.get(3).unwrap().as_str().parse().unwrap();
                let flavor: i64 = cap.get(4).unwrap().as_str().parse().unwrap();
                let texture: i64 = cap.get(5).unwrap().as_str().parse().unwrap();
                let calories: i64 = cap.get(6).unwrap().as_str().parse().unwrap();
                Ingredient::new(name, capacity, durability, flavor, texture, calories)
            }
            None => unreachable!()
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let ingredients = input_generator(&create_test_data());
        assert_eq!(ingredients.len(), 2);

        let butterscotch = Ingredient::new("Butterscotch", -1, -2, 6, 3, 8);
        let cinnamon = Ingredient::new("Cinnamon", 2, 3, -2, -1, 3);

        assert_eq!(ingredients[0], butterscotch);
        assert_eq!(ingredients[1], cinnamon);
    }

    #[test]
    fn can_score_ratios() {
        let butterscotch = Ingredient::new("Butterscotch", -1, -2, 6, 3, 8);
        let cinnamon = Ingredient::new("Cinnamon", 2, 3, -2, -1, 3);
        let ratios = vec![(&butterscotch, 44i64), (&cinnamon, 56)];
        assert_eq!(score_ingredients(&ratios), 62842880);

        // 0 score if any contribution leads to -ve contribution
        let ratios = vec![(&butterscotch, 67 as i64), (&cinnamon, 33)];
        assert_eq!(score_ingredients(&ratios), 0);
    }

    fn create_test_data() -> String {
        String::from(
            r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
                Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#
        )
    }
}