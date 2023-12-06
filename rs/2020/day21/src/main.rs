use advent_of_code::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut ingredient_to_allergen_freq = HashMap::new();
    let mut allergen_total_freq = HashMap::new();
    let mut ingredients_total_freq = HashMap::new();
    read_input_as_lines("2020/day21/src/input.txt")
        .into_iter()
        .for_each(|line| {
            let (ingredients, allergens) =
                line[..line.len() - 1].split_once(" (contains ").unwrap();
            let allergens = allergens
                .split(", ")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            for allergen in allergens.iter() {
                *allergen_total_freq.entry(allergen.to_owned()).or_insert(0) += 1;
            }

            for ingredient in ingredients.split(' ') {
                *ingredients_total_freq
                    .entry(ingredient.to_owned())
                    .or_insert(0) += 1;
                let ingredient_map = ingredient_to_allergen_freq
                    .entry(ingredient.to_owned())
                    .or_insert_with(HashMap::new);
                for allergen in allergens.iter() {
                    *ingredient_map.entry(allergen.to_owned()).or_insert(0) += 1;
                }
            }
        });

    let clean_ingredients = ingredient_to_allergen_freq
        .iter()
        .filter(|(_, freq_map)| {
            freq_map
                .iter()
                .all(|(allergen, freq)| allergen_total_freq.get(allergen).unwrap() > freq)
        })
        .map(|(ingredient, _)| ingredient.to_owned())
        .collect::<HashSet<_>>();

    let answer1 = {
        ingredients_total_freq
            .iter()
            .filter(|(ingredient, _)| clean_ingredients.contains(*ingredient))
            .map(|(_, freq)| freq)
            .sum::<i32>()
    };

    let answer2 = {
        let mut solved_ingredient_to_allergen = HashMap::<String, String>::new();
        for clean_ingredient in clean_ingredients {
            ingredient_to_allergen_freq.remove(&clean_ingredient);
        }
        loop {
            let mut deduced_at_least_one_solution = false;
            for (allergen, allergen_freq) in allergen_total_freq.iter() {
                let ingredients_with_same_count = ingredient_to_allergen_freq
                    .iter()
                    .filter(|(ingredient, ingredient_freq_map)| {
                        !solved_ingredient_to_allergen.contains_key(*ingredient)
                            && ingredient_freq_map.get(allergen.as_str()).unwrap() == allergen_freq
                    })
                    .map(|(ingredient, _)| ingredient.to_owned())
                    .collect::<Vec<_>>();

                if ingredients_with_same_count.len() != 1 {
                    continue;
                }

                let target_ingredient = ingredients_with_same_count.into_iter().next().unwrap();
                solved_ingredient_to_allergen.insert(target_ingredient, allergen.clone());
                deduced_at_least_one_solution = true;
                break;
            }
            if !deduced_at_least_one_solution {
                break;
            }
        }

        let mut solution_tuples = solved_ingredient_to_allergen
            .into_iter()
            .collect::<Vec<(String, String)>>();
        solution_tuples.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        solution_tuples
            .into_iter()
            .map(|(ingredient, _)| ingredient)
            .collect::<Vec<String>>()
            .join(",")
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
