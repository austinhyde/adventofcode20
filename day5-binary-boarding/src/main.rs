use std::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", part1(input));
    println!("Part 2 Answer: {}", part2(input));
}

/** Part 1:

*/
fn part1(input: &str) -> usize {
    let mut passes = input.lines()
        .map(|l| {
            let l = l
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1");
            let (row, col) = l.split_at(7);
            (usize::from_str_radix(row, 2).unwrap(), usize::from_str_radix(col, 2).unwrap())
        })
        .collect::<Vec<_>>();
    
    passes.sort_by(|a, b| {
        match a.0.partial_cmp(&b.0).unwrap() {
            Ordering::Equal => a.1.partial_cmp(&b.1).unwrap(),
            x => x
        }
    });

    let (row, col) = passes.last().unwrap();
    row * 8 + col
}

/** Part 2:

*/
fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("example1.txt");
        let expected = 0;
        assert_eq!(expected, part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("example1.txt");
        let expected = 0;
        assert_eq!(expected, part2(input));
    }
}