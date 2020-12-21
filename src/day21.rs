use std::collections::{HashMap, HashSet};

type Input = String;
type Output = String;

register!(
    "input/day21.txt";
    run(input: Input) -> Output {
        run(input)
    }
);

fn run(input: Vec<Input>) -> (Output, Output) {
    let mut all_ingredients = Vec::<&str>::new();
    let mut possible = HashMap::<_, Vec<_>>::new();

    for food in &input {
        let mut parts = food.split(" (contains ");

        let ingredients = parts.next().unwrap();
        let ingredients = ingredients.split(' ').collect::<HashSet<_>>();
        all_ingredients.extend(ingredients.iter());

        let allergenes = parts.next().unwrap().trim_end_matches(')');
        for allergen in allergenes.split(", ") {
            possible
                .entry(allergen)
                .or_default()
                .push(ingredients.clone());
        }
    }

    let mut confirmed = HashMap::new();
    let mut added = Vec::new();
    while !possible.is_empty() {
        for (allergen, ingredients) in &possible {
            let mut possible = ingredients
                .iter()
                .fold(None::<HashSet<&str>>, |intersection, b| {
                    Some(intersection.map_or_else(|| b.clone(), |a| &a & b))
                })
                .unwrap_or_default();

            if possible.len() > 1 {
                possible = &possible - &confirmed.values().copied().collect();
            }

            if possible.len() == 1 {
                added.push(*allergen);
                confirmed.insert(*allergen, possible.into_iter().next().unwrap());
            }
        }

        for added in added.drain(..) {
            let _ = possible.remove(&added);
        }
    }

    let allergenic_ingredients = confirmed.values().copied().collect::<HashSet<_>>();
    let pt1 = all_ingredients
        .iter()
        .filter(|x| !allergenic_ingredients.contains(*x))
        .count();

    let mut ingredients = confirmed.into_iter().collect::<Vec<_>>();
    ingredients.sort_unstable_by_key(|(allergen, _)| *allergen);
    let ingredients = ingredients
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .collect::<Vec<_>>();

    (format!("{}", pt1), ingredients.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2020::Solution;

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1.as_str(), "2170");
        assert_eq!(
            res2.as_str(),
            "nfnfk,nbgklf,clvr,fttbhdr,qjxxpr,hdsm,sjhds,xchzh"
        );
    }

    #[test]
    fn test_pt1() {
        assert_eq!(
            (String::from("5"), String::from("mxmxvkd,sqjhc,fvjkl")),
            Solver::run_on(
                "
                mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
                trh fvjkl sbzzf mxmxvkd (contains dairy)
                sqjhc fvjkl (contains soy)
                sqjhc mxmxvkd sbzzf (contains fish)
            ",
            )
        );
    }
}
