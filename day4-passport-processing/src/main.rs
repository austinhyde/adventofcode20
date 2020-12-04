use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", part1(input));
    // Correct answer: 170
    println!("Part 2 Answer: {}", part2(input));
}

/** Part 1:
    Passport data is validated in batch files (your puzzle input).
    Each passport is represented as a sequence of key:value pairs separated by spaces or newlines.
    Passports are separated by blank lines.

    Count the number of valid passports - those that have all required fields.
    Treat cid as optional. In your batch file, how many passports are valid?
*/
fn part1(input: &str) -> usize {
    use lib::parse::*;

    string(character().repeat(3)).map_into::<FieldName>()
    .skip(":")
    .then(string(character().up_until(whitespace())))
    .repeat_delimited(Many, " ".or("\n"))
    .repeat_delimited(Many, "\n\n")
    .parse_result(input).unwrap()
    .into_iter()
    .filter(is_valid_passport)
    .count()
}

#[allow(clippy::ptr_arg)]
fn is_valid_passport(passport: &Vec<(FieldName, String)>) -> bool {
    use FieldName::*;
    let required = [BirthYear, IssueYear, ExpirationYear, Height, HairColor, EyeColor, PassportId];
    let mut matches = required.iter().map(|f| (f, false)).collect::<HashMap<_,_>>();
    for (field, _) in passport {
        *matches.entry(field).or_default() = true;
    }
    matches.iter().all(|(_, found)| *found)
}

/** Part 2:

*/
fn part2(input: &str) -> usize {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum FieldName {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
    Unknown(String),
}
impl From<String> for FieldName {
    fn from(s: String) -> Self {
        use FieldName::*;
        match s.as_str() {
            "byr" => BirthYear,
            "iyr" => IssueYear,
            "eyr" => ExpirationYear,
            "hgt" => Height,
            "hcl" => HairColor,
            "ecl" => EyeColor,
            "pid" => PassportId,
            "cid" => CountryId,
            _ => Unknown(s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("example1.txt");
        let expected = 2;
        assert_eq!(expected, part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("example1.txt");
        let expected = 0;
        assert_eq!(expected, part2(input));
    }
}