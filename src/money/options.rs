const DEFAULT_SYMBOL: char = '$';
const DEFAULT_SHOW_SYMBOL: bool = true;
const DEFAULT_NEGATIVE_VIEW: NegativeView = NegativeView::Minus;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NegativeView {
	Minus,
	Paren,
	Hide,
}

#[derive(Copy, Clone, Debug)]
pub struct Options {
	symbol: char,
	show_symbol: bool,
	negative_view: NegativeView,
}

impl Options {
	#[doc(hidden)]
	pub fn new() -> Self {
		Self {
			symbol: DEFAULT_SYMBOL,
			show_symbol: DEFAULT_SHOW_SYMBOL,
			negative_view: DEFAULT_NEGATIVE_VIEW,
		}
	}
	
	/// Returns the current money symbol in use.
	pub fn symbol(&self) -> char {
		self.symbol
	}
	
	/// Returns whether the money symbol is enabled in the return string.
	pub fn show_symbol(&self) -> bool {
		self.show_symbol
	}
	
	/// Returns the "negative view" setting in use.
	pub fn negative_view(&self) -> NegativeView {
		self.negative_view
	}
	
	/// Set the money symbol to use.  
	/// Default: '$'
	///
	/// Digits are considered invalid symbols, and the function will return `false`.  
	/// Otherwise, the function returns `true`.
	///
	/// Example
	///
	/// ```
	/// # use nmoney::{Money, MoneySign};
	/// let mut m = Money::new(5, 25, MoneySign::Positive).unwrap();
	/// m.options().set_symbol('£');
	///
	/// assert_eq!(m.to_string(), "£5.25");
	/// ```
	pub fn set_symbol(&mut self, symbol: char) -> bool {
		if symbol.is_ascii_digit() {
			false
		} else {
			self.symbol = symbol;
			true
		}
	}
	
	/// Set whether the money symbol is included in the string.  
	/// Default: true
	///
	/// Example
	///
	/// ```
	/// # use nmoney::{Money, MoneySign};
	/// let mut m = Money::new(5, 25, MoneySign::Positive).unwrap();
	/// m.options().set_show_symbol(false);
	///
	/// assert_eq!(m.to_string(), "5.25");
	/// ```
	pub fn set_show_symbol(&mut self, show_symbol: bool) {
		self.show_symbol = show_symbol;
	}

	/// Set the negative representation to use.  
	/// Default: Minus
	///
	/// Examples
	///
	/// ```
	/// # use nmoney::{Money, MoneySign};
	/// # use nmoney::money::options::NegativeView;
	/// let mut m = Money::new(5, 25, MoneySign::Negative).unwrap();
	/// m.options().set_negative_view(NegativeView::Paren);
	///
	/// assert_eq!(m.to_string(), "($5.25)");
	/// ```
	///
	/// ```
	/// # use nmoney::{Money, MoneySign};
	/// # use nmoney::money::options::NegativeView;
	/// let mut m = Money::new(5, 25, MoneySign::Negative).unwrap();
	/// m.options().set_negative_view(NegativeView::Hide);
	///
	/// assert_eq!(m.to_string(), "$5.25");
	/// ```
	pub fn set_negative_view(&mut self, negative_view: NegativeView) {
		self.negative_view = negative_view;
	}
}