#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    And,
    Or,
    Not,
    Xor,
    Material,
    Equal,
    Set((Vec<i32>, bool))
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

    pub fn not(sub_tree: BTree) -> BTree {
        BTree {
            c1: None,
            c2: Some(Box::new(sub_tree)),
            node: Operator::Not
        }
    }

    pub fn create(node: Operator, c1: BTree, c2: BTree) -> BTree {
        BTree {
            node,
            c1: Some(Box::new(c1)),
            c2: Some(Box::new(c2))
        }
    }

    pub fn insert_a(&mut self, sub_tree: BTree) {
        self.c1 = Some(Box::new(sub_tree));
    }

    pub fn insert_b(&mut self, sub_tree: BTree) {
        self.c2 = Some(Box::new(sub_tree));
    }

    fn insert_not(&self) -> Self {
        let mut new = BTree::not((*self).clone());
        new.modify();
        new
    }

    fn modify(&mut self) {
        match (&self.node, &mut  self.c1, &mut self.c2) {
            (Operator::Material, Some(c1), Some(c2)) => {
                c1.modify();
                c2.modify();
                *self = BTree::create(Operator::Or, c1.insert_not(), (**c2).clone())
            }
            (Operator::Xor, Some(c1), Some(c2)) => {
                // AB!&A!B&|
                let mut ret = BTree::new(Operator::Or);
                c1.modify();
                c2.modify();
                //AB!& C1
                ret.insert_a(BTree::create(Operator::And, (**c1).clone(), c2.insert_not()));
                //A!B& C2
                ret.insert_b(BTree::create(Operator::And, c1.insert_not(), (**c2).clone()));
                *self = ret;
            }
            (Operator::Equal, Some(c1), Some(c2)) => {
                let mut ret = BTree::new(Operator::Or);
                c1.modify();
                c2.modify();
                // let a = modify_tree(&c1);
                // let b = modify_tree(&c2);
                //AB& C1
                let mut new_c1 = BTree::new(Operator::And);
                new_c1.insert_a((**c1).clone());
                new_c1.insert_b((**c2).clone());
                ret.insert_a(new_c1);
    
                //A!B!& C2
                let mut new_c2 = BTree::new(Operator::And);
                new_c2.insert_a(c1.insert_not());
                new_c2.insert_b(c2.insert_not());
                ret.insert_b(new_c2);
                *self = ret;
            }
            (Operator::Not, None, Some(c2)) => match &c2.node {
                Operator::Set(_) => {},
                _ => {
                    c2.modify();
                    match &c2.node {
                        Operator::And => *self = BTree::create(
                            Operator::Or,
                            c2.c1.as_ref().unwrap().insert_not(),
                            c2.c2.as_ref().unwrap().insert_not(),
                        ),
                        Operator::Or => *self = BTree::create(
                            Operator::And,
                            c2.c1.as_ref().unwrap().insert_not(),
                            c2.c2.as_ref().unwrap().insert_not(),
                        ),
                        Operator::Not => *self = (**c2.c2.as_ref().unwrap()).clone(),
                        _ => {},
                    }
                }
            },
            (Operator::Set(_), None, None) => {},
            (_, Some(c1), Some(c2)) => {
                let mut ret = BTree::new(self.node.clone());
                c1.modify();
                c2.modify();
                ret.insert_a((**c1).clone());
                ret.insert_b((**c2).clone());
                *self = ret
            }
            _ => unreachable!(),
        }
    }

    fn from_str(formula: &mut String, sets: &Vec<Vec<i32>>) -> Result<Self, String> {
        if let Some(last_c) = formula.pop() {
            let mut ret = match last_c {
                '&' => BTree::new(Operator::And),
                '|' => BTree::new(Operator::Or),
                '^' => BTree::new(Operator::Xor),
                '>' => BTree::new(Operator::Material),
                '=' => BTree::new(Operator::Equal),
                '!' => BTree::new(Operator::Not),
                'A'..='Z' => {
                    match sets.get((last_c as u8 - 65) as usize) {
                        Some(set) => return Ok(BTree::new(Operator::Set((set.clone(), true)))),
                        None => return Err(format!("no existing set for {last_c}"))
                    }
                    
                },
                c => return Err(format!("unexpected variable {c}"))
            };
            ret.insert_b(BTree::from_str(formula, sets)?);
            if last_c != '!' {
                ret.insert_a(BTree::from_str(formula, sets)?);
            }
            return Ok(ret)
        }
        Err(String::from("Error while parsing formula"))
    }

    fn eval(&self) -> Result<(Vec<i32>, bool), String> {
        match (&self.node, &self.c1, &self.c2) {
            (Operator::And, Some(c1), Some(c2)) => {
                let ret = intersect(c1.eval()?, c2.eval()?);
                Ok(ret)
            },
            (Operator::Or, Some(c1), Some(c2)) => {
                let (mut a, a_b) = c1.eval()?;
                let (mut b, b_b) = c2.eval()?;
                match (a_b, b_b) {
                    (true, true) => {
                        a.append(&mut b);
                        a.sort_unstable();
                        a.dedup();
                        Ok((a, true))
                    }
                    (true, false) => {
                        let mut ret = Vec::new();
                        for child in b {
                            if !a.contains(&child) {
                                ret.push(child)
                            }    
                        }
                        Ok((ret, false))
                    }
                    (false, true) => {
                        let mut ret = Vec::new();
                        for child in a {
                            if !b.contains(&child) {
                                ret.push(child)
                            }    
                        }
                        Ok((ret, false))
                    }
                    (false, false) => {
                        let (ret, _) = intersect((a, true), (b, true));
                        Ok((ret, false))
                    }
                }
            },
            (Operator::Not, Some(c), None) | (Operator::Not, None, Some(c))=> {
                let (set, b) = c.eval()?;
                Ok((set, !b))
            },
            (Operator::Set(set), None, None) => Ok(set.clone()),
            _ => return Err(String::from("Error while calculating formula")),
        }
    }
}

fn intersect((mut a, bool_a): (Vec<i32>, bool), (mut b, bool_b): (Vec<i32>, bool)) -> (Vec<i32>, bool) {
    let mut ret = Vec::new();
    match (bool_a, bool_b) {
        (true, true) => {
            for child in a {
                if b.contains(&child) {
                    ret.push(child);
                }
            }
            (ret, true)
        }
        (true, false) => {
            for child in a {
                if !b.contains(&child) {
                    ret.push(child);
                }
            }
            (ret, true)
        }
        (false, true) => {
            for child in b {
                if !a.contains(&child) {
                    ret.push(child);
                }
            }
            (ret, true)
        }
        (false, false) => {
            a.append(&mut b);
            (a, false)
        }
    }
}

#[allow(dead_code)]
fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    match BTree::from_str(&mut formula.to_owned(), &sets) {
        Ok(mut tree) => {
            tree.modify();
            match tree.eval() {
                Ok((res, b)) => {
                    return match b {
                        true => res,
                        false => Vec::new()
                    }
                },
                Err(e) => println!("{e}")
            }
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
        assert_eq!(eval_set("A!", vec![vec![1,2]]), vec![]);
        assert_eq!(eval_set("AB&", vec![vec![1,2], vec![2,3]]), vec![2]);
        assert_eq!(eval_set("AB^", vec![vec![1,2], vec![2,3]]), vec![1,3]);
        assert_eq!(eval_set("AB|", vec![vec![1,2], vec![2,3]]), vec![1,2,3]);
    }
}
