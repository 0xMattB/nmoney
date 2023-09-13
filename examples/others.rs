extern crate nmoney;

use nmoney::Money;
use nmoney::money::options::NegativeView;

fn main() {
	// to-from string
	let m = Money::new(12, 99, true).unwrap();
	println!("original: {m}");

	let ms = m.to_string();
	println!("as String: {ms}");
	
	let ms = ms.replace("9", "0");
	let ms = ms.replace("$", "#");
	println!("modified String: {ms}");
	
	let m = Money::from_str(&ms).unwrap();
	println!("as Money (from String): {m}");
	println!();
	

	// to-from coin
	let m = Money::new(109, 85, false).unwrap();
	println!("original: {m}");
	
	let cents = m.as_cents();
	println!("in pennies: {cents}");
	
	let cents = cents + 20000;
	println!("pennies + 20000: {cents}");
	
	let m = Money::from_cents(cents);
	println!("last pennies as Money: {m}");
	println!();
	
	// copy options
	let mut m1 = Money::new(59, 99, false).unwrap();
	m1.options().set_symbol('#');
	m1.options().set_negative_view(NegativeView::Paren);
	println!("m1: {m1}");
	
	let mut m2 = Money::new(1098, 54, false).unwrap();
	println!("m2 before 'copy_options(): {m2}");
	
	Money::copy_options(&mut m2, &m1);
	println!("m2 after 'copy_options(): {m2}");
}