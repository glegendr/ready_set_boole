use crate::exo_06::conjunctive_normal_form;

pub fn sat(formula: &str) -> bool {
    let new_formula = conjunctive_normal_form(formula);
    true
}