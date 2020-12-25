fn main() {
  let before = std::time::Instant::now();
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    panic!("Not enough arguments");
  }
  let data = std::fs::read_to_string(&args[1]).unwrap();
  let before_parsing = std::time::Instant::now();
  let foods = data.lines().map(|l| parse_line(l)).collect();
  let parsing_time = before_parsing.elapsed();

  solve(&foods);

  let total_time = before.elapsed();
  println!("Parsing: elapsed time: {:.2?}", parsing_time);
  println!("Total elapsed time: {:.2?}", total_time);
  println!("Print elapsed time: {:.2?}", before.elapsed());
}

struct Food<'a> {
  ingredients: Vec<&'a str>,
  allergens: Vec<&'a str>,
}

fn parse_line(line: &str) -> Food {
  let mut line_split = line.split(" (contains ");
  Food {
    ingredients: line_split.next().unwrap().split(' ').collect(),
    allergens: line_split
      .next()
      .unwrap()
      .split(')')
      .next()
      .unwrap()
      .split(", ")
      .collect(),
  }
}

use std::collections::HashMap;
use std::collections::HashSet;

fn solve(foods: &Vec<Food>) -> () {
  let before_p1 = std::time::Instant::now();
  let allergen_to_ingredients =
    foods
      .iter()
      .fold(HashMap::<&str, HashSet<&str>>::new(), |mut acc, f| {
        for allergen in &f.allergens {
          let ingredients = f.ingredients.iter().cloned().collect::<HashSet<&str>>();
          acc.insert(
            allergen,
            ingredients
              .intersection(acc.get(allergen).unwrap_or(&ingredients))
              .cloned()
              .collect(),
          );
        }
        acc
      });
  let safe_ingredients =
    &get_all_ingredients(&foods) - &get_dangerous_ingredients(&allergen_to_ingredients);
  let p1_time = before_p1.elapsed();
  println!(
    "Part1: {:>4} | elapsed time: {:.2?}",
    foods
      .iter()
      .map(|f| {
        f.ingredients
          .iter()
          .filter(|ingredient| safe_ingredients.contains(*ingredient))
          .count()
      })
      .sum::<usize>(),
    p1_time
  );

  let before_p2 = std::time::Instant::now();
  let mut allergen_to_dangerous_ingredients = HashMap::new();
  for (k, _) in &allergen_to_ingredients {
    allergen_to_dangerous_ingredients.insert(
      *k,
      (&allergen_to_ingredients[k] - &safe_ingredients)
        .iter()
        .cloned()
        .collect(),
    );
  }
  let mut uniq_dangerous_ingredients = remove_duplicates(&mut allergen_to_dangerous_ingredients);
  uniq_dangerous_ingredients.sort_by(|a, b| a.cmp(b));
  let canonical_dangerous_ingredients =
    uniq_dangerous_ingredients
      .iter()
      .fold(String::new(), |r, i| {
        if r.is_empty() {
          format!("{}", i.1)
        } else {
          format!("{},{}", r, i.1)
        }
      });
  let p2_time = before_p2.elapsed();
  println!(
    "Part2: {:>4} | elapsed time: {:.2?}",
    canonical_dangerous_ingredients, p2_time,
  );
}

fn get_all_ingredients<'a>(foods: &Vec<Food<'a>>) -> HashSet<&'a str> {
  foods.iter().fold(HashSet::new(), |acc, f| {
    if acc.is_empty() {
      f.ingredients.iter().cloned().collect()
    } else {
      &f.ingredients.iter().cloned().collect::<HashSet<&str>>() | &acc
    }
  })
}

fn get_dangerous_ingredients<'a>(
  allergen_to_ingredients: &HashMap<&str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
  allergen_to_ingredients
    .iter()
    .fold(HashSet::new(), |acc, allergen| {
      if acc.is_empty() {
        allergen.1.clone()
      } else {
        allergen.1 | &acc
      }
    })
}

fn remove_duplicates<'a>(
  allergen_to_ingredients: &HashMap<&'a str, HashSet<&'a str>>,
) -> Vec<(&'a str, &'a str)> {
  let mut allergen_to_ingredients_clone = allergen_to_ingredients.clone();
  let mut found = Vec::new();
  loop {
    let mut to_remove = None;
    for (k, v) in &allergen_to_ingredients_clone {
      if v.len() == 1 && !found.contains(k) {
        to_remove = Some(v.iter().next().unwrap().clone());
        found.push(k);
        break;
      }
    }
    if to_remove.is_none() {
      break;
    }
    for (_, v) in &mut allergen_to_ingredients_clone {
      if v.len() != 1 && v.contains(to_remove.unwrap()) {
        v.remove(to_remove.unwrap());
      }
    }
  }
  allergen_to_ingredients_clone
    .iter()
    .map(|(a, i)| (*a, *i.iter().next().unwrap()))
    .collect::<Vec<(&str, &str)>>()
}
