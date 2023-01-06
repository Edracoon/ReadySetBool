// Explanation:
// 4 + 6
// 0100 & 0110 = 0100
// 0100 << 1 = 1000
// carry = 1000 (=8)
// Repeat:
// result = 0100 ^ 0110 = 0010 (=2)
// Repeat with carry and result
// 1000 & 0010 = 0000 << 1 = 0000
// 0010 ^ 1000 = 1010 (=10)
// if (carry == 0) return result (=10)
// Repeat in case of carry != 0

pub mod ex00_adder {
	pub fn adder(mut a: u32, mut b: u32) -> u32 {

		let mut carry	= 1;
		let mut sum		= 0;

		while carry != 0 {
			carry = (a & b) << 1;	// AND and shift left to get the carry (la retenue)
			sum = a ^ b;			// XOR to get the sum result
			a = carry;
			b = sum;
		}
		return sum;
	}
}

use ex00_adder::adder;

fn main() {
	let a = 1293;
	let b = 7;
	println!("{} + {} = {}", a, b, adder(a, b));
}