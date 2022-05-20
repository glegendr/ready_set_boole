use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    And,
    Or,
    Not,
    Xor,
    Material,
    Equal,
    B(u8),
    L(char)
}
// XOR -> NOT OU
// AB|! -> A!B!&

impl Operator {
    fn operate(&mut self, b1: &Option<(usize, Operator)>, b2: &Option<(usize, Operator)>) -> Result<usize, String> {
        match (&b1, &b2) {
            (Some(b1), Some(b2)) => {
                match (self.clone(), &b1.1, &b2.1) {
                    (Operator::B(_), _, _) => return Ok(0),
                    (Operator::And, Operator::B(b1), Operator::B(b2)) => *self = Operator::B(b1 & b2),
                    (Operator::Or, Operator::B(b1), Operator::B(b2)) => *self = Operator::B(b1 | b2),
                    (Operator::Xor, Operator::B(b1), Operator::B(b2)) => *self = Operator::B(b1 ^ b2),
                    (Operator::Material, Operator::B(b1), Operator::B(b2)) => if *b2 == 1 && *b1 == 0 {
                        *self = Operator::B(0)
                    } else {
                        *self = Operator::B(1)
                    },
                    (Operator::Equal, Operator::B(b1), Operator::B(b2)) => if b1 == b2 {
                        *self = Operator::B(1)
                    } else {
                        *self = Operator::B(0)
                    },
                    (Operator::Not, Operator::B(b1), _) => {
                        *self = Operator::B(!b1 & 1);
                        return Ok(1)
                    }
                    _ => return Err(String::from("Error: Expected boolean found operator"))
                }
            },
            (Some(b1), None) => {
                match (self.clone(), &b1.1) {
                    (Operator::Not, Operator::B(b1)) => {
                        *self = Operator::B(!b1 & 1);
                        return Ok(1)
                    },
                    (Operator::Not, _) => return Err(String::from("Error: Expected boolean found operator")),
                    _ => return Ok(0),
                }
            },
            _ => return Ok(0)
        }
        return Ok(2)
    }
}

fn formula_to_map(formula: &str) -> Result<BTreeMap<usize, Operator>, String> {
    let mut map = BTreeMap::new();
    for (i, c) in formula.chars().enumerate() {
        match c {
            '&' => map.insert(i, Operator::And),
            '|' => map.insert(i, Operator::Or),
            '^' => map.insert(i, Operator::Xor),
            '>' => map.insert(i, Operator::Material),
            '=' => map.insert(i, Operator::Equal),
            '!' => map.insert(i, Operator::Not),
            '1' => map.insert(i, Operator::B(1)),
            '0' => map.insert(i, Operator::B(0)),
            c => return Err(format!("unknown token {c}"))
        };
    }
    Ok(map)
}

pub fn eval_formula(formula: &str) -> bool {
    match formula_to_map(formula) {
        Ok(mut map) => {
            let mut last = map.clone();
            loop {
                map = match map.iter_mut().fold(Ok((BTreeMap::new(), None, None)), |acc: Result<(BTreeMap<usize, Operator>, Option<(usize, Operator)>, Option<(usize, Operator)>), String>, (k, v)| {
                    match acc {
                        Ok((mut map_acc, mut b1, mut b2)) => {
                            match v.operate(&b1, &b2) {
                                Ok(1) => if let Some(loc_b1) = b1 {
                                    map_acc.remove(&loc_b1.0);
                                    b1 = Some((*k, v.clone()));
                                },
                                Ok(2) => match (&b1, &b2) {
                                    (Some(loc_b1), Some(loc_b2)) => {
                                        map_acc.remove(&loc_b1.0);
                                        map_acc.remove(&loc_b2.0);
                                        b1 = Some((*k, v.clone()));
                                        b2 = match &map_acc.iter().last() {
                                            Some((last_key, last_value)) => Some((**last_key, (**last_value).clone())),
                                            None => None,
                                        };
                                    },
                                    _ => ()
                                },
                                Ok(_) => {
                                    b2 = b1;
                                    b1 = Some((*k, v.clone()));
                                },
                                Err(e) => return Err(e)
                            }
                            map_acc.insert(*k, v.clone());
                            Ok((map_acc, b1, b2))
                        },
                        Err(e) => Err(e)
                    }
                }) {
                    Ok((map_acc, _b1, _b2)) => map_acc,
                    Err(e) => {
                        println!("{e}");
                        return false
                    }
                };
                if map.len() == 1 {
                    match map.iter().next() {
                        Some((_, Operator::B(b))) => return *b == 1,
                        _ => {
                            println!("unknown error 1");
                            return false
                        }
                    }
                }
                if last.len() == map.len() && last.keys().all(|k| map.contains_key(k)) {
                    println!("{:?}", map);
                    println!("unknown error 2");
                    return false
                }
                last = map.clone();
            };
        },
        Err(e) => println!("{e}")
    }
    false
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
 *                      A
 * DAC=B|&!         =
 *            |         C
 * ! -> &           B
 *            D
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

pub fn to_tree(formula: &str) -> BTree {
    let mut new_formula = String::from(formula);
    x(&mut new_formula)
}

fn x(formula: &mut String) -> BTree {
    if let Some(last_c) = formula.pop() {
        let mut ret = match last_c {
            '&' => BTree::new(Operator::And),
            '|' => BTree::new(Operator::Or),
            '^' => BTree::new(Operator::Xor),
            '>' => BTree::new(Operator::Material),
            '=' => BTree::new(Operator::Equal),
            '!' => BTree::new(Operator::Not),
            '1' => return BTree::new(Operator::B(1)),
            '0' => return BTree::new(Operator::B(0)),
            _ => return BTree::new(Operator::L(last_c))
        };
        ret.insert(x(formula));
        if last_c != '!' {
            ret.insert(x(formula));
        }
        return ret
    }
    unreachable!()
}