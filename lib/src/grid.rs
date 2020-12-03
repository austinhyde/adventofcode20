
pub struct Grid<T> {
    cells: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn from_lines_chars(s: &str, parse: impl Fn(char)->T) -> Self {
        Self {
            cells: s
                .lines()
                .map(|l| {
                    l.chars().map(&parse).collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn width(&self) -> usize {
        self.cells[0].len()
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y][x]
    }
}