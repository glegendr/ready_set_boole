use crate::exo_04::eval_formula;

fn print_titles(vars: &str) {
    print!("|");
    for var in vars.chars() {
        print!(" {} |", var);
    }
    print!(" = |\n|");
    for var in vars.chars() {
        print!("---|");
    }
    print!("---|\n");
}

fn print_results(results: Vec<(u32, bool)>, vars_len: usize) {
    for result in results {
        for i in 0..vars_len {
            print!("| {} ", (result.0 >> i) & 1);
        }
        println!("| {} |", if result.1 {
            1
        } else {
            0
        });
    }
}

pub fn print_truth_table(formula: &str) {
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
        return
    }
    let permutations: Vec<u32> = (0..=u32::MAX >> (32 - vars.len())).collect();
    let mut results: Vec<(u32, bool)> = Vec::new();
    for permutation in &permutations {
        let mut permutation_formula = String::from(formula);
        for (i, var) in vars.chars().enumerate() {
            permutation_formula = permutation_formula.replace(var, &((permutation >> i) & 1).to_string());
        }
        results.push((*permutation, eval_formula(&permutation_formula)));
    }
    print_titles(&vars);
    print_results(results, vars.len());
}