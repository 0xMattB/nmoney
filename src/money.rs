pub mod options;

use options::{Options, NegativeView};
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg};
use std::cmp::{PartialEq, Ordering};
use std::fmt;
use std::str::FromStr;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct MoneyErrorCents;

impl Error for MoneyErrorCents {}

impl fmt::Display for MoneyErrorCents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid number of cents")
    }
}

#[derive(Debug, Clone)]
pub struct MoneyErrorString;

impl Error for MoneyErrorString {}

impl fmt::Display for MoneyErrorString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid money string")
    }
}

#[derive(Debug, Clone)]
pub struct MoneyErrorOverflow;

impl Error for MoneyErrorOverflow {}

impl fmt::Display for MoneyErrorOverflow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "addition has resulted in overflow")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MoneySign {
	Positive,
	Negative,
}

#[derive(Debug, Copy, Clone)]
pub struct Money {
	dollars: u64,
	cents: u8,
	sign: MoneySign,
	options: Options,
}

impl Money {
	/// Creates a new Money instance.
	///
	/// `dollars` and `cents` are separate fields, and are absolute values.
	/// The `sign` field indicates whether the whole value is positive or negative.
	/// The `options` field allow certain options to be changed.
	///
	/// # Example
	///
	/// ```
	/// # use nmoney::{Money, MoneySign};
	/// let m = Money::new(5, 25, MoneySign::Positive).unwrap();
	/// 
	/// assert_eq!(m.to_string(), "$5.25");
	/// ```
	pub fn new(dollars: u64, cents: u8, mut sign: MoneySign) -> Result<Self, MoneyErrorCents> {
		if dollars == 0 && cents == 0 {
			sign = MoneySign::Positive;  // prevents negative 0.00
		}
		
		if cents < 100 {
			Ok(
				Self {
					dollars,
					cents,
					sign,
					options: Options::new(),
				}
			)
		} else {
			Err(MoneyErrorCents)
		}
	}
	
	/// Returns the `dollars` value of the Money instance.
	pub fn dollars(&self) -> u64 {
		self.dollars
	}
	
	/// Returns the `cents` value of the Money instance.
	pub fn cents(&self) -> u8 {
		self.cents
	}
	
	/// Returns the `sign` value of the Money instance.
	pub fn sign(&self) -> MoneySign {
		self.sign
	}
	
	/// Returns a mutable reference to the `options` value, allowing options to be updated.
	pub fn options(&mut self) -> &mut Options {
		&mut self.options
	}
	
	fn options_immutable(&self) -> &Options {
		&self.options
	}
	
	/// Returns the Money instance as the total number of cents, or an error if an overflow has occurred.
	///
	/// # Example
	///
	/// ```
	/// # use nmoney::{Money, MoneySign};
	/// let m = Money::new(5, 25, MoneySign::Negative).unwrap();
	/// let c = m.as_cents().unwrap();
	///
	/// assert_eq!(c, -525);
	/// ```
	pub fn as_cents(&self) -> Result<i64, MoneyErrorOverflow> {
		convert_money_to_whole(self)
	}
	
	/// Returns the cents as a Money instance.
	///
	/// # Example
	///
	/// ```
	/// # use nmoney::{Money, MoneySign};
	/// let m = Money::from_cents(-525);
	///
	/// assert_eq!(m.to_string(), "-$5.25");
	/// ```
	pub fn from_cents(cents: i64) -> Money {
		convert_whole_to_money(cents)
	}
	
