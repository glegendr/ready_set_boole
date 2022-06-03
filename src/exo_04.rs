use crate::exo_03::eval_formula;

fn print_titles(vars: &str) -> String {
    let mut ret = String::from("|");
    for var in vars.chars() {
        ret.push_str(&format!(" {var} |"));
    }
    ret.push_str(" = |\n|");
    for _var in vars.chars() {
        ret.push_str("---|");
    }
    ret.push_str("---|\n");
    ret
}

fn print_results(results: Vec<(u32, bool)>, vars_len: usize) -> String {
    let mut ret = String::default();
    for result in results {
        for i in 0..vars_len {
            ret.push_str(&format!("| {} ", (result.0 >> i) & 1));
        }
        ret.push_str(&format!("| {} |\n", if result.1 { 1 } else { 0 }));
    }
    ret
}


pub fn print_truth_table(formula: &str) {
    println!("{}", calc_truth_table(formula))
}

pub fn calc_truth_table(formula: &str) -> String {
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
        return String::default()
    }
    let mut chars = vars.chars().collect::<Vec<char>>();
    chars.sort_by(|a, b| a.cmp(b));
    vars = String::from_iter(chars);
    let permutations: Vec<u32> = (0..=u32::MAX >> (32 - vars.len())).collect();
    let mut results: Vec<(u32, bool)> = Vec::new();
    let mut permutation_formula = String::with_capacity(formula.len());
    for permutation in &permutations {
        for c in formula.chars() {
            match c {
                'A'..='Z' => {
                    if let Some(i) = vars.find(c) {
                        permutation_formula.push(((permutation >> i) & 1).to_string().chars().next().unwrap());
                    } else {
                        println!("Unexpected variable {c}");
                        return String::default()
                    }
                },
                _ => permutation_formula.push(c)
            }
        }
        results.push((*permutation, eval_formula(&permutation_formula)));
        permutation_formula.clear();
    }
    format!("{}{}", print_titles(&vars), print_results(results, vars.len()))
}