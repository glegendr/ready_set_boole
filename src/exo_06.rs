use crate::exo_03::{to_tree, BTree, Operator};
use crate::exo_05::negation_normal_form;


fn is_and_or(op: Operator) -> bool {
    match op {
        Operator::Or | Operator::And => true,
        _ => false
    }
}

fn modify_tree(tree: &Box<BTree>) -> BTree {
    match (&tree.node, &tree.c1, &tree.c2) {
        (Operator::Or, Some(c1), Some(c2)) => {
            match (&c1.node, &c2.node) {
                (Operator::L(_) | Operator::Not, Operator::Or | Operator::And) => {
                    let mut ret = BTree::new(c2.node.clone());
                    ret.insert_a(BTree::create(tree.node.clone(), *c1.clone(), *(c2.c1.clone().unwrap())));
                    ret.insert_b(BTree::create(tree.node.clone(), *c1.clone(), *(c2.c2.clone().unwrap())));

                    ret
                }
                (Operator::Or | Operator::And, Operator::L(_) | Operator::Not) => {
                    let mut ret = BTree::new(c1.node.clone());
                    ret.insert_a(BTree::create(tree.node.clone(), *c2.clone(), *(c1.c1.clone().unwrap())));
                    ret.insert_b(BTree::create(tree.node.clone(), *c2.clone(), *(c1.c2.clone().unwrap())));

                    ret
                }
                (Operator::Or | Operator::And, Operator::Or | Operator::Not) => {
                    let mut ret = BTree::new(c1.node.clone());

                }
                _ => *tree.clone()
            }
        }
        _ => *tree.clone()
    }

}

pub fn conjunctive_normal_form(formula: &str) -> String {
    match to_tree(&negation_normal_form(formula)) {
        Ok(tree) => modify_tree(&Box::new(tree)).to_string(),
        Err(e) => {
            println!("{e}");
            String::default()
        }
    }
}