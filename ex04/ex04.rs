
fn eval_formula(formula: &str) -> bool {

	/* Collect chars from input in a array */
	let char_vec: Vec<char> = formula.chars().collect();

	/* Declare our stack to solve the RPN formula */
	let mut stack: Vec<bool> = Vec::new();

	/* Iterate over the array to solve the formula */
	for token in char_vec {
		match token {
			'!' => { // NOT	¬x ex: ¬1 = 0, ¬0 = 1
				let a = stack.pop().unwrap();
				stack.push(!a);
			}
			'&' => { // AND x ∧ y ex: 1 ∧ 1 = 1, 1 ∧ 0 = 0, 0 ∧ 1 = 0, 0 ∧ 0 = 0
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(a & b);
			}
			'|' => { // OR x ∨ y ex: 1 ∨ 1 = 1, 1 ∨ 0 = 1, 0 ∨ 1 = 1, 0 ∨ 0 = 0
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(a | b);
			}
			'^' => { // XOR x ⊕ y ex: 1 ⊕ 1 = 0, 1 ⊕ 0 = 1, 0 ⊕ 1 = 1, 0 ⊕ 0 = 0
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(a ^ b);
			}
			
			'=' => { // EQUALS x = y ex: 1 = 1 = 1, 1 = 0 = 0, 0 = 1 = 0, 0 = 0 = 1
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(a == b);
			}
			'>' => { // IMPLIES or MATERIAL CONDITIONAL x → y ex: 1 → 1 = 1, 1 → 0 = 0, 0 → 1 = 1, 0 → 0 = 1
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(!a | b);
			}
			/* If not an operand then it's a bit */
			_ => {
				let		n: i32 = token.to_digit(10).unwrap() as i32;
				let		bit = n == 1;
				stack.push(bit);
			}
		}
	}
	return stack.pop().unwrap();
}


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
			formula = formula.replace(*c, &s[i..i+1]);
		}
		print!(" {} |", eval_formula(&formula) as i32);
	}
	println!();
}

fn main() {
	let formula = "AB&C^";
	print_truth_table(formula);
}