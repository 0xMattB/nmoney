extern crate nmoney;

use nmoney::Money;

fn main() {
	let m1 = Money::new(10, 25, true).unwrap();
	let m2 = Money::new(21, 33, true).unwrap();
	
	// testing '+' operator
	let sum = m1 + m2;
	println!("Testing '+' operator : {m1} + {m2} = {sum}");
	
	// testing '+=' operator
	let mut sum = m2;
	sum += m1;
	println!("Testing '+=' operator: {m2} + {m1} = {sum}");
	
	// testing '-' operator
	let diff = m1 - m2;
	println!("Testing '-' operator : {m1} - {m2} = {diff}");
	
	// testing '-=' operator
	let mut diff = m2;
	diff -= m1;
	println!("Testing '-=' operator: {m2} - {m1} = {diff}");
}