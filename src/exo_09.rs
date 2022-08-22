#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    And,
    Or,
    Not,
    Xor,
    Material,
    Equal,
    Set(Vec<i32>)
}

#[derive(Debug, Clone)]
pub struct BTree {
    pub c1: Option<Box<BTree>>,
    pub c2: Option<Box<BTree>>,
    pub node: Operator
}

impl BTree {
    pub fn new(node: Operator) -> BTree {
        BTree {
            c1: None,
            c2: None,
            node,
        }
    }

    pub fn insert_a(&mut self, sub_tree: BTree) {
        self.c1 = Some(Box::new(sub_tree));
    }

    pub fn insert_b(&mut self, sub_tree: BTree) {
        self.c2 = Some(Box::new(sub_tree));
    }
    
    pub fn to_string(&self) -> String {
        match (&self.node, &self.c1, &self.c2) {
            (Operator::And, Some(c1), Some(c2)) => format!("{}{}&", c1.to_string(), c2.to_string()),
            (Operator::Or, Some(c1), Some(c2)) => format!("{}{}|", c1.to_string(), c2.to_string()),
            (Operator::Xor, Some(c1), Some(c2)) => format!("{}{}^", c1.to_string(), c2.to_string()),
            (Operator::Equal, Some(c1), Some(c2)) => format!("{}{}=", c1.to_string(), c2.to_string()),
            (Operator::Material, Some(c1), Some(c2)) => format!("{}{}>", c1.to_string(), c2.to_string()),
            (Operator::Not, Some(c1), None) => format!("{}!", c1.to_string()),
            (Operator::Not, None, Some(c2)) => format!("{}!", c2.to_string()),
            (Operator::Set(s), None, None) => format!("{s:?}"),
            _ => format!("{self:?}")
        }
    }

    fn from_str(mut formula: String) -> Result<Self, String> {
        if let Some(last_c) = formula.pop() {
            let mut ret = match last_c {
                '&' => BTree::new(Operator::And),
                '|' => BTree::new(Operator::Or),
                '^' => BTree::new(Operator::Xor),
                '>' => BTree::new(Operator::Material),
                '=' => BTree::new(Operator::Equal),
                '!' => BTree::new(Operator::Not),
                'A'..='Z' => BTree::new(Operator::Set(vec![])),
                _ => Err(format!)
            };
            ret.insert_b(BTree::from_str(formula)?);
            if last_c != '!' {
                ret.insert_a(BTree::from_str(formula)?);
            }
            return Ok(ret)
        }
        Err(String::from("Error while parsing formula"))
    }
}



#[allow(dead_code)]
fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    match BTree::from_str(formula.to_owned()) {
        Ok(tree) => {
            println!("{}", tree.to_string());
        },
        Err(e) => {
            println!("{e}");
        }
    };
    Vec::new()
}

#[cfg(test)]
mod eval_set {

    use super::*;
    #[test]
    fn eval_set_test() {
        eval_set("AB&", vec![vec![1,2], vec![2,3]]);
        eval_set("A!", vec![vec![1,2], vec![2,3]]);
        eval_set("''&", vec![vec![1,2], vec![2,3]]);
        assert_eq!(1,2);
    }
}
