
mod ex00_adder;
use crate::ex00_adder::ex00_adder::adder;

fn multiplier(mut a: u32, mut b: u32) -> u32 {
	let mut result	= 0;

	while b != 0
	{
		// If b is odd, add a to result
		if (b & 1) > 0 {
			result = adder(result, a);
		}
		a = a << 1;	// Multiplication by 2
		b = b >> 1;	// Division by 2
	}
	return result;
}

fn main() {
	let a = 3;
	let mut b = 3;
	let mut res;
	while b != 0
	{
		res = multiplier(a, b);
		println!("{} * {} = {}\t=> {}", a, b, res, (if a * b == res { "OK" } else { "KO" }));
		b -= 1;
	}
}