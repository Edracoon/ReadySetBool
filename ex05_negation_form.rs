use std::boxed::Box;

struct Node {
	pub value: char,
	pub left: Option<Box<Node>>,
	pub right: Option<Box<Node>>,
}

impl Node {
	pub fn new(value: char, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
		Node {
			value,
			left,
			right,
		}
	}
}

fn print_binary_tree(node: &Option<Box<Node>>, space: i32) {
	match node {
		Some(ref node) => {
			print_binary_tree(&node.left, space + 5);
			for _ in 0..space {
				print!(" ");
			}
			println!("|----{}", node.value);
			print_binary_tree(&node.right, space + 5);
		},
		None => {},
	}
}

/*
 * Parse Reverse Polish Notation (RPN) using a stack and a binary tree
*/
fn parse_rpn(formula: &str) -> Option<Box<Node>> {
	let mut stack: Vec<Option<Box<Node>>> = Vec::new();
	for token in formula.chars() {
		match token {
			'!' => { // NOT	¬x ex: ¬1 = 0, ¬0 = 1
				let node = Node::new(token, stack.pop().unwrap(), None);
				stack.push(Some(Box::new(node)));
			}
			'&' => { // AND x ∧ y ex: 1 ∧ 1 = 1, 1 ∧ 0 = 0, 0 ∧ 1 = 0, 0 ∧ 0 = 0
				let node = Node::new(token, stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(Some(Box::new(node)));
			}
			'|' => { // OR x ∨ y ex: 1 ∨ 1 = 1, 1 ∨ 0 = 1, 0 ∨ 1 = 1, 0 ∨ 0 = 0
				let node = Node::new(token, stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(Some(Box::new(node)));
			}
			'^' => { // XOR x ⊕ y ex: 1 ⊕ 1 = 0, 1 ⊕ 0 = 1, 0 ⊕ 1 = 1, 0 ⊕ 0 = 0
				let node = Node::new(token, stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(Some(Box::new(node)));
			}
			'=' => { // EQUALS x = y ex: 1 = 1 = 1, 1 = 0 = 0, 0 = 1 = 0, 0 = 0 = 1
				let node = Node::new(token, stack.pop().unwrap(), stack.pop().unwrap());
				stack.push(Some(Box::new(node)));
			}
			_ => { // Variables
				let node = Node::new(token, None, None);
				stack.push(Some(Box::new(node)));
			}
		}
		// print the stack

	}
	// Now print the tree using the print btree function:
	let ret = stack.pop().unwrap();
	print_binary_tree(&ret, 4);
	return ret;
}

// fn get_truth_table(formula: &str) {
// 	// Make string for all result of the truth table
// 	let mut results = "";
// 	// Count number of letters
// 	let mut letters: Vec<char> = Vec::new();
// 	for c in formula.chars() {
// 		if c.is_alphabetic() && !letters.contains(&c) {
// 			letters.push(c);
// 		}
// 	}
// 	letters.sort();
// 	let letters_count = letters.len();

// 	let nb_comb = 2_i32.pow(letters_count as u32); // 2^letters_count = number of possible combinations of bits

// 	// Process and print truth table
// 	for i in 0..nb_comb {
// 		let mut s = format!("{:b}", i);	  // Convert i to binary
// 		while s.len() < letters_count {   // Add 0 to the left if needed 
// 			s = format!("0{}", s);
// 		}

// 		let mut formula = formula.to_string();	// Copy formula
// 		for (i, c) in letters.iter().enumerate() {
// 			formula = formula.replace(*c, &s[i..i+1]); // Replace letters by bits
// 		}
// 		results += eval_formula(&formula) as i32.to_string();
// 	}
// }

/*
* Wikipedia: https://en.wikipedia.org/wiki/Negation_normal_form
* The ! operator is the negation operator, it must only be applied to a single variable.
* The other two allowed operators are the conjunction (AND,&) and disjunction (OR,|) operators.
*/

// Write a function that takes a formula in RPN as a string and returns its negation normal form.
pub fn negation_normal_form(formula: &str) -> String {
	// println!("{:?} {:?}", get_truth_table(formula), get_truth_table("A!B!|"));
	let node = parse_rpn(formula);
	match node {
		Some(node) => println!("{}", node.value),
		None => {},
	}
	return "".to_string();
}

fn main() {
	let formula = "G!A^!B&C&E&";
	println!("{} -> {}", formula, negation_normal_form(formula));
}
	