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

    fn map_into<U: From<T>>(self) -> BoxedParser<'a, U> where Self: Sized+'a {
        self.map(|v| v.into())
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

    /// runs this parser multiple times (according to `rep`), collecting its results into a Vec
    /// ```
    /// use lib::parse::*;
    /// let twice = "a".or("b").repeat(2);
    /// assert_eq!(twice.parse("ab"), Some((vec!["a", "b"], "")));
    /// assert_eq!(twice.parse("aaaa"), Some((vec!["a", "a"], "aa")));
    /// assert_eq!(twice.parse("a"), None);
    /// 
    /// let any = "a".or("b").repeat(Any);
    /// assert_eq!(any.parse("ab"), Some((vec!["a", "b"], "")));
    /// assert_eq!(any.parse("abaab"), Some((vec!["a", "b", "a", "a", "b"], "")));
    /// assert_eq!(any.parse("xyz"), Some((vec![], "xyz")));
    /// assert_eq!(any.parse(""), Some((vec![], "")));
    /// ```
    fn repeat(self, rep: impl Into<Repetition> + 'a) -> BoxedParser<'a, Vec<T>> where Self: Sized+'a, T:'a {
        self.repeat_delimited(rep, succeed)
    }

    /// consumes instances of this parser delimited by the given separator parser.
    /// ```
    /// use lib::parse::*;
    /// let parser = ["foo", "bar", "baz"].repeat_delimited(Many, ",");
    /// assert_eq!(parser.parse("foo,bar,baz"), Some((vec!["foo", "bar", "baz"], "")));
    /// assert_eq!(parser.parse("foo,bar,baz,"), Some((vec!["foo", "bar", "baz"], ",")));
    /// assert_eq!(parser.parse("foo,bar baz"), Some((vec!["foo", "bar"], " baz")));
    /// assert_eq!(parser.parse("foo,dog,cat"), Some((vec!["foo"], ",dog,cat")));
    /// assert_eq!(parser.parse("dog,cat,bird"), None);
    /// 
    /// let parser = ["a","b"].repeat_delimited(2, ",");
    /// assert_eq!(parser.parse("a,b"), Some((vec!["a","b"], "")));
    /// assert_eq!(parser.parse("a,b,a,b"), Some((vec!["a","b"], ",a,b")));
    /// assert_eq!(parser.parse("a"), None);
    /// assert_eq!(parser.parse("a,c"), None);
    /// 
    /// let parser = ["a", "b"].repeat_delimited(Any, ",");
    /// assert_eq!(parser.parse("a,b"), Some((vec!["a","b"], "")));
    /// assert_eq!(parser.parse("x,y"), Some((vec![], "x,y")));
    /// 
    /// //let parser = "x".repeat_delimited(Any, ["y", "z"]);
    /// //assert_eq!(parser.parse("xyxzx"), Some((vec!["x", "x", "x"], "")))
    /// ```
    fn repeat_delimited<U>(self, rep: impl Into<Repetition> + 'a, sep: impl Parser<'a, U> + 'a) -> BoxedParser<'a, Vec<T>> where Self: Sized+'a, T:'a {
        let rep = rep.into();
        BoxedParser::new(move |input| {
            let mut input = input;
            let mut result = Vec::new();
            let mut is_first = true;

            // until we hit the minimum, we MUST match
            for _ in 0..rep.min() {
                if !is_first {
                    let (_, rest) = sep.parse(input)?;
                    input = rest;
                }

                let (val, rest) = self.parse(input)?;
                input = rest;
                result.push(val);
                is_first = false;
            }

            if rep.met_or_exceeded_by(result.len()) {
                return Some((result, input));
            }

            // keep parsing until we hit the maximum, or we no longer match
            loop {
                let mut i = input;
                if !is_first {
                    if let Some((_, rest)) = sep.parse(input) {
                        i = rest;
                    } else {
                        break
                    }
                }

                if let Some((val, rest)) = self.parse(i) {
                    input = rest;
                    result.push(val);
                    is_first = false;
                } else {
                    break
                }
                if rep.met_or_exceeded_by(result.len()) {
                    break
                }
            }
            Some((result, input))
        })
    }

    // nom calls a similar operation `many_till(a, b)`, but this doesn't return a pair
    /// runs this parser until the other parser matches, but does not consume the second parser's input.
    /// think of it like a non-greedy repeat(Any)
    /// ```
    /// use lib::parse::Parser;
    /// let parser = "abc".up_until("end");
    /// assert_eq!(parser.parse("abcabcend"), Some((vec!["abc", "abc"], "end")));
    /// assert_eq!(parser.parse("abc123end"), None);
    /// assert_eq!(parser.parse("123123end"), None);
    /// assert_eq!(parser.parse(""), None);
    /// assert_eq!(parser.parse("abcendefg"), Some((vec!["abc"], "endefg")));
    /// ```
    fn up_until<U>(self, other: impl Parser<'a, U> + 'a) -> BoxedParser<'a, Vec<T>> where Self: Sized+'a {
        BoxedParser::new(move |input| {
            let mut input = input;
            let mut result = Vec::new();

            // implementation strongly influenced by nom::multi::many_till (but different, and way less verbose)
            loop {
                // first check the second parser, if we match, we're done
                if let Some((_val2, _rest)) = other.parse(input) {
                    return Some((result, input));
                }
                // if we didn't match, run the first parser and collect its result
                let (val, next) = self.parse(input)?;
                result.push(val);
                input = next;
            }
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

/// all BoxedParsers are Parsers
impl<'a, T> Parser<'a, T> for BoxedParser<'a, T> {
    fn parse(&self, input: &'a str) -> Result<'a, T> {
        self.parser.parse(input)
    }
}

/// all functions of &str->Result are Parsers
impl<'a, T, F: Fn(&'a str)->Result<T>> Parser<'a, T> for F {
    fn parse(&self, input: &'a str) -> Result<'a, T> {
        self(input)
    }
}


pub enum Repetition {
    /// Zero times
    Never,
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
        (self.min(), self.max())
    }
    pub fn min(&self) -> usize {
        match *self {
            Never | Any | AtMost(_) => 0,
            Many => 1,
            Exactly(n) | AtLeast(n) | Between(n, _) => n,
        }
    }
    pub fn max(&self) -> Option<usize> {
        match *self {
            Never => Some(0),
            Any | Many | AtLeast(_) => None,
            Exactly(n) | AtMost(n) | Between(_, n) => Some(n),
        }
    }
    pub fn has_max(&self) -> bool {
        self.max().is_some()
    }
    pub fn met_or_exceeded_by(&self, other: usize) -> bool {
        if let Some(n) = self.max() {
            other >= n
        } else {
            // there is no max, so we can never exceed
            false
        }
    }
}

/// number literals can be used as Repetition::Exactly(n)
impl From<usize> for Repetition {
    fn from(n: usize) -> Self { Exactly(n) }
}

/// A Parser that matches any one single character, and returns it
pub fn character<'a>() -> impl Parser<'a, char> {
    move |input: &'a str| input.chars().next().map(|c| (c, &input[c.len_utf8()..]))
}

/// Creates a Parser that matches a specific string and returns it
pub fn literal<'a>(expected: impl AsRef<str>) -> impl Parser<'a, &'a str> {
    move |input: &'a str| {
        let expected = expected.as_ref();
        input
            .get(0..expected.len())
            .filter(|s| *s == expected)
            .map(|s| (s, &input[expected.len()..]))
    }
}

/// all static strings are interpreted as `literal("...")`
impl<'a> Parser<'a, &'a str> for &'static str {
    fn parse(&self, input: &'a str) -> Result<'a, &'a str> {
        literal(self).parse(input)
    }
}

// fails to match anything
fn fail<T>(_: &str) -> Result<'_, T> {
    None
}
// successfully matches zero characters
fn succeed(input: &str) -> Result<'_, ()> {
    Some(((), input))
}

/// slices of strings are Parsers, representing alternatives
impl<'a> Parser<'a, &'a str> for &'a [&'a str] {
    fn parse(&self, input: &'a str) -> Result<'a, &'a str> {
        self.iter()
            .fold(BoxedParser::new(fail), |acc, p| acc.or(literal(p)))
            .parse(input)
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

// string runs the parser, collecting the traversed input into a string, then discarding the parser's internal result
pub fn string<'a, T>(p: impl Parser<'a, T>) -> impl Parser<'a, String> {
    move |input| {
        let (_val, rest) = p.parse(input)?;
        let consumed_len = input.len()-rest.len();
        Some((input[0..consumed_len].to_string(), rest))
    }
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