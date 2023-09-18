extern crate nmoney;

use nmoney::{Money, MoneySign};

fn main() {
	let m1 = Money::new(21, 33, MoneySign::Positive).unwrap();
	let m2 = Money::new(10, 25, MoneySign::Positive).unwrap();
	let m3 = m2;
	
	comparison_tests(&m1, &m2);
	println!();
	
	comparison_tests(&m2, &m1);
	println!();
	
	comparison_tests(&m2, &m3);
	println!();
}

fn comparison_tests(a: &Money, b: &Money) {
	println!("{a} >  {b}?  {}", a > b);
	println!("{a} <  {b}?  {}", a < b);
	println!("{a} >= {b}?  {}", a >= b);
	println!("{a} <= {b}?  {}", a <= b);
	println!("{a} == {b}?  {}", a == b);
}