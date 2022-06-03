use crate::exo_03::{to_tree, BTree, Operator};
use crate::exo_05::negation_normal_form;

fn modify_tree(tree: &Box<BTree>) -> BTree {
    match (&tree.node, &tree.c1, &tree.c2) {
        (Operator::Or, Some(c1), Some(c2)) => {
            match (&c1.node, &c2.node) {
                (_, Operator::And) => {
                    let mut ret = BTree::new(c2.node.clone());
                    let a = modify_tree(c1);
                    let b = modify_tree(c2);
                    ret.insert_a(BTree::create(tree.node.clone(), a.clone(), *(b.c1.unwrap())));
                    ret.insert_b(BTree::create(tree.node.clone(), a, *(b.c2.unwrap())));

                    modify_tree(&Box::new(ret))
                }
                (Operator::And, _) => {
                    let mut ret = BTree::new(c1.node.clone());
                    let a = modify_tree(c2);
                    let b = modify_tree(c1);
                    ret.insert_a(BTree::create(tree.node.clone(), a.clone(), *(b.c1.unwrap())));
                    ret.insert_b(BTree::create(tree.node.clone(), a, *(b.c2.unwrap())));

                    modify_tree(&Box::new(ret))
                }
                _ => {
                    let a = modify_tree(c1);
                    let b = modify_tree(c2);
                    if a.node == Operator::And || b.node == Operator::And {
                        return modify_tree(&Box::new(BTree::create(Operator::Or, a, b)))
                    }
                    BTree::create(Operator::Or, a, b)
                }
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