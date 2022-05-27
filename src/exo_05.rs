use crate::exo_03::{to_tree, BTree, Operator};

fn insert_not(tree: BTree) -> BTree {
    modify_tree(&Box::new(BTree::not(tree)))
}

fn modify_tree(tree: &Box<BTree>) -> BTree {
    match (&tree.node, &tree.c1, &tree.c2) {
        (Operator::Material, Some(c1), Some(c2)) => {
            let a = modify_tree(&c1);
            let b = modify_tree(&c2);
            BTree::create(Operator::Or, insert_not(a), b)
        }
        (Operator::Xor, Some(c1), Some(c2)) => {
            // AB!&A!B&|
            let mut ret = BTree::new(Operator::Or);
            let a = modify_tree(&c1);
            let b = modify_tree(&c2);
            //AB!& C1
            ret.insert_a(BTree::create(Operator::And, a.clone(), insert_not(b.clone())));
            //A!B& C2
            ret.insert_b(BTree::create(Operator::And, insert_not(a.clone()), b.clone()));
            ret
        }
        (Operator::Equal, Some(c1), Some(c2)) => {
            let mut ret = BTree::new(Operator::Or);
            let a = modify_tree(&c1);
            let b = modify_tree(&c2);
            //AB& C1
            let mut new_c1 = BTree::new(Operator::And);
            new_c1.insert_a(a.clone());
            new_c1.insert_b(b.clone());
            ret.insert_a(new_c1);

            //A!B!& C2
            let mut new_c2 = BTree::new(Operator::And);
            new_c2.insert_a(insert_not(a));
            new_c2.insert_b(insert_not(b));
            ret.insert_b(new_c2);
            ret
        }
        (Operator::Not, None, Some(c2)) => match &c2.node {
            Operator::B(_) => *tree.clone(),
            Operator::L(_) => *tree.clone(),
            _ => {
                let a = modify_tree(&c2);
                match &a.node {
                    Operator::And => {
                        let mut ret = BTree::new(Operator::Or);
                        let mut c2_1 = BTree::new(Operator::Not);
                        c2_1.insert_b(*a.c2.unwrap());
                        ret.insert_b(modify_tree(&Box::new(c2_1)));
                        let mut c1_1 = BTree::new(Operator::Not);
                        c1_1.insert_b(*a.c1.unwrap());
                        ret.insert_a(modify_tree(&Box::new(c1_1)));
                        ret
                    }
                    Operator::Or => {
                        let mut ret = BTree::new(Operator::And);
                        let mut c2_1 = BTree::new(Operator::Not);
                        c2_1.insert_b(*a.c2.unwrap());
                        ret.insert_b(modify_tree(&Box::new(c2_1)));
                        let mut c1_1 = BTree::new(Operator::Not);
                        c1_1.insert_b(*a.c1.unwrap());
                        ret.insert_a(modify_tree(&Box::new(c1_1)));
                        ret
                    }
                    Operator::Not => (*a.c2.unwrap()).clone(),
                    _ => *tree.clone(),
                }
            }
        },
        (Operator::L(_l), None, None) => *tree.clone(),
        (Operator::B(_b), None, None) => *tree.clone(),
        (_, Some(c1), Some(c2)) => {
            let mut ret = BTree::new(tree.node.clone());
            ret.insert_a(modify_tree(&c1));
            ret.insert_b(modify_tree(&c2));
            ret
        }
        _ => unreachable!(),
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    match to_tree(formula) {
        Ok(tree) => modify_tree(&Box::new(tree)).to_string(),
        Err(e) => {
            println!("{e}");
            String::default()
        }
    }
}
