/**
    largely inspired by https://bodil.lol/parser-combinators/# and nom
    but nom is cheating :P

    this all looks suspiciously similar to the bodil implementation,
    but that's because you can't actually get much simpler than that. in all honesty,
    I started implementing this from first principles, then remembered that bodil article
    existed, and looked at it, and was like, "huh, okay, guess I'm on the right track".

    after that, I wound up just using bodil as reference with some input from nom

    my goal with this is to be ergonomic and simple, so I can solve problems and
    not get hung up on parsing. also to practice my rust-fu.
*/

/*
    bodil's article uses Result<(rest, result), input>, but I think that's a bit overkill
    for this quick and dirty lib, then you have to juggle error types everywhere, it's messy.

    plus, the order of the tuple seems weird to me... if you match "a" in "abc", my mental model
    is that we split to "a" and "bc", which is (result, rest) instead.

    so, we'll simplify, and say parse::Result<T> is Option<(result, rest)>. None means we couldn't parse.
*/
type Result<'a, T> = Option<(T, &'a str)>;

/// Represents something capable of producing a T from an input string
/// Call parse_result to get Some(T) if the parse succeeded or None if not
pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> Result<'a, T>;

    fn parse_result(&self, input: &'a str) -> Option<T> {
        self.parse(input).map(|(val, _)| val)
    }

    /// transforms the results of this parser
    fn map<U>(self, f: impl Fn(T)->U + 'a) -> BoxedParser<'a, U> where Self: Sized+'a {
        BoxedParser::new(move |input| {
            let (val, rest) = self.parse(input)?;
            Some((f(val), rest))
        })
    }

    // nom calls this `a.flat_map(f)`, bodil calls this `and_then(a, f)`, haskell calls it `a >>= f`,
    // and this makes Parser a monoid in the category of endofunctors, as the kids say
    /// uses the results of this parser to make a new one.
    fn and_then<U, P2: Parser<'a, U>>(self, f: impl Fn(T) -> P2 + 'a) -> BoxedParser<'a, U> where Self: Sized+'a {
        BoxedParser::new(move |input| {
            let (val, rest) = self.parse(input)?;
            f(val).parse(rest)
        })
    }

    // nom calls this `a.and_then(b)` or `terminated(a, b)`, bodil calls this `left(a, b)`, but I think neither of those is intuitive
    // also, haskell calls this `a >> b`
    /// runs this, and skips over the results of the given Parser. Opposite of `but_really`. `a.skip(b)` yields `A`
    fn skip<U>(self, second: impl Parser<'a, U> + 'a) -> BoxedParser<'a, T> where Self: Sized+'a {
        BoxedParser::new(move |input| {
            let (val1, next) = self.parse(input)?;
            let (_val2, rest) = second.parse(next)?;
            Some((val1, rest))
        })
    }

    // nom calls this `preceded(a, b)`, bodil calls this `right(a, b)`
    /// throws away this result, and keeps the second. Opposite of `skip`. `a.but_really(b)` yields `b`
    fn but_really<U>(self, second: impl Parser<'a, U> + 'a) -> BoxedParser<'a, U> where Self: Sized+'a {
        BoxedParser::new(move |input| {
            let (_val1, next) = self.parse(input)?;
            let (val2, rest) = second.parse(next)?;
            Some((val2, rest))
        })
    }

    // nom calls this `a.and(b)`, bodil calls this `pair(a, b)` but neither makes sense as a method name
    /// captures the results of a second parser in a tuple of (T, U). `a.then(b)` yields `(a, b)`
    fn then<U>(self, second: impl Parser<'a, U> + 'a) -> BoxedParser<'a, (T, U)> where Self: Sized + 'a {
        BoxedParser::new(move |input| {
            let (val1, next) = self.parse(input)?;
            let (val2, rest) = second.parse(next)?;
            Some(((val1, val2), rest))
        })
    }

    /// falls back to a second parser if the first doesn't pan out
    fn or(self, second: impl Parser<'a, T> + 'a) -> BoxedParser<'a, T> where Self: Sized + 'a {
        BoxedParser::new(move |input| {
            self.parse(input).or_else(|| second.parse(input))
        })
    }

    /// rejects the result of this parser if it doesn't match the predicate
    fn filter(self, f: impl Fn(&T)->bool + 'a) -> BoxedParser<'a, T>  where Self: Sized+'a {
        BoxedParser::new(move |input| {
            let (val, rest) = self.parse(input)?;
            if f(&val) {
                Some((val, rest))
            } else {
                None
            }
        })
    }

    fn repeat(self, rep: Repetition) -> BoxedParser<'a, Vec<T>> where Self: Sized+'a {
        let (min, max) = rep.range();
        BoxedParser::new(move |input| {
            let mut input = input;
            let mut result = Vec::new();

            // until we hit the minimum, we MUST match
            for _ in 0..min {
                if let Some((val, rest)) = self.parse(input) {
                    input = rest;
                    result.push(val);
                } else {
                    return None
                }
            }

            if let Some(max) = max {
                if result.len() > max {
                    return Some((result, input))
                }
            }

            // keep parsing until we hit the maximum, or we no longer match
            while let Some((val, rest)) = self.parse(input) {
                input = rest;
                result.push(val);
                if let Some(max) = max {
                    if result.len() >= max {
                        break
                    }
                }
            }

            Some((result, input))
        })
    }
}

