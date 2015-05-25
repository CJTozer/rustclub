use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let data_path = Path::new("data/grid");
    let grid: Grid = Grid::from_file(data_path);
    grid.solve();
}

struct Grid {
    cells: Vec<Vec<u32>>,
}

impl Grid {
    /// Simple initializer - empty grid
    fn new() -> Grid {
        Grid {
            cells: Vec::new()
        }
    }

    /// Create new grid from file
    fn from_file(path: &Path) -> Grid {
        let mut g = Grid::new();
        let file = File::open(path).unwrap();
        let buf = BufReader::new(file);
        for line in buf.lines() {
            let mut row = Vec::new();
            for cell in line.unwrap().split(" ") {
                row.push(cell.parse::<u32>().unwrap());
            }
            g.cells.push(row);
        }
        g
    }

    /// Solve [the problem](https://projecteuler.net/problem=11), by finding
    /// the largest product of 4 consecutive numbers in a row, column or
    /// diagonal
    fn solve(&self) -> u32 {
        // TODO
        &self._max();
        println!("Grid.solve not implemented yet");
        123
    }

    /// Get the maximum value in the grid
    fn _max(&self) -> u32 {
        let mut max: u32 = 0;
        for row in &self.cells {
            for cell in row {
                if *cell > max {
                    max = *cell;
                }
            }
        }
        max
    }

    /// Get the cell-by-cell product of this Grid and another (smaller) one
    fn _product(&self, g: Grid) -> Grid {
        let product = Grid::new();
        // TODO
        println!("Grid.product not implemented yet");
        product
    }

    /// 'Translate' a grid
    fn _translate(&self, x: u32, y: u32) -> Grid {
        let mut translated = Grid::new();
        let mut drop_x = x;
        for row in &self.cells {
            if drop_x > 0 {
                // Drop this row
                drop_x -= 1;
            } else {
                // Keep this row
                let mut drop_y = y;
                let mut new_row = Vec::new();
                for cell in row {
                    if drop_y > 0 {
                        // Drop this cell
                        drop_y -= 1;
                    } else {
                        // Keep this cell
                        new_row.push(*cell);
                    }
                }
                translated.cells.push(new_row);
            }
        }
        translated
    }
}
