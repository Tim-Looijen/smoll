use std::fmt;

pub struct Grid2D<T> {
    pub width: usize,
    pub height: usize,
    pub coordinates: Vec<usize>,

    grid: Box<[T]>,
}

impl Grid2D<char> {
    pub fn new(input: &Vec<String>) -> Self {
        let mut grid_vec: Vec<char> = Vec::new();
        for row in input {
            for cell in row.chars() {
                grid_vec.push(cell)
            }
        }

        let height = input.len();
        let width = grid_vec.len() / height;

        let grid = grid_vec.into_boxed_slice();
        let coordinates = grid.iter().enumerate().map(|(i, _)| i).collect();
        Self {
            height,
            width,
            grid,
            coordinates,
        }
    }
}

impl<T: Clone> Grid2D<T> {
    pub fn filled(width: usize, height: usize, value: T) -> Self {
        let grid = vec![value; width * height].into_boxed_slice();
        let coordinates = grid.iter().enumerate().map(|(i, _)| i).collect();
        Self {
            width,
            height,
            grid,
            coordinates,
        }
    }
}

impl<T: Clone> Grid2D<T> {
    pub fn get_grid_ref(&self) -> &Box<[T]> {
        &self.grid
    }
}

impl<T> Grid2D<T> {
    pub fn row(&self, y: usize) -> &[T] {
        let width = self.width;
        let start = y * width;
        &self.grid[start..start + width]
    }

    pub fn col(&self, x: usize) -> impl DoubleEndedIterator<Item = &T> {
        self.grid[x..].iter().step_by(self.width)
    }

    pub fn row_indices(&self, y: usize) -> impl DoubleEndedIterator<Item = usize> {
        let width = self.width;
        let start = y * width;
        start..start + width
    }

    pub fn col_indices(&self, x: usize) -> impl DoubleEndedIterator<Item = usize> {
        (x..self.grid.len()).step_by(self.width)
    }
}

impl<T> Grid2D<T> {
    const DIRS4: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    const DIRS8: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    pub fn neighbors_4(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.neighbors(x, y, &Self::DIRS4)
    }

    pub fn neighbors_8(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.neighbors(x, y, &Self::DIRS8)
    }

    fn neighbors(
        &self,
        x: usize,
        y: usize,
        dirs: &'static [(i32, i32)],
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        dirs.iter().filter_map(move |&(dx, dy)| {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            self.in_bounds(nx, ny).then_some((nx as usize, ny as usize))
        })
    }
}

impl<T> Grid2D<T> {
    pub fn at(&self, i: usize) -> &T {
        &self.grid[i]
    }

    pub fn at_xy(&self, x: usize, y: usize) -> &T {
        &self.grid[self.to_i(x, y)]
    }

    pub fn to_i(&self, x: usize, y: usize) -> usize {
        (y * self.width + x) as usize
    }

    pub fn to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y)
    }

    pub fn update_at(&mut self, x: usize, y: usize, value: T) {
        let i = self.to_i(x, y);
        self.grid[i] = value;
    }
}

impl<T> Grid2D<T> {
    fn fmt_with(&self, f: &mut fmt::Formatter, cell: impl Fn(&T) -> char) -> fmt::Result {
        let biggest = self.width.max(self.height).saturating_sub(1);
        let w = biggest.to_string().len();

        writeln!(f)?;
        write!(f, "{:w$}  ", "")?;
        for x in 0..self.width {
            write!(f, " {:>w$}", x)?;
        }
        writeln!(f)?;

        write!(f, "{:w$} ┌", "")?;
        for _ in 0..self.width * (w + 1) {
            write!(f, "─")?;
        }
        writeln!(f)?;

        for y in 0..self.height {
            write!(f, "{:>w$} │", y)?;
            for c in self.row(y) {
                write!(f, " {:>w$}", cell(c))?;
            }
            if y + 1 < self.height {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Grid2D<char> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with(f, |c| *c)
    }
}

impl fmt::Debug for Grid2D<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with(f, |c| if *c { 'x' } else { '.' })
    }
}
