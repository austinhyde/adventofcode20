
fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", part1(input));
    // Correct answer: 801
    println!("Part 2 Answer: {}", part2(input));
    // Correct answer: 597
}

/** Part 1:
    A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".

    The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.

    The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.

    What is the highest seat ID on a boarding pass?
*/
fn part1(input: &str) -> usize {
    *parse_and_sort(input).last().unwrap()
}

/** Part 2:
    Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.

    What is the ID of your seat?
*/
fn part2(input: &str) -> usize {
    let passes = parse_and_sort(input);
    let window = passes.windows(2).find(|window| window[1] - window[0] == 2).unwrap();
    window[0] + 1
}

fn parse_and_sort(input: &str) -> Vec<usize> {
    // insight 1: this is just a disguised 2d coordinate system mapped onto 1d system
    // aka storing a grid in a single vec. the coordinates are in binary
    // so, the example of FBFBBFFRLR => (0101100,101) => (44,5)
    // then mapped with the typical index = y * width + x, gives the seat id 357

    // insight 2:
    // index = y * width + x
    //    id = coord.0 * 8 + coord.1
    //       = seat[0..6] * 8 + seat[7..11]
    //       = (seat >> 3) * 8 + (seat & 7)
    //       = (seat >> 3) << 3 + (seat & 7)
    //       = (seat & ~7) + (seat & 7)
    //       = seat
    //
    // and also note that nothing in the problem actually requires row/col coordinates, just id
    let mut passes = input.lines()
        .map(|l| {
            let l = l
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1");
            usize::from_str_radix(&l, 2).unwrap()
        })
        .collect::<Vec<_>>();
    passes.sort_unstable();
    
    passes
}
