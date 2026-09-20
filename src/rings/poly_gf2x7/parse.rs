use super::{MAX_DEGREE, PolyGF2x7};
use crate::{fields::gf2::GF2Field, monoid::Monoid, ring::Ring};
use std::iter::Peekable;

struct ParsedTerm<T> {
    coeff: T,
    inverse: bool,
    exp: usize,
}

/// Parse ASCII digits consuming them from `chars` iterator.
///
/// Returns `Ok(None)` if no digits are present
///
/// # Errors
///
/// Returns an error if integer represented doesn't fit into `usize`.
fn parse_decimal(chars: &mut Peekable<std::str::Chars<'_>>) -> Result<Option<usize>, &'static str> {
    let mut dec = 0usize;

    if !chars.peek().is_some_and(|s| s.is_ascii_digit()) {
        return Ok(None);
    }

    while chars.peek().is_some_and(|s| s.is_ascii_digit()) {
        let digit = chars.next().unwrap().to_digit(10).unwrap() as usize;
        dec = dec
            .checked_mul(10)
            .and_then(|n| n.checked_add(digit))
            .ok_or("decimal overflow")?;
    }

    Ok(Some(dec as usize))
}

fn parse_term(t: &str) -> Result<ParsedTerm<GF2Field>, &'static str> {
    // [sign][coeff]x[^exp] | [sign]coeff
    let mut chars = t.chars().peekable();
    let mut parsed = ParsedTerm {
        coeff: GF2Field::zero(),
        inverse: false,
        exp: 0,
    };

    // consume optional sign
    if chars.peek().is_some_and(|s| *s == '-' || *s == '+') {
        parsed.inverse = chars.next() == Some('-');
    }

    // if it's a coeff
    // TODO: imagine allowing rings to not be bound to decimal repr?
    let coeff_dec = parse_decimal(&mut chars)?;

    // BAD: we are (in the parser of all places) mapping decimal to field elems
    parsed.coeff = GF2Field::new(coeff_dec.unwrap_or(1) != 0);

    // [sign]coeff
    if chars.peek().is_none() {
        if coeff_dec.is_none() {
            return Err("Parsing error: sign alone is not a valid term");
        }
        return Ok(parsed);
    }

    // next char should be an x
    if chars.next() != Some('x') {
        return Err("Parsing error: expected x");
    }

    // [sign][coeff]x
    if chars.peek().is_none() {
        parsed.exp = 1;
        return Ok(parsed);
    }

    if chars.next() != Some('^') {
        return Err("Parsing error: expected ^");
    }

    parsed.exp = parse_decimal(&mut chars)?.ok_or("Parsing error: expected number after ^")?;

    if chars.peek().is_some() {
        return Err("Parse error: characters not allowed after decimal in exponent");
    }

    Ok(parsed)
}

impl std::str::FromStr for PolyGF2x7 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bin_coeffs = 0u8;

        let mut negative = false;
        for term in s.split_whitespace() {
            let mut parsed_term: ParsedTerm<GF2Field>;
            match term {
                "+" => {
                    negative = false;
                    continue;
                }
                "-" => {
                    negative = true;
                    continue;
                }
                t => {
                    parsed_term = parse_term(t)?;
                    parsed_term.inverse ^= negative;

                    // adding x^7 is a no op since we work modulo x^7
                    if parsed_term.exp > MAX_DEGREE {
                        continue;
                    }
                    bin_coeffs ^= u8::from(parsed_term.coeff == GF2Field::one()) << parsed_term.exp;
                }
            };
        }

        Ok(Self(bin_coeffs))
    }
}
