use std::thread;
use std::time::Duration;

mod grid;
use crate::grid::Grid;

fn main() {
    let mut iteration = 0;
    let mut grid = Grid::new(50, 50).randomise_grid();
    grid.print();
    thread::sleep(Duration::from_secs(2));
    loop {
        iteration += 1;
        let new_grid = grid.iterated_grid();
        print!("\x1B[2J\x1B[H"); // Clear screen and move cursor to top-left
        new_grid.print();
        thread::sleep(Duration::from_millis(30));
        if new_grid == grid {
            println!("Stable configuration reached after {iteration} iterations.");
            break;
        }
        if iteration == 1500 {
            println!("Maximum iterations reached.");
            break;
        }
        grid = new_grid;
    }
}
