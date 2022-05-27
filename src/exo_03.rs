#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    And,
    Or,
    Not,
    Xor,
    Material,
    Equal,
    B(u8),
    L(char)
}

/*
 * AB|C&!
 * 
 *      !
 *     / 
 *    &
 *   / \
 *  C   |
 *     / \
 *    A   B
 * 
 * 
 * DAC=B|&!
 * 
 *    !
 *    |
 *    &
 *   / \
 *  D   |
 *     / \
 *    B   =
 *       / \
 *      A   C
*/

#[derive(Debug)]
pub struct BTree {
    c1: Option<Box<BTree>>,
    c2: Option<Box<BTree>>,
    node: Operator
}

impl BTree {
    fn new(node: Operator) -> BTree {
        BTree {
            c1: None,
            c2: None,
            node,
        }
    }

    fn insert(&mut self, sub_tree: BTree) {
        match (&self.c1, &self.c2) {
            (None, _) => self.c1 = Some(Box::new(sub_tree)),
            (_, None) => self.c2 = Some(Box::new(sub_tree)),
            _ => panic!("No left space on node")
        }
    }
}


pub fn eval_formula(formula: &str) -> bool {
    match to_tree(formula) {
        Ok(tree) => {
            match calc_formula(&Box::new(tree)) {
                Ok(res) => res,
                Err(e) => {
                    println!("{e}");
                    false
                }
            }
        },
        Err(e) => {
            println!("{e}");
            false
        }
    }
}

pub fn to_tree(formula: &str) -> Result<BTree, String> {
    let mut new_formula = String::from(formula);
    create_tree(&mut new_formula)
}

fn create_tree(formula: &mut String) -> Result<BTree, String> {
    if let Some(last_c) = formula.pop() {
        let mut ret = match last_c {
            '&' => BTree::new(Operator::And),
            '|' => BTree::new(Operator::Or),
            '^' => BTree::new(Operator::Xor),
            '>' => BTree::new(Operator::Material),
            '=' => BTree::new(Operator::Equal),
            '!' => BTree::new(Operator::Not),
            '1' => return Ok(BTree::new(Operator::B(1))),
            '0' => return Ok(BTree::new(Operator::B(0))),
            _ => return Ok(BTree::new(Operator::L(last_c)))
        };
        ret.insert(create_tree(formula)?);
        if last_c != '!' {
            ret.insert(create_tree(formula)?);
        }
        return Ok(ret)
    }
    Err(String::from("Error while parsing formula"))
}

fn calc_formula(tree: &Box<BTree>) -> Result<bool, String> {
    match (&tree.node, &tree.c1, &tree.c2) {
        (Operator::And, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? & calc_formula(&c2)?),
        (Operator::Or, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? | calc_formula(&c2)?),
        (Operator::Xor, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? ^ calc_formula(&c2)?),
        (Operator::Material, Some(c1), Some(c2)) => Ok(!(!calc_formula(&c1)? && calc_formula(&c2)?)),
        (Operator::Equal, Some(c1), Some(c2)) => Ok(calc_formula(&c1)? == calc_formula(&c2)?),
        (Operator::Not, Some(c1), None) => Ok(!calc_formula(&c1)?),
        (Operator::Not, None, Some(c2)) => Ok(!calc_formula(&c2)?),
        (Operator::B(b), None, None) => Ok(*b == 1),
        _ => {
            return Err(String::from("Error while calculating formula"))
        }
    }
}