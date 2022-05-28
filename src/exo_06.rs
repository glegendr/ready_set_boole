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
                (Operator::L(_) | Operator::Not, Operator::And) => {
                    let mut ret = BTree::new(c2.node.clone());
                    let a = modify_tree(c1);
                    let b = modify_tree(c2);
                    ret.insert_a(BTree::create(tree.node.clone(), a.clone(), *(b.c1.unwrap())));
                    ret.insert_b(BTree::create(tree.node.clone(), a, *(b.c2.unwrap())));

                    ret
                }
                (Operator::And, Operator::L(_) | Operator::Not) => {
                    let mut ret = BTree::new(c1.node.clone());
                    let a = modify_tree(c2);
                    let b = modify_tree(c1);
                    ret.insert_a(BTree::create(tree.node.clone(), a.clone(), *(b.c1.unwrap())));
                    ret.insert_b(BTree::create(tree.node.clone(), a, *(b.c2.unwrap())));

                    ret
                }
                // (Operator::Or | Operator::And, Operator::Or | Operator::Not) => {
                //     let mut ret = BTree::new(c1.node.clone());

                // }
                _ => BTree::create(Operator::Or, modify_tree(c1), modify_tree(c2))
            }
        }
        (_, Some(c1), Some(c2)) => BTree::create(
            tree.node.clone(),
            modify_tree(&Box::new(*c1.clone())),
            modify_tree(&Box::new(*c2.clone()))
        ),
        _ => *tree.clone()
    }

}

pub fn conjunctive_normal_form(formula: &str) -> String {
    match to_tree(&negation_normal_form(formula)) {
        Ok(tree) => modify_tree(&Box::new(tree)).to_cnfstring(),
        Err(e) => {
            println!("{e}");
            String::default()
        }
    }
}