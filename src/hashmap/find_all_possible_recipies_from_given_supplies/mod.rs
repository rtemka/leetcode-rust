use std::collections::HashSet;

// https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/description
struct Solution;

impl Solution {
    pub fn find_all_recipes(
        mut recipes: Vec<String>,
        mut ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut res = Vec::<String>::new();
        let mut supplies: HashSet<String> = supplies.into_iter().collect();
        let mut have_progress = true;

        while have_progress {
            have_progress = false;

            let mut i = 0;
            while i < recipes.len() {
                if ingredients[i].iter().all(|ingr| supplies.contains(ingr)) {
                    have_progress = true;
                    ingredients.swap_remove(i);
                    let recipe = recipes.swap_remove(i);
                    supplies.insert(recipe.clone());
                    res.push(recipe);
                    continue;
                }
                i += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all_possible_recipes_from_given_supplies() {
        assert_eq!(
            vec!["bread"],
            Solution::find_all_recipes(
                vec!["bread".to_string()],
                vec![vec!["yeast".to_string(), "flour".to_string()]],
                vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()]
            )
        );

        assert_eq!(
            vec!["bread"],
            Solution::find_all_recipes(
                vec!["bread".to_string(), "omlet".to_string()],
                vec![
                    vec!["yeast".to_string(), "flour".to_string()],
                    vec!["eggs".to_string(), "milk".to_string()],
                ],
                vec!["yeast".to_string(), "flour".to_string(), "eggs".to_string()]
            )
        );

        assert_eq!(
            vec!["bread", "sandwich", "burger"],
            Solution::find_all_recipes(
                vec![
                    "bread".to_string(),
                    "sandwich".to_string(),
                    "burger".to_string()
                ],
                vec![
                    vec!["yeast".to_string(), "flour".to_string()],
                    vec!["bread".to_string(), "meat".to_string()],
                    vec![
                        "sandwich".to_string(),
                        "meat".to_string(),
                        "bread".to_string()
                    ],
                ],
                vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()]
            )
        );
    }
}
