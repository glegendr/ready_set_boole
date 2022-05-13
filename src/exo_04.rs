/* MATHEMATICS */
fn and_op(a: char, b: char) -> Result<i32, String> {
    match (a.to_string().parse::<i32>(), b.to_string().parse::<i32>()) {
        (Ok(a), Ok(b)) => if (a & b) == 1 {
            Ok(1)
        } else {
            Ok(0)
        },
        _ => Err(String::from("Parsing Error"))
    }
}

fn or_op(a: char, b: char) -> Result<i32, String> {
    match (a.to_string().parse::<i32>(), b.to_string().parse::<i32>()) {
        (Ok(a), Ok(b)) => if (a | b) == 1 {
            Ok(1)
        } else {
            Ok(0)
        },
        _ => Err(String::from("Parsing Error"))
    }
}

fn xor_op(a: char, b: char) -> Result<i32, String> {
    match (a.to_string().parse::<i32>(), b.to_string().parse::<i32>()) {
        (Ok(a), Ok(b)) => if (a ^ b) == 1 {
            Ok(1)
        } else {
            Ok(0)
        },
        _ => Err(String::from("Parsing Error"))
    }
}

fn material_op(a: char, b: char) -> Result<i32, String> {
    match (a.to_string().parse::<i32>(), b.to_string().parse::<i32>()) {
        (Ok(a), Ok(b)) => if !(b == 0 && a == 1) {
            Ok(1)
        } else {
            Ok(0)
        },
        _ => Err(String::from("Parsing Error"))
    }
}

fn equal_op(a: char, b: char) -> Result<i32, String> {
    match (a.to_string().parse::<i32>(), b.to_string().parse::<i32>()) {
        (Ok(a), Ok(b)) => if a == b {
            Ok(1)
        } else {
            Ok(0)
        },
        _ => Err(String::from("Parsing Error"))
    }
}

/* PARSER */
fn is_op(c: char) -> bool {
    match c {
        '&' | '|' | '^' | '>' | '=' => true,
        _ => false
    }
}

fn is_nb(c: char) -> bool {
    c == '0' || c == '1'
}

fn check(formula: &str) -> bool {
    match (formula.chars().nth(0), formula.chars().nth(1), formula.chars().nth(2)) {
        (Some(c1), Some(c2), Some(c3)) => (is_nb(c1) && is_nb(c2) && is_op(c3)) || (is_nb(c1) && c2 == '!'),
        (Some(c1), Some(c2), _) => is_nb(c1) && c2 == '!',
        _ => false
    }
}

/* OPERATIONS */

fn operate(formula: String) -> Result<Option<String>, String> {
    if check(&formula) {
        if formula.chars().nth(1).unwrap() == '!' {
            match formula.chars().nth(0).unwrap() {
                '0' => return Ok(Some(format!("1{}", &formula[2..]))),
                '1' => return Ok(Some(format!("0{}", &formula[2..]))),
                c => return Err(format!("Unknown token {}", c))
            }
        }
        match formula.chars().nth(2).unwrap() {
            '&' => Ok(Some(format!("{}{}", and_op(formula.chars().nth(0).unwrap(), formula.chars().nth(1).unwrap())?, &formula[3..]))),
            '|' => Ok(Some(format!("{}{}", or_op(formula.chars().nth(0).unwrap(), formula.chars().nth(1).unwrap())?, &formula[3..]))),
            '^' => Ok(Some(format!("{}{}", xor_op(formula.chars().nth(0).unwrap(), formula.chars().nth(1).unwrap())?, &formula[3..]))),
            '>' => Ok(Some(format!("{}{}", material_op(formula.chars().nth(0).unwrap(), formula.chars().nth(1).unwrap())?, &formula[3..]))),
            '=' => Ok(Some(format!("{}{}", equal_op(formula.chars().nth(0).unwrap(), formula.chars().nth(1).unwrap())?, &formula[3..]))),
            c => Err(format!("Unknown token {}", c)) 
        }
    } else {
        Ok(None)
    }
}

pub fn eval_formula(formula: &str) -> bool {
    let mut intern_formula = String::from(formula);
    let mut i = 0;
    while i < intern_formula.len() {
        match operate((&intern_formula[i..]).to_string()) {
            Ok(Some(new_formula)) => {
                intern_formula = format!("{}{}", &intern_formula[..i], new_formula);
                i = 0;
            },
            Ok(None) => i = i + 1,
            Err(e) => {
                println!("Err: {}", e);
                return false
            }
        }
    }
    if intern_formula == "0" {
        false
    } else if intern_formula == "1" {
        true
    } else {
        println!("Err: {}", intern_formula);
        return false
    }
}


// 123  +-
// (1)(23)  (-)(+)

// 1011 =||

// 1011||=
// 11|
// 01|
// 11=1

// 110|& == 10|1&

// 110 &| != 101 &|
/*
 * 
 * 10|1&
 * 
 * 3*(1+2) - 1
 *  12+3*1-
 *  1312+*-
 *  
 * 3*(1+2) - 1*(8-9)
 * 
 * 89-1*12+3*-
 * 
 * 312+*189-*-
 * ->
 * 123+-
 */