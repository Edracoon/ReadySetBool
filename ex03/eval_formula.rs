/*
 * Write a function that evaluates a formula in reverse polish notation (RPN) and returns the result.
 * The formula is a string of numbers and operators separated by spaces.
 * The operators are +, -, *, /, and ^ (exponentiation).
 * https://www.youtube.com/watch?v=qN8LPIcY6K4
 */

/*
 * Wikipedia on boolean algebra:
 * https://en.wikipedia.org/wiki/Boolean_algebra
 */

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

fn main() {
	let formula = "1011||=";
	println!("{} = {}", formula, eval_formula(formula));
}