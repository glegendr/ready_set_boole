use crate::exo_03::eval_formula;

pub fn sat(formula: &str) -> bool {
    let mut vars = String::new();

    for c in formula.chars() {
        if !vars.contains(&c.to_string()) {
            match c {
                'A'..='Z' => vars.push(c),
                _ => (),
            }
        }
    }

    if vars.len() == 0 {
        return false
    }
    let mut chars = vars.chars().collect::<Vec<char>>();
    chars.sort_by(|a, b| a.cmp(b));
    vars = String::from_iter(chars);
    let permutations: Vec<u32> = (0..=u32::MAX >> (32 - vars.len())).collect();
    let mut permutation_formula = String::with_capacity(formula.len());
    for permutation in &permutations {
        for c in formula.chars() {
            match c {
                'A'..='Z' => {
                    if let Some(i) = vars.find(c) {
                        permutation_formula.push(((permutation >> i) & 1).to_string().chars().next().unwrap());
                    } else {
                        println!("Unexpected variable {c}");
                        return false
                    }
                },
                _ => permutation_formula.push(c)
            }
        }
        if eval_formula(&permutation_formula) {
            return true
        }
        permutation_formula.clear();
    }
    false
}