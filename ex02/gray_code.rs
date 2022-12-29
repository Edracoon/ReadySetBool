/**
 ** Code gray est un type de codage binaire permettant de ne modifier
 ** qu'un seul bit à la fois quand un nombre est augmenté d'une unité.
 ** C'est très utile dans une séquence normale de chiffres binaires générés
 ** par du hardware qui pourrait causer une erreur / ambiguité durant
 ** une transition d'un nombre vers le suivant.
 * DECI		BIN		GRAY
 * 	5		0101	0111
 * 	6		0110	0101
 */

// Exemple avec 6:
// 6:	   0110
// 6 >> 1: 0011
//       = 0101 = 5(deci)

fn gray_code(n: u32) -> u32 {
	return n ^ (n >> 1);
}

fn main() {
	let mut i = 0;
	println!("Deci\t| Gray");
	while i < 10 {
		println!("{}\t| {}", i, gray_code(i));
		i += 1;
	}
	println!("{}\t| {}", 200, gray_code(200));
}