mod	binary_tree;
use binary_tree::binary_tree::BinaryTree;
use binary_tree::binary_tree::print_binary_tree;

mod ex03_eval_formula;
use ex03_eval_formula::ex03_eval_formula::eval_formula;

/*
* Parse Reverse Polish Notation (RPN) using a stack and a binary tree
*/
fn parse_rpn(formula: &str) -> BinaryTree::Node {
	let mut stack: Vec<BinaryTree::Node> = Vec::new();
	let mut tree: BinaryTree::Node = BinaryTree::Node::new();
	for token in formula.chars() {
		match token {
			'!' => { // NOT	¬x ex: ¬1 = 0, ¬0 = 1
				let mut node = BinaryTree::Node::new();
				node.value = token;
				node.left = Some(Box::new(stack.pop().unwrap()));
				stack.push(node);
			}
			'&' => { // AND x ∧ y ex: 1 ∧ 1 = 1, 1 ∧ 0 = 0, 0 ∧ 1 = 0, 0 ∧ 0 = 0
				let mut node = BinaryTree::Node::new();
				node.value = token;
				node.right = Some(Box::new(stack.pop().unwrap()));
				node.left = Some(Box::new(stack.pop().unwrap()));
				stack.push(node);
			}
			'|' => { // OR x ∨ y ex: 1 ∨ 1 = 1, 1 ∨ 0 = 1, 0 ∨ 1 = 1, 0 ∨ 0 = 0
				let mut node = BinaryTree::Node::new();
				node.value = token;
				node.right = Some(Box::new(stack.pop().unwrap()));
				node.left = Some(Box::new(stack.pop().unwrap()));
				stack.push(node);
			}
			'^' => { // XOR x ⊕ y ex: 1 ⊕ 1 = 0, 1 ⊕ 0 = 1, 0 ⊕ 1 = 1, 0 ⊕ 0 = 0
				let mut node = BinaryTree::Node::new();
				node.value = token;
				node.right = Some(Box::new(stack.pop().unwrap()));
				node.left = Some(Box::new(stack.pop().unwrap()));
				stack.push(node);
			}
			'=' => { // EQUALS x = y ex: 1 = 1 = 1, 1 = 0 = 0, 0 = 1 = 0, 0 = 0 = 1
				let mut node = BinaryTree::Node::new();
				node.value = token;
				node.right = Some(Box::new(stack.pop().unwrap()));
				node.left = Some(Box::new(stack.pop().unwrap()));
				stack.push(node);
			}
			_ => { // Letters
				let mut node = BinaryTree::Node::new();
				node.value = token;
				stack.push(node);
			}
		}
	}
	tree = stack.pop().unwrap();
	print_binary_tree(&tree, 4);
	return tree;
}

/*
* Wikipedia: https://en.wikipedia.org/wiki/Negation_normal_form
* The ! operator is the negation operator, it must only be applied to a single variable.
* The other two allowed operators are the conjunction (AND,&) and disjunction (OR,|) operators.
*/

// Write a function that takes a formula in RPN as a string and returns its negation normal form.
pub fn negation_normal_form(formula: &str) -> String {
	println!("{:?} {:?}", get_truth_table(formula), get_truth_table("A!B!|"));
	return "".to_string();
}

fn main() {
	let formula = "AB&!";
	println!("{} -> {}", formula, negation_normal_form(formula));
}
	