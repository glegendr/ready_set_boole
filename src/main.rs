mod exo_00;
mod exo_01;
mod exo_02;
mod exo_03;
mod exo_04;
mod exo_05;
mod exo_06;
use exo_00::adder;
use exo_01::multiplier;
use exo_02::gray_code;
use exo_03::{eval_formula, to_tree};
use exo_04::print_truth_table;
use exo_05::negation_normal_form;
use exo_06::conjunctive_normal_form;
use colored::Colorize;

fn check(assert: bool, title: &str) {
    let is_ok = if assert {
        "✓".green()
    } else {
        "x".red()
    };
    println!("[{}] {}", is_ok, title)
}

fn check_string(expected: &str, founded: &str, title: &str) {
    if expected == founded {
        println!("[{}] {}", "✓".green(), title)
    } else {
        println!("[{}] expected: {}  founded: {}", "x".red(), expected, founded)
    };

}

fn main() {
    println!("========== Adder ==========");
    check(2 == adder(1, 1), "1 + 1");
    check(0 == adder(0, 0), "0 + 0");
    check(129 == adder(55, 74), "55 + 74");
    check(255 == adder(205, 50), "205 + 50");
    check(4294967295 == adder(876498323, 3418468972), "876498323 + 3418468972");
    println!("======== Multiplier =======");
    check(1 == multiplier(1, 1), "1 x 1");
    check(0 == multiplier(0, 0), "0 x 0");
    check(0 == multiplier(13, 0), "13 x 0");
    check(4070 == multiplier(55, 74), "55 x 74");
    check(10250 == multiplier(205, 50), "205 x 50");
    check(1139447699 == multiplier(87649823, 13), "87649823 x 13");
    println!("======== Gray Code ========");
    check(0 == gray_code(0), "0");
    check(1 == gray_code(1), "1");
    check(3 == gray_code(2), "2");
    check(2 == gray_code(3), "3");
    check(6 == gray_code(4), "4");
    check(7 == gray_code(5), "5");
    check(5 == gray_code(6), "6");
    check(4 == gray_code(7), "7");
    check(12 == gray_code(8), "8");
    check(7019 == gray_code(4685), "4685");
    println!("======== Boolean evaluation ========");
    check(!eval_formula("10&"), "10&");
    check(eval_formula("1!1|"), "1!1|");
    check(eval_formula("00>"), "00>");
    check(!eval_formula("10>"), "10>");
    check(eval_formula("01>"), "01>");
    check(eval_formula("11>"), "11>");
    check(!eval_formula("10="), "10=");
    check(eval_formula("1011||="), "1011||=");
    check(eval_formula("101&^011|^|"), "101&^011|^|");
    check(eval_formula("1!01&^011|^|"), "1!01&^011|^|");
    check(!eval_formula("1011|b|="), "1011|b|=");
    check(!eval_formula("1011||^="), "1011||^=");
    check(!eval_formula(""), "");
    println!("======== Truth Table ========");
    // print_truth_table("ABA&^BCA|^|");
    println!("---- AB&C| ---");
    print_truth_table("AB&C|");
    // print_truth_table("AB!C^DE|FGHIJKLM&=|=&|^^^=");
    // println!("\n--- AB!C^DE|FGHIJKLMNOPQZSTU&=|=&|^^^=|=^>>&=^ ---");
    // print_truth_table("AB!C^DE|FGHIJKLMNOPQZSTU&=|=&|^^^=|=^>>&=^");
    // print_truth_table("ABA&^BCA|^|")
    // print_truth_table("ABA&^BCA|^|")
    println!("======== Negation Normal Form ========");
    println!("{}", negation_normal_form("AB="));
    println!("{}", negation_normal_form("AB=!"));
    println!("{}", negation_normal_form("AB>"));
    println!("{}", negation_normal_form("CD^"));
    println!("{}", negation_normal_form("AB=CD^>"));
    println!("{}", negation_normal_form("AB=!!"));
    println!("======== Conjonctive Normal Form ========");
    // println!("{}", negation_normal_form("AB&C&D&"));
    // print_truth_table("AB&C&");
    // print_truth_table("ABC&&");
    check_string("A!B!|", &conjunctive_normal_form("AB&!"), "AB&!");
    check_string("A!B!&", &conjunctive_normal_form("AB|!"), "AB|!");
    check_string("AB|C&", &conjunctive_normal_form("AB|C&"), "AB|C&");
    check_string("ABCD|||", &conjunctive_normal_form("AB|C|D|"), "AB|C|D|");
    check_string("ABCD&&&", &conjunctive_normal_form("AB&C&D&"), "AB&C&D&");
    check_string("A!B!C!||", &conjunctive_normal_form("AB&!C!|"), "AB&!C!|");
    check_string("A!B!C!&&", &conjunctive_normal_form("AB|!C!&"), "AB|!C!&");
    /*
     *     &
     *   |   A
     *  B &
     *   C D
    */
    // print_truth_table("PQ|RS&&");//(P|Q)&(R&S)
    // print_truth_table("PR&PS&|QR&QS&&|");//(P&R)|(P&S)|(Q&R)&(Q&S)
    // AB&A!B!&|!AB!&A!B&||
    // A!B!|AB|&
}
// ab!c^de|f&=|
// ((!b) ^ c) = (d | e) & f | a

/*
 * AB^
 * 
 *      |
 *     / \
 *    &   &
 *   /|  / \
 *  A ! B   !
 *    |     |
 *    B     A
 * 
 * AB=
 *      |
 *     / \
 *    &   & 
 *   / \  |\
 *  !   ! A B
 *  |   |
 *  B   A
 *
 * 
 * AB|!!
 * AB|
 * AB|!
 * A!B!& 
 * 1 2 + 3 *
 * 3 1 * 2 3 * +
 * 3(1 + 2)
 * 3 * 1 + 3 * 2
 * AB|C&
 * CA&AB&|
 * 
 * 
 * 
 * CB&A&
 * ABC&&
 * 
 * A & ( B & C )
 * (A & B) & (A & C)
 * AB&AC&&
 * 
 * D & ( C & ( B & A ))
 * AB|C|
 * ABC||
 * 
 *      |
 *     / \
 *    A   &
 *       / \
 *      B   C
 *
 * 
 *        &
 *    |       |
 *  A   B   A   C
 *
 * 
 *  ABCD&&&
 * 
 *       &
 *      / \
 *     &   &
 *    /|   |\
 *   A B   C D
 * 
 *  A    B
 * (AB&)(CD&)|
 * 
 * 
 * (1 + 2) / (3 - 4)
*/
