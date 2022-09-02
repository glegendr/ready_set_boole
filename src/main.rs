mod exo_00;
mod exo_01;
mod exo_02;
mod exo_03;
mod exo_04;
mod exo_05;
mod exo_06;
mod exo_07;
mod exo_08;
mod exo_09;
mod exo_10;
use exo_00::adder;
use exo_01::multiplier;
use exo_02::gray_code;
use exo_03::eval_formula;
use exo_04::{print_truth_table, calc_truth_table};
use exo_05::negation_normal_form;
use exo_06::conjunctive_normal_form;
use exo_07::sat;
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
    print_truth_table("AA!^");
    // print_truth_table("AB!C^DE|FGHIJKLM&=|=&|^^^=");
    // println!("\n--- AB!C^DE|FGHIJKLMNOPQZSTU&=|=&|^^^=|=^>>&=^ ---");
    // print_truth_table("AB!C^DE|FGHIJKLMNOPQZSTU&=|=&|^^^=|=^>>&=^");
    print_truth_table("ABA&^BCA|^|");
    // print_truth_table("ABA&^BCA|^|")
    println!("======== Negation Normal Form ========");
    println!("{}", negation_normal_form("AB="));
    println!("{}", negation_normal_form("AB=!"));
    println!("{}", negation_normal_form("AB>"));
    println!("{}", negation_normal_form("CD^"));
    println!("{}", negation_normal_form("AB=CD^>"));
    println!("{}", negation_normal_form("AB=!!"));
    println!("======== Conjonctive Normal Form ========");
    // conjunctive_normal_form("AB&CD&EF&GH&|||");
    check_string("A!B!|", &conjunctive_normal_form("AB&!"), "AB&!");
    check_string("A!B!&", &conjunctive_normal_form("AB|!"), "AB|!");
    check_string("AB|C&", &conjunctive_normal_form("AB|C&"), "AB|C&");
    check_string("ABCD|||", &conjunctive_normal_form("AB|C|D|"), "AB|C|D|");
    check_string("ABCD&&&", &conjunctive_normal_form("AB&C&D&"), "AB&C&D&");
    check_string("A!B!C!||", &conjunctive_normal_form("AB&!C!|"), "AB&!C!|");
    check_string("A!B!C!&&", &conjunctive_normal_form("AB|!C!&"), "AB|!C!&");
    check_string("CA|CB|&DA|DB|&&", &conjunctive_normal_form("AB&CD&|"), "AB&CD&|");
    check_string(&calc_truth_table("AB&CD&|"), &calc_truth_table("CA|CB|&DA|DB|&&"), "CA|CB|&DA|DB|&& == AB&CD&|");
    check_string("IKOM|||ACGE||||IKOM|||ACGF||||&IKOM|||ACHE||||IKOM|||ACHF||||&&IKOM|||ADGE||||IKOM|||ADGF||||&IKOM|||ADHE||||IKOM|||ADHF||||&&&IKOM|||BCGE||||IKOM|||BCGF||||&IKOM|||BCHE||||IKOM|||BCHF||||&&IKOM|||BDGE||||IKOM|||BDGF||||&IKOM|||BDHE||||IKOM|||BDHF||||&&&&IKON|||ACGE||||IKON|||ACGF||||&IKON|||ACHE||||IKON|||ACHF||||&&IKON|||ADGE||||IKON|||ADGF||||&IKON|||ADHE||||IKON|||ADHF||||&&&IKON|||BCGE||||IKON|||BCGF||||&IKON|||BCHE||||IKON|||BCHF||||&&IKON|||BDGE||||IKON|||BDGF||||&IKON|||BDHE||||IKON|||BDHF||||&&&&&IKPM|||ACGE||||IKPM|||ACGF||||&IKPM|||ACHE||||IKPM|||ACHF||||&&IKPM|||ADGE||||IKPM|||ADGF||||&IKPM|||ADHE||||IKPM|||ADHF||||&&&IKPM|||BCGE||||IKPM|||BCGF||||&IKPM|||BCHE||||IKPM|||BCHF||||&&IKPM|||BDGE||||IKPM|||BDGF||||&IKPM|||BDHE||||IKPM|||BDHF||||&&&&IKPN|||ACGE||||IKPN|||ACGF||||&IKPN|||ACHE||||IKPN|||ACHF||||&&IKPN|||ADGE||||IKPN|||ADGF||||&IKPN|||ADHE||||IKPN|||ADHF||||&&&IKPN|||BCGE||||IKPN|||BCGF||||&IKPN|||BCHE||||IKPN|||BCHF||||&&IKPN|||BDGE||||IKPN|||BDGF||||&IKPN|||BDHE||||IKPN|||BDHF||||&&&&&&ILOM|||ACGE||||ILOM|||ACGF||||&ILOM|||ACHE||||ILOM|||ACHF||||&&ILOM|||ADGE||||ILOM|||ADGF||||&ILOM|||ADHE||||ILOM|||ADHF||||&&&ILOM|||BCGE||||ILOM|||BCGF||||&ILOM|||BCHE||||ILOM|||BCHF||||&&ILOM|||BDGE||||ILOM|||BDGF||||&ILOM|||BDHE||||ILOM|||BDHF||||&&&&ILON|||ACGE||||ILON|||ACGF||||&ILON|||ACHE||||ILON|||ACHF||||&&ILON|||ADGE||||ILON|||ADGF||||&ILON|||ADHE||||ILON|||ADHF||||&&&ILON|||BCGE||||ILON|||BCGF||||&ILON|||BCHE||||ILON|||BCHF||||&&ILON|||BDGE||||ILON|||BDGF||||&ILON|||BDHE||||ILON|||BDHF||||&&&&&ILPM|||ACGE||||ILPM|||ACGF||||&ILPM|||ACHE||||ILPM|||ACHF||||&&ILPM|||ADGE||||ILPM|||ADGF||||&ILPM|||ADHE||||ILPM|||ADHF||||&&&ILPM|||BCGE||||ILPM|||BCGF||||&ILPM|||BCHE||||ILPM|||BCHF||||&&ILPM|||BDGE||||ILPM|||BDGF||||&ILPM|||BDHE||||ILPM|||BDHF||||&&&&ILPN|||ACGE||||ILPN|||ACGF||||&ILPN|||ACHE||||ILPN|||ACHF||||&&ILPN|||ADGE||||ILPN|||ADGF||||&ILPN|||ADHE||||ILPN|||ADHF||||&&&ILPN|||BCGE||||ILPN|||BCGF||||&ILPN|||BCHE||||ILPN|||BCHF||||&&ILPN|||BDGE||||ILPN|||BDGF||||&ILPN|||BDHE||||ILPN|||BDHF||||&&&&&&&JKOM|||ACGE||||JKOM|||ACGF||||&JKOM|||ACHE||||JKOM|||ACHF||||&&JKOM|||ADGE||||JKOM|||ADGF||||&JKOM|||ADHE||||JKOM|||ADHF||||&&&JKOM|||BCGE||||JKOM|||BCGF||||&JKOM|||BCHE||||JKOM|||BCHF||||&&JKOM|||BDGE||||JKOM|||BDGF||||&JKOM|||BDHE||||JKOM|||BDHF||||&&&&JKON|||ACGE||||JKON|||ACGF||||&JKON|||ACHE||||JKON|||ACHF||||&&JKON|||ADGE||||JKON|||ADGF||||&JKON|||ADHE||||JKON|||ADHF||||&&&JKON|||BCGE||||JKON|||BCGF||||&JKON|||BCHE||||JKON|||BCHF||||&&JKON|||BDGE||||JKON|||BDGF||||&JKON|||BDHE||||JKON|||BDHF||||&&&&&JKPM|||ACGE||||JKPM|||ACGF||||&JKPM|||ACHE||||JKPM|||ACHF||||&&JKPM|||ADGE||||JKPM|||ADGF||||&JKPM|||ADHE||||JKPM|||ADHF||||&&&JKPM|||BCGE||||JKPM|||BCGF||||&JKPM|||BCHE||||JKPM|||BCHF||||&&JKPM|||BDGE||||JKPM|||BDGF||||&JKPM|||BDHE||||JKPM|||BDHF||||&&&&JKPN|||ACGE||||JKPN|||ACGF||||&JKPN|||ACHE||||JKPN|||ACHF||||&&JKPN|||ADGE||||JKPN|||ADGF||||&JKPN|||ADHE||||JKPN|||ADHF||||&&&JKPN|||BCGE||||JKPN|||BCGF||||&JKPN|||BCHE||||JKPN|||BCHF||||&&JKPN|||BDGE||||JKPN|||BDGF||||&JKPN|||BDHE||||JKPN|||BDHF||||&&&&&&JLOM|||ACGE||||JLOM|||ACGF||||&JLOM|||ACHE||||JLOM|||ACHF||||&&JLOM|||ADGE||||JLOM|||ADGF||||&JLOM|||ADHE||||JLOM|||ADHF||||&&&JLOM|||BCGE||||JLOM|||BCGF||||&JLOM|||BCHE||||JLOM|||BCHF||||&&JLOM|||BDGE||||JLOM|||BDGF||||&JLOM|||BDHE||||JLOM|||BDHF||||&&&&JLON|||ACGE||||JLON|||ACGF||||&JLON|||ACHE||||JLON|||ACHF||||&&JLON|||ADGE||||JLON|||ADGF||||&JLON|||ADHE||||JLON|||ADHF||||&&&JLON|||BCGE||||JLON|||BCGF||||&JLON|||BCHE||||JLON|||BCHF||||&&JLON|||BDGE||||JLON|||BDGF||||&JLON|||BDHE||||JLON|||BDHF||||&&&&&JLPM|||ACGE||||JLPM|||ACGF||||&JLPM|||ACHE||||JLPM|||ACHF||||&&JLPM|||ADGE||||JLPM|||ADGF||||&JLPM|||ADHE||||JLPM|||ADHF||||&&&JLPM|||BCGE||||JLPM|||BCGF||||&JLPM|||BCHE||||JLPM|||BCHF||||&&JLPM|||BDGE||||JLPM|||BDGF||||&JLPM|||BDHE||||JLPM|||BDHF||||&&&&JLPN|||ACGE||||JLPN|||ACGF||||&JLPN|||ACHE||||JLPN|||ACHF||||&&JLPN|||ADGE||||JLPN|||ADGF||||&JLPN|||ADHE||||JLPN|||ADHF||||&&&JLPN|||BCGE||||JLPN|||BCGF||||&JLPN|||BCHE||||JLPN|||BCHF||||&&JLPN|||BDGE||||JLPN|||BDGF||||&JLPN|||BDHE||||JLPN|||BDHF||||&&&&&&&&", &conjunctive_normal_form("AB&CD&EF&GH&|||IJ&KL&MN&OP&||||"), "AB&CD&EF&GH&|||IJ&KL&MN&OP&||||");
    check_string(&calc_truth_table("AB&CD&EF&GH&|||"), &calc_truth_table("ACGE|||ACGF|||&ACHE|||ACHF|||&&ADGE|||ADGF|||&ADHE|||ADHF|||&&&BCGE|||BCGF|||&BCHE|||BCHF|||&&BDGE|||BDGF|||&BDHE|||BDHF|||&&&&"), "CA|CB|&DA|DB|&& == AB&CD&|");
    println!("======== SAT ========");
    check_string("true", &sat("AB|").to_string(), "AB|");
    check_string("true", &sat("AB&").to_string(), "AB&");
    check_string("false", &sat("AA!&").to_string(), "AA!&");
    check_string("false", &sat("AA^").to_string(), "AA^");
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
 * 
 * 
 * 
 * ((A!A|) (A!A|) &) ((A!A|) (A!A|) &) &
 * 
 * A <-> A
 * !A <-> !A
 * 
*/