pub struct BoxedParser<'a, T> {
    parser: Box<dyn Parser<'a, T> + 'a>,
}

impl<'a, T> BoxedParser<'a, T> {
    fn new(parser: impl Parser<'a, T> + 'a) -> Self {
        Self { parser: Box::new(parser) }
    }
}

impl<'a, T> Parser<'a, T> for BoxedParser<'a, T> {
    fn parse(&self, input: &'a str) -> Result<'a, T> {
        self.parser.parse(input)
    }
}

/// all functions of &str->Result are parsers
impl<'a, T, F: Fn(&'a str)->Result<T>> Parser<'a, T> for F {
    fn parse(&self, input: &'a str) -> Result<'a, T> {
        self(input)
    }
}


pub enum Repetition {
    /// Zero or more times
    Any,
    /// One or more times
    Many,
    /// Exactly this many times
    Exactly(usize),
    /// Any number of times, but at least this many
    AtLeast(usize),
    /// At most this many times, even zero
    AtMost(usize),
    /// Between this many times (inclusive)
    Between(usize, usize),
}
pub use Repetition::*;

impl Repetition {
    pub fn range(&self) -> (usize, Option<usize>) {
        match self {
            Any => (0, None),
            Many => (1, None),
            Exactly(n) => (*n, Some(*n)),
            AtLeast(n) => (*n, None),
            AtMost(n) => (0, Some(*n)),
            Between(min, max) => (*min, Some(*max))
        }
    }
}


/// A Parser that matches any one single character, and returns it
pub fn character<'a>() -> impl Parser<'a, char> {
    move |input: &'a str| input.chars().next().map(|c| (c, &input[c.len_utf8()..]))
}

/// Creates a Parser that matches a specific string and returns it
pub fn literal<'a>(expected: &'static str) -> impl Parser<'a, &'a str> {
    move |input: &'a str| input
        .get(0..expected.len())
        .filter(|s| *s == expected)
        .map(|s| (s, &input[expected.len()..]))
}

/// all static strings are interpreted as a literal parser
impl<'a> Parser<'a, &'a str> for &'static str {
    fn parse(&self, input: &'a str) -> Result<'a, &'a str> {
        literal(self).parse(input)
    }
}

pub fn digit<'a>() -> impl Parser<'a, char> {
    character().filter(|c| c.is_numeric())
}

// 'u32' is already taken, guess we'll be more verbose
pub fn uint32<'a>() -> impl Parser<'a, u32> {
    digit()
        .repeat(Many)
        .map(|ds| ds.iter().fold(0, |n, d| n * 10 + d.to_digit(10).unwrap()))
}

pub fn whitespace<'a>() -> impl Parser<'a, char> {
    character().filter(|c| c.is_whitespace())
}

pub fn word<'a>() -> impl Parser<'a, String> {
    character()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .repeat(Many)
        .map(|v| v.into_iter().collect())
}

pub fn identifier<'a>() -> impl Parser<'a, String> {
    character()
        .filter(|c| c.is_alphabetic() || *c == '_')
        .then(word())
        .map(|(c, s)| c.to_string() + &s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_parse_chained() {
        let input = "1-3 a: abcdef";

        let parser =
            uint32()
            .skip("-")
            .then(uint32())
            .skip(whitespace().repeat(Any))
            .then(character())
            .skip(":")
            .skip(whitespace().repeat(Any))
            .then(word());
        
        let (((min, max), letter), password) = parser.parse_result(input).unwrap();
        assert_eq!(1, min);
        assert_eq!(3, max);
        assert_eq!('a', letter);
        assert_eq!("abcdef", password);
    }
}