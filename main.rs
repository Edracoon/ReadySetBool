

/* Main adder */
// mod ex00_adder;
// use crate::ex00_adder::ex00_adder::adder;
// fn main() {
// 	let a = 1293;
// 	let b = 7;
// 	println!("{} + {} = {}", a, b, adder(a, b));
// }

/* Main multiplier */
// mod ex01_multiplier;
// use crate::ex01_multiplier::ex01_multiplier::multiplier;
// fn main() {
// 	let a = 3;
// 	let mut b = 3;
// 	let mut res;
// 	while b != 0
// 	{
// 		res = multiplier(a, b);
// 		println!("{} * {} = {}\t=> {}", a, b, res, (if a * b == res { "OK" } else { "KO" }));
// 		b -= 1;
// 	}
// }

/* Main gray_code */
// mod ex02_gray_code;
// use crate::ex02_gray_code::ex02_gray_code::gray_code;
// fn main() {
// 	let mut i = 0;
// 	println!("Deci\t| Gray");
// 	while i < 10 {
// 		println!("{}\t| {}", i, gray_code(i));
// 		i += 1;
// 	}
// 	println!("{}\t| {}", 200, gray_code(200));
// }

/* Main eval_formula */
// mod ex03_eval_formula;
// use crate::ex03_eval_formula::ex03_eval_formula::eval_formula;
// fn main() {
// 	let formula = "1011||=";
// 	println!("{} = {}", formula, eval_formula(formula));
// }

/* Main truth_table */
// mod ex04_truth_table;
// use crate::ex04_truth_table::ex04_truth_table::print_truth_table
// fn main() {
// 	let formula = "AB&C|";
// 	print_truth_table(formula);
// }

/* Main negation_normal_form */
mod ex05_negation_form;
use crate::ex05_negation_form::ex05_negation_form::negation_normal_form;
fn main() {
	let formula = "AB&!";
	println!("{} -> {}", formula, negation_normal_form(formula));
}