	/// Converts a string into a Money type.
	///
	/// # Example
	///
	/// ```
	/// # use nmoney::{Money, MoneySign};
	/// let m1 = Money::new(5, 25, MoneySign::Positive).unwrap();
	/// let m2 = Money::from_str("5.25").unwrap();
	///
	/// assert_eq!(m1, m2);
	/// ```
	pub fn from_str(s: &str) -> Result<Self, MoneyErrorString> {
		let mut sign = MoneySign::Positive;
		let mut is_paren = false;
		let mut symbol = None;
		let mut r = String::from(s);
		
		// check for negative
		if r.starts_with("-") {
			sign = MoneySign::Negative;
			let _ = r.remove(0);
		} else if r.starts_with("(") {
			if r.ends_with(")") {
				sign = MoneySign::Negative;
				is_paren = true;
				let _ = r.remove(0);
				let _ = r.pop();
			} else {
				return Err(MoneyErrorString);
			}
		}
		
		// check for symbol
		let leading = r.remove(0);
		
		if leading.is_ascii_digit() {
			r.insert(0, leading);
		} else {
			symbol = Some(leading);
		}
		
		// break apart string
		let v: Vec<_> = r.split(".").collect();
		
		if v.len() != 2 {
			return Err(MoneyErrorString);
		}
		
		// convert vec elements
		let d = match v[0].parse::<u64>() {
			Ok(r) => { r },
			Err(_) => { return Err(MoneyErrorString); },
		};
		
		let c = match v[1].parse::<u8>() {
			Ok(r) => { r },
			Err(_) => { return Err(MoneyErrorString); },
		};

		if c >= 100 {
			return Err(MoneyErrorString);
		}
		
		let mut m = Money::new(d, c, sign).unwrap();
		
		if is_paren {
			m.options().set_negative_view(NegativeView::Paren);
		}
		
		if let Some(sym) = symbol {
			m.options().set_symbol(sym);
		} else {
			m.options().set_show_symbol(false);
		}
		
		Ok(m)
	}
	
	/// Copies the `options` of `src` to `dest`.
	///
	/// # Example
	///
	/// ```
	/// # use nmoney::{Money, MoneySign};
	/// # use nmoney::money::options::NegativeView;
	/// let mut m1 = Money::new(59, 99, MoneySign::Negative).unwrap();
	/// m1.options().set_symbol('#');
	/// m1.options().set_negative_view(NegativeView::Paren);
	///
	/// let mut m2 = Money::new(1098, 54, MoneySign::Negative).unwrap();
	/// Money::copy_options(&mut m2, &m1);
	///
	/// assert_eq!(m2.to_string(), "(#1098.54)");
	/// ```
	pub fn copy_options(dest: &mut Money, src: &Money) {
		dest.options = src.options;
	}
}

fn convert_money_to_whole(money: &Money) -> Result<i64, MoneyErrorOverflow> {
	let dollars: i64 = (money.dollars * 100) as i64;
	let cents: i64 = (money.cents) as i64;
	
	match dollars.checked_add(cents) {
		Some(mut sum) => {
			if money.sign == MoneySign::Negative {
				sum *= -1;
			}
			Ok(sum)
		},
		None => {
			Err(MoneyErrorOverflow)
		},
	}
}

fn convert_whole_to_money(mut whole: i64) -> Money {
	let mut sign = MoneySign::Positive;
	
	if whole < 0 {
		sign = MoneySign::Negative;
		whole *= -1;
	}
	
	Money {
		dollars: (whole / 100) as u64,
		cents: (whole % 100) as u8,
		sign,
		options: Options::new()
	}
}

impl Default for Money {
    fn default() -> Self {
		Self {
			dollars: 0,
			cents: 0,
			sign: MoneySign::Positive,
			options: Options::new(),
		}
	}
}

impl Add for Money {
	type Output = Self;
	
	fn add(self, other: Self) -> Self {
		let whole_1 = convert_money_to_whole(&self).expect("overflow on addition");
		let whole_2 = convert_money_to_whole(&other).expect("overflow on addition");
		
		match whole_1.checked_add(whole_2) {
			Some(sum) => {
				convert_whole_to_money(sum)
			},
			None => {
				panic!("overflow on addition");
			},
		}
	}
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Money {
	type Output = Self;
	
	fn sub(self, other: Self) -> Self {
		let whole_1 = convert_money_to_whole(&self).expect("overflow on subtraction");
		let whole_2 = convert_money_to_whole(&other).expect("overflow on subtraction");
		
		match whole_1.checked_sub(whole_2) {
			Some(difference) => {
				convert_whole_to_money(difference)
			},
			None => {
				panic!("underflow on subtraction");
			},
		}
	}
}

impl SubAssign for Money {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for Money {
	type Output = Self;
	
