use lib::grid::Grid;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 Answer: {}", part_1(input));
    // Correct answer: 244

    println!("Part 2 Answer: {}", part_2(input));
    // Correct answer: 9406609920
}

/** Part 1:
    Due to the local geology, trees in this area only grow on exact integer coordinates in a grid.
    You make a map (your puzzle input) of the open squares (.) and trees (#) you can see.

    These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability,
    the same pattern repeats to the right many times

    You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).

    Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
*/
fn part_1(input: &str) -> usize {
    let grid: Grid<Cell> = Grid::from_lines_chars(input, |c| match c { '.' => Empty, _ => Tree});
    count_trees(&grid, (3, 1))
}

/** Part 2:
    Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:

        Right 1, down 1.
        Right 3, down 1. (This is the slope you already checked.)
        Right 5, down 1.
        Right 7, down 1.
        Right 1, down 2.

    What do you get if you multiply together the number of trees encountered on each of the listed slopes?
*/
fn part_2(input: &str) -> usize {
    let grid: Grid<Cell> = Grid::from_lines_chars(input, |c| match c { '.' => Empty, _ => Tree});
    count_trees(&grid, (1, 1))
    * count_trees(&grid, (3, 1))
    * count_trees(&grid, (5, 1))
    * count_trees(&grid, (7, 1))
    * count_trees(&grid, (1, 2))
}

#[derive(Debug, PartialEq)]
enum Cell {
    Empty,
    Tree,
}
use Cell::*;

fn count_trees(grid: &Grid<Cell>, (right, down): (usize, usize)) -> usize {
    // the problem says to start at the top-left, but we already know that's empty
    // so start at the first point instead
    let mut x = right;
    let mut count = 0;
    for y in (down..grid.height()).step_by(down) {
        if grid[(x, y)] == Tree {
            count += 1
        }

        x = (x + right) % grid.width();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("example1.txt");
        assert_eq!(7, part_1(input));
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("example1.txt");
        assert_eq!(336, part_2(input));
    }
}