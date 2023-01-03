

fn eval_formula(formula: &str) -> bool {
	let char_vec: Vec<char> = formula.chars().collect();
	let mut stack: Vec<bool> = Vec::new();
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

fn get_truth_table(formula: &str) -> Vec<i32> {
	let mut letters: Vec<char> = Vec::new();
	for c in formula.chars() {
		if c.is_alphabetic() && !letters.contains(&c) { letters.push(c); }
	}
	letters.sort();

	let letters_count = letters.len();
	let nb_comb = 2_i32.pow(letters_count as u32); // 2^letters_count = number of possible combinations of bits

	let mut results = vec![0; nb_comb as usize]; // Create a vector of 0 with nb_comb elements

	// Process and print truth table
	for i in 0..nb_comb {
		let mut s = format!("{:b}", i);	  // Convert i to binary
		while s.len() < letters_count {   // Add 0 to the left if needed 
			s = format!("0{}", s);
		}
		let mut formula = formula.to_string();	// Copy formula
		for (i, c) in letters.iter().enumerate() {
			formula = formula.replace(*c, &s[i..i+1]); // Replace letters by bits
		}
		results[i as usize] = eval_formula(&formula) as i32;
	}
	return results;
}

/*
 * Wikipedia: https://en.wikipedia.org/wiki/Negation_normal_form
 * The ! operator is the negation operator, it must only be applied to a single variable.
 * The other two allowed operators are the conjunction (AND,&) and disjunction (OR,|) operators.
 */

// Write a function that takes a formula in RPN as a string and returns its negation normal form.
fn negation_normal_form(formula: &str) -> String {
	println!("{:?} {:?}", get_truth_table(formula), get_truth_table("A!B!|"));
	return "".to_string();
}

fn main() {
	let formula = "AB&!";
	println!("{} -> {}", formula, negation_normal_form(formula));
}
