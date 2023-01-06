pub mod binary_tree {

	use std::boxed::Box;

	/* Binary Tree */
	pub enum BinaryTree {
		Node {
			value: String,
			left: Box<BinaryTree>,
			right: Box<BinaryTree>,
		},
		Leaf, // Represent an empty node
	}

	impl BinaryTree {
		pub fn new(value: String, left: BinaryTree, right: BinaryTree) -> BinaryTree {
			BinaryTree::Node {
				value,
				left: Box::new(left),
				right: Box::new(right),
			}
		}
	}

	pub fn print_binary_tree(tree: &BinaryTree, mut space: i32) {
		let	mut i = 5;
		match tree {
			// If it's a node
			BinaryTree::Node { value, left, right } => {
				space += 5;
				println!("{}", value);
				print_binary_tree(right, space);
				while i < space {
					print!(" ");
					i += 1;
				}
				print!("{}(", value);
				print_binary_tree(left, space);
			}
			// If it's a leaf do nothing
			BinaryTree::Leaf => (),
		}
	}
}