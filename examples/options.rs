extern crate nmoney;

use nmoney::Money;
use nmoney::money::options::NegativeView;

fn main() {
	let mut m = Money::new(10, 25, true).unwrap();

	println!("Default    : {m}");
	
	m.options().set_symbol('£');
	println!("New symbol : {m}");
	
	m.options().set_show_symbol(false);
	println!("Hide symbol: {m}");
	
	let mut m = -m;
	
	m.options().set_negative_view(NegativeView::Paren);
	println!("Negative Parenthesis: {m}");
	
	m.options().set_negative_view(NegativeView::Minus);
	println!("Negative Minus Sign : {m}");
	
	m.options().set_negative_view(NegativeView::Hide);
	println!("Negative Hidden Sign: {m}");
}