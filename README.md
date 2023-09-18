Simple money representation.

# Overview

The `Money` type represents `dollars` and `cents` as separate values to ensure precision.
Additionally, it tracks whether the amount is positive or negative with an enum member.

The library supports basic mathematical and relational operations on `Money` types.
A string representation of the `Money` type is available with `.to_string()`.

# Basic Usage

The arguments to `Money::new()` are:
1) dollars (unsigned integer)
2) cents (unsigned integer)
3) sign (MoneyType)

`dollars` and `cents` are absolute values. Whether the entire value is positive or negative is determined by the `sign` argument.

```
# use nmoney::{Money, MoneySign};
let m1 = Money::new(5, 25, MoneySign::Positive).unwrap();    // default string representation: "$5.25"
let m2 = Money::new(12, 96, MoneySign::Negative).unwrap();  // default string representation: "-$12.96"
```

# Options (Customization)

The library supports some basic customization with regard to the string received from the `Money` type.

## Symbol

As the `Money` type was designed around American money representation, the default symbol is the dollar sign ('$'). However, this can be changed via options:

```
# use nmoney::{Money, MoneySign};
let mut m = Money::new(5, 25, MoneySign::Positive).unwrap();
m.options().set_symbol('£');

assert_eq!(m.to_string(), "£5.25");
```

## Show Symbol

The symbol is shown by default. If the symbol is not needed in the string representation, it can be suppressed:

```
# use nmoney::{Money, MoneySign};
let mut m = Money::new(5, 25, MoneySign::Positive).unwrap();
m.options().set_show_symbol(false);

assert_eq!(m.to_string(), "5.25");
```

## Negative View

Negative amounts are indicated with a minus sign by default. The options are:
* Minus (default)
* Parenthesis
* Hide (negation is not indicated)

```
# use nmoney::{Money, MoneySign};
# use nmoney::money::options::NegativeView;
let mut m = Money::new(5, 25, MoneySign::Negative).unwrap();
m.options().set_negative_view(NegativeView::Minus);  // default setting

assert_eq!(m.to_string(), "-$5.25");
```

```
# use nmoney::{Money, MoneySign};
# use nmoney::money::options::NegativeView;
let mut m = Money::new(5, 25, MoneySign::Negative).unwrap();
m.options().set_negative_view(NegativeView::Paren);

assert_eq!(m.to_string(), "($5.25)");
```

```
# use nmoney::{Money, MoneySign};
# use nmoney::money::options::NegativeView;
let mut m = Money::new(5, 25, MoneySign::Negative).unwrap();
m.options().set_negative_view(NegativeView::Hide);

assert_eq!(m.to_string(), "$5.25");
```

# Panics

Panics can occur on addition overflow or subtraction underflow for:
* `+`
* `+=`
* `-`
* `-=`
* `as_cents()`

# License

`nmoney` uses the MIT license.

# Versions

* 1.0.0
    * Improved negation logic
    * Used 'checked_add/sub' traits to monitor for overflow
    * Changed 'positive' in Money type from boolean to enum
    * Implemented 'Error' trait on error types in money.rs
* 0.0.0
    * Initial development.
