use std::boxed::Box;

/* Binary Tree */
enum BinaryTree<T> {
	Node {
		value: T,
		left: Box<BinaryTree<T>>,
		right: Box<BinaryTree<T>>,
	},
	Leaf, // Represent an empty node
}

impl<T> BinaryTree<T> {
	fn new(value: T, left: BinaryTree<T>, right: BinaryTree<T>) -> BinaryTree<T> {
		BinaryTree::Node {
			value,
			left: Box::new(left),
			right: Box::new(right),
		}
	}
}

fn print_binary_tree<T>(tree: &BinaryTree<T>, space: i32) {
	let	i = 5;
	match tree {
		// If it's a node
		BinaryTree::Node { value, left, right } => {
			space += 5;
			println!("{}", value);
			print_binary_tree(right, space);
			while i++ < space {
				print!(" ");
			}
			print!("{}(", value);
			print_binary_tree(left, space);
		}
		// If it's a leaf do nothing
		BinaryTree::Leaf => (),
	}
}