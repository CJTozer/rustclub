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

/// Get the cell-by-cell product of 4 Grid structures
///
/// Returns a Grid with the smallest dimensions of the input set
fn product(g1: Grid, g2: Grid, g3: Grid, g4: Grid) -> Grid {
    let mut product = Grid::new();
    for (((r1, r2), r3), r4) in g1.cells.iter()
                                        .zip(g2.cells.iter())
                                        .zip(g3.cells.iter())
                                        .zip(g4.cells.iter()) {
        // Start a new row
        let mut new_row = Vec::new();
        for (((c1, c2), c3), c4) in r1.iter()
                                          .zip(r2.iter())
                                          .zip(r3.iter())
                                          .zip(r4.iter()) {
            new_row.push(c1 * c2 * c3 * c4);
        }
        product.cells.push(new_row);
    }
    product
}

fn grid_string(g: Grid) -> String {
    let mut s = String::new();
    for row in g.cells {
        for cell in row {
            s = s + &format!(" {}", cell);
        }
        s.push_str("\n");
    }
    s
}

#[test]
#[should_panic]
fn strings_differ() {
    // Check that the two test grids don't start off the same
    let in_path = Path::new("data/test1");
    let exp_path = Path::new("data/test1_a");
    let g1 = Grid::from_file(in_path);
    let g2 = Grid::from_file(exp_path);

    assert_eq!(grid_string(g1),
               grid_string(g2));
}

#[test]
fn x_translate() {
    // Translate the grid by (1, 0)
    let in_path = Path::new("data/test1");
    let exp_path = Path::new("data/test1_10");
    let g1 = Grid::from_file(in_path)._translate(1, 0);
    let g2 = Grid::from_file(exp_path);

    assert_eq!(grid_string(g1),
               grid_string(g2));
}

#[test]
fn y_translate() {
    // Translate the grid by (0, 1)
    let in_path = Path::new("data/test1");
    let exp_path = Path::new("data/test1_01");
    let g1 = Grid::from_file(in_path)._translate(0, 1);
    let g2 = Grid::from_file(exp_path);

    assert_eq!(grid_string(g1),
               grid_string(g2));
}