	fn neg(self) -> Self {
		let sign = if self.sign == MoneySign::Positive {
			MoneySign::Negative
		} else {
			MoneySign::Positive
		};
		
		Self {
			dollars: self.dollars,
			cents: self.cents,
			sign,
			options: self.options,
		}
	}
}

impl PartialEq for Money {
	fn eq(&self, other: &Self) -> bool {
		self.dollars == other.dollars &&
		self.cents == other.cents &&
		self.sign == other.sign
	}
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let m1 = self.as_cents().unwrap();
		let m2 = other.as_cents().unwrap();
		
		if m1 < m2 {
			Some(Ordering::Less)
		} else if m1 > m2 {
			Some(Ordering::Greater)
		} else {
			Some(Ordering::Equal)
		}
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut s = format!["{}.{:02}", self.dollars, self.cents];
		
		if self.options_immutable().show_symbol() {
			s.insert(0, self.options_immutable().symbol());
		}
		
		if self.sign() == MoneySign::Negative {
			/* 'NegativeView::Hide' simply omits the logic to add the negative indicator */
			if self.options_immutable().negative_view() == NegativeView::Minus {
				s.insert(0, '-');
			} else if self.options_immutable().negative_view() == NegativeView::Paren {
				s.insert(0, '(');
				s.push_str(")");
			}
		}

		write!(f, "{}", s)
    }
}

impl FromStr for Money {
    type Err = MoneyErrorString;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match Money::from_str(s) {
			Ok(r) => {
				Ok(r)
			},
			Err(_) => {
				Err(MoneyErrorString)
			},
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;
	
	#[test]
	fn positive_plus_positive() {
		let m1 = Money::new( 4, 56, MoneySign::Positive).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		assert_eq!(
			m1 + m2,
			Money { dollars: 17, cents: 5, sign: MoneySign::Positive, options: Options::new() }
		);
	}
	
	#[test]
	fn positive_plus_negative() {
		let m1 = Money::new( 4, 56, MoneySign::Positive).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Negative).unwrap();
		
		assert_eq!(
			m1 + m2,
			Money { dollars: 7, cents: 93, sign: MoneySign::Negative, options: Options::new() }
		);
	}
	
	#[test]
	fn negative_plus_positive() {
		let m1 = Money::new( 4, 56, MoneySign::Negative).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		assert_eq!(
			m1 + m2,
			Money { dollars: 7, cents: 93, sign: MoneySign::Positive, options: Options::new() }
		);
	}
	
	#[test]
	fn negative_plus_negative() {
		let m1 = Money::new( 4, 56, MoneySign::Negative).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Negative).unwrap();
		
		assert_eq!(
			m1 + m2,
			Money { dollars: 17, cents: 5, sign: MoneySign::Negative, options: Options::new() }
		);
	}
	
	#[test]
	fn positive_minus_positive() {
		let m1 = Money::new( 4, 56, MoneySign::Positive).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		assert_eq!(
			m1 - m2,
			Money { dollars: 7, cents: 93, sign: MoneySign::Negative, options: Options::new() }
		);
	}
	
	#[test]
	fn positive_minus_negative() {
		let m1 = Money::new( 4, 56, MoneySign::Positive).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Negative).unwrap();
		
