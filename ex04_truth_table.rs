mod ex03_eval_formula;
use crate::ex03_eval_formula::ex03_eval_formula;

// Exemple with the formula "AB&C|" :
// | A | B | C | = |
// |---|---|---|---|
// | 0 | 0 | 0 | 0 |
// | 0 | 0 | 1 | 1 |
// | 0 | 1 | 0 | 0 |
// | 0 | 1 | 1 | 1 |
// | 1 | 0 | 0 | 0 |
// | 1 | 0 | 1 | 1 |
// | 1 | 1 | 0 | 1 |
// | 1 | 1 | 1 | 1 |

fn print_truth_table(formula: &str) {
	// Count number of letters
	let mut letters: Vec<char> = Vec::new();
	for c in formula.chars() {
		if c.is_alphabetic() && !letters.contains(&c) {
			letters.push(c);
		}
	}
	letters.sort();
	let letters_count = letters.len();

	// Print header
	for c in letters.iter() {
		print!("| {} ", c);
	}
	print!("| = |\n|---|");
	for _c in letters.iter() {
		print!("---|");
	}

	let nb_comb = 2_i32.pow(letters_count as u32); // 2^letters_count = number of possible combinations of bits

	// Process and print truth table
	for i in 0..nb_comb {
		let mut s = format!("{:b}", i);	  // Convert i to binary
		while s.len() < letters_count {   // Add 0 to the left if needed 
			s = format!("0{}", s);
		}
		print!("\n|");
		for c in s.chars() {
			print!(" {} |", c);
		}
		let mut formula = formula.to_string();	// Copy formula
		for (i, c) in letters.iter().enumerate() {
			formula = formula.replace(*c, &s[i..i+1]); // Replace letters by bits
		}
		print!(" {} |", eval_formula(&formula) as i32);
	}
	println!();
}

fn main() {
	let formula = "AB&C|";
	print_truth_table(formula);
}