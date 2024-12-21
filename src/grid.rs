#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    cells: Vec<bool>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    pub fn randomise_grid(&self) -> Self {
        let mut cells = vec![false; self.width * self.height];
        for cell in cells.iter_mut() {
            *cell = rand::random();
        }

        Self {
            width: self.width,
            height: self.height,
            cells,
        }
    }

    // Internal function to convert 2D coordinates to 1D index
    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        self.cells[self.get_index(x, y)]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: bool) {
        let index = self.get_index(x, y);
        self.cells[index] = value;
    }

    pub fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in [self.height - 1, 0, 1].iter().cloned() {
            for dx in [self.height - 1, 0, 1].iter().cloned() {
                // Don't count the current cell in its neighbors
                if dx == 0 && dy == 0 {
                    continue;
                }

                let x = (x + dx) % self.width;
                let y = (y + dy) % self.height;
                let index = self.get_index(x, y);
                count += self.cells[index] as usize;
            }
        }
        count
    }

    pub fn iterated_grid(&self) -> Grid {
        let mut new_grid = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let count = self.count_neighbors(x, y);
                let index = self.get_index(x, y);
                let cell = self.cells[index];
                new_grid.set_cell(
                    x,
                    y,
                    match count {
                        2 => cell,
                        3 => true,
                        _ => false,
                    },
                );
            }
        }
        new_grid
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = if self.get_cell(x, y) { '◼' } else { '◻' };
                print!("{}", symbol);
            }
            println!();
        }
    }
}