		assert_eq!(
			m1 - m2,
			Money { dollars: 17, cents: 5, sign: MoneySign::Positive, options: Options::new() }
		);
	}
	
	#[test]
	fn negative_minus_positive() {
		let m1 = Money::new( 4, 56, MoneySign::Negative).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		assert_eq!(
			m1 - m2,
			Money { dollars: 17, cents: 5, sign: MoneySign::Negative, options: Options::new() }
		);
	}
	
	#[test]
	fn negative_minus_negative() {
		let m1 = Money::new( 4, 56, MoneySign::Negative).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Negative).unwrap();
		
		assert_eq!(
			m1 - m2,
			Money { dollars: 7, cents: 93, sign: MoneySign::Positive, options: Options::new() }
		);
	}

	#[test]
	fn negate() {
		let m = Money::new(15, 30, MoneySign::Positive).unwrap();
		let m2 = -m;
		
		assert_eq!(
			m2,
			Money { dollars: 15, cents: 30, sign: MoneySign::Negative, options: Options::new() }
		);
	}
	
	#[test]
	fn as_cents() {
		let m = Money::new(15, 30, MoneySign::Negative).unwrap();
		
		assert_eq!(
			m.as_cents().unwrap(),
			-1530
		);
	}
	
	#[test]
	fn add_assign() {
		let m1 = Money::new( 4, 56, MoneySign::Positive).unwrap();
		let mut m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		m2 += m1;
		
		assert_eq!(
			m2,
			Money { dollars: 17, cents: 5, sign: MoneySign::Positive, options: Options::new() }
		);
	}
	
	#[test]
	fn sub_assign() {
		let m1 = Money::new( 4, 56, MoneySign::Positive).unwrap();
		let mut m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		m2 -= m1;
		
		assert_eq!(
			m2,
			Money { dollars: 7, cents: 93, sign: MoneySign::Positive, options: Options::new() }
		);
	}
	
	#[test]
	fn less_than() {
		let m1 = Money::new( 4, 56, MoneySign::Positive).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		let e1 = m1 < m2;
		let e2 = m1 > m2;
		
		assert!(e1 && !e2);
	}
	
	#[test]
	fn less_than_or_equal() {
		let m1 = Money::new(12, 49, MoneySign::Positive).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		let e1 = m1 <= m2;
		let e2 = m1 >= m2;
		
		assert!(e1 && e2);
	}
	
	#[test]
	fn greater_than() {
		let m1 = Money::new( 4, 56, MoneySign::Positive).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		let e1 = m2 > m1;
		let e2 = m2 < m1;
		
		assert!(e1 && !e2);
	}
	
	#[test]
	fn greater_than_or_equal() {
		let m1 = Money::new(12, 50, MoneySign::Positive).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		let e1 = m1 >= m2;
		let e2 = m1 <= m2;
		
		assert!(e1 && !e2);
	}
	
	#[test]
	fn equal_to() {
		let m1 = Money::new(12, 49, MoneySign::Positive).unwrap();
		let m2 = Money::new(12, 49, MoneySign::Positive).unwrap();
		
		assert!(m1 == m2);
	}
	
	#[test]
	fn to_string_default() {
		let m = Money::new(12, 29, MoneySign::Positive).unwrap();
		
		assert_eq!(m.to_string(), "$12.29");
	}
	
	#[test]
	fn to_string_new_symbol() {
		let mut m = Money::new(12, 29, MoneySign::Positive).unwrap();
		m.options.set_symbol('#');
		
		assert_eq!(m.to_string(), "#12.29");
	}
	
	#[test]
	fn to_string_neg_minus() {
		let m = Money::new(12, 29, MoneySign::Negative).unwrap();
		
		assert_eq!(m.to_string(), "-$12.29");
	}
	
	#[test]
	fn to_string_neg_paren() {
		let mut m = Money::new(12, 29, MoneySign::Negative).unwrap();
		m.options.set_negative_view(NegativeView::Paren);
		
		assert_eq!(m.to_string(), "($12.29)");
	}
	
	#[test]
	fn to_string_neg_hide() {
		let mut m = Money::new(12, 29, MoneySign::Negative).unwrap();
		m.options.set_negative_view(NegativeView::Hide);
		
		assert_eq!(m.to_string(), "$12.29");
	}
	
	#[test]
	fn from_cents() {
		let m = Money::new(5, 76, MoneySign::Positive).unwrap();
		
		assert_eq!(m, Money::from_cents(576));
	}
	
	#[test]
	fn set_symbol_valid() {
		let mut m = Money::new(5, 76, MoneySign::Positive).unwrap();
		
		assert!(m.options().set_symbol('#'));
	}
	
	#[test]
	fn set_symbol_invalid() {
		let mut m = Money::new(5, 76, MoneySign::Positive).unwrap();
		
		assert!(!m.options().set_symbol('1'));
	}
	
	#[test]
	fn from_str_pos_no_symbol() {
		let m1 = Money::new(5, 34, MoneySign::Positive).unwrap();
		let m2 = Money::from_str("5.34").unwrap();
		
		assert!(
			m1 == m2 &&
			m2.options_immutable().symbol() == '$' &&
			m2.options_immutable().show_symbol() == false
		);
	}
	
	#[test]
	fn from_str_pos_symbol() {
		let m1 = Money::new(5, 34, MoneySign::Positive).unwrap();
		let m2 = Money::from_str("$5.34").unwrap();
		
		assert!(
			m1 == m2 &&
			m2.options_immutable().symbol() == '$' &&
			m2.options_immutable().show_symbol() == true
		);
	}
	
	#[test]
	fn from_str_minus_no_symbol() {
		let m1 = Money::new(5, 34, MoneySign::Negative).unwrap();
		let m2 = Money::from_str("-5.34").unwrap();
		
		assert!(
			m1 == m2 &&
			m2.options_immutable().symbol() == '$' &&
			m2.options_immutable().show_symbol() == false &&
			m2.options_immutable().negative_view() == NegativeView::Minus
		);
	}
	
	#[test]
	fn from_str_minus_symbol() {
		let m1 = Money::new(5, 34, MoneySign::Negative).unwrap();
		let m2 = Money::from_str("-$5.34").unwrap();
		
		assert!(
			m1 == m2 &&
			m2.options_immutable().symbol() == '$' &&
			m2.options_immutable().show_symbol() == true &&
			m2.options_immutable().negative_view() == NegativeView::Minus
		);
	}

	#[test]
	fn from_str_paren_no_symbol() {
		let m1 = Money::new(5, 34, MoneySign::Negative).unwrap();
		let m2 = Money::from_str("(5.34)").unwrap();
		
		assert!(
			m1 == m2 &&
			m2.options_immutable().symbol() == '$' &&
			m2.options_immutable().show_symbol() == false &&
			m2.options_immutable().negative_view() == NegativeView::Paren
		);
	}
	
	#[test]
	fn from_str_paren_symbol() {
		let m1 = Money::new(5, 34, MoneySign::Negative).unwrap();
		let m2 = Money::from_str("($5.34)").unwrap();
		
		assert!(
			m1 == m2 &&
			m2.options_immutable().symbol() == '$' &&
			m2.options_immutable().show_symbol() == true &&
			m2.options_immutable().negative_view() == NegativeView::Paren
		);
	}

	#[test]
	fn from_str_pos_diff_symbol() {
		let m1 = Money::new(5, 34, MoneySign::Positive).unwrap();
		let m2 = Money::from_str("£5.34").unwrap();
		
		assert!(
			m1 == m2 &&
			m2.options_immutable().symbol() == '£' &&
			m2.options_immutable().show_symbol() == true
		);
	}
	
	#[test]
	fn invalid_money_cents() {
		match Money::new(5, 101, MoneySign::Positive) {
			Ok(_) => { assert!(false); },
			Err(_) => { assert!(true); },
		}
	}
	
	#[test]
	fn invalid_money_string() {
		match Money::from_str("$a.00") {
			Ok(_) => { assert!(false); },
			Err(_) => { assert!(true); },
		}
	}
	
	#[test]
	fn copy_options() {
		let mut src = Money::new(5, 25, MoneySign::Negative).unwrap();
		let mut dest = Money::new(10, 50, MoneySign::Negative).unwrap();
		
		src.options().set_symbol('#');
		src.options().set_negative_view(NegativeView::Paren);
		
		Money::copy_options(&mut dest, &src);
		
		assert!(
			dest.options_immutable().symbol() == src.options_immutable().symbol() &&
			dest.options_immutable().show_symbol() == src.options_immutable().show_symbol() &&
			dest.options_immutable().negative_view() == src.options_immutable().negative_view()
		);
	}
}