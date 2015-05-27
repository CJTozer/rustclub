use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::cmp;

pub struct Grid {
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
    pub fn from_file(path: &Path) -> Grid {
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
    pub fn solve(&self) -> u32 {
        // Check East
        let max_e = product(&self,
                            &self.translate(1, 0),
                            &self.translate(2, 0),
                            &self.translate(3, 0)).max();

        // Check South
        let max_s = product(&self,
                            &self.translate(0, 1),
                            &self.translate(0, 2),
                            &self.translate(0, 3)).max();

        // Check South-East
        let max_se = product(&self,
                             &self.translate(1, 1),
                             &self.translate(2, 2),
                             &self.translate(3, 3)).max();

        // Check North-East
        let max_ne = product(&self.translate(0, 3),
                             &self.translate(1, 2),
                             &self.translate(2, 1),
                             &self.translate(3, 0)).max();

        cmp::max(cmp::max(max_e, max_s),
                 cmp::max(max_se, max_ne))
    }

    /// Get the maximum value in the grid
    fn max(&self) -> u32 {
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
    fn translate(&self, x: u32, y: u32) -> Grid {
        let mut translated = Grid::new();
        let mut drop_y = y;
        for row in &self.cells {
            if drop_y > 0 {
                // Drop this row
                drop_y -= 1;
            } else {
                // Keep this row
                let mut drop_x = x;
                let mut new_row = Vec::new();
                for cell in row {
                    if drop_x > 0 {
                        // Drop this cell
                        drop_x -= 1;
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

    /// Script helper to give an easy comparison of Grids in UTs
    pub fn as_string(&self) -> String {
        let mut s = String::new();
        for row in &self.cells {
            for cell in row {
                s = s + &format!(" {}", cell);
            }
            s.push_str("\n");
        }
        s
    }
}

/// Get the cell-by-cell product of 4 Grid structures
///
/// Returns a Grid with the smallest dimensions of the input set
fn product(g1: &Grid, g2: &Grid, g3: &Grid, g4: &Grid) -> Grid {
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

#[test]
fn x_translate() {
    // Translate the grid by (1, 0)
    let in_path = Path::new("data/test1");
    let exp_path = Path::new("data/test1_10");
    let g1 = Grid::from_file(in_path).translate(1, 0);
    let g2 = Grid::from_file(exp_path);

    assert_eq!(g1.as_string(),
               g2.as_string());
}

#[test]
fn y_translate() {
    // Translate the grid by (0, 1)
    let in_path = Path::new("data/test1");
    let exp_path = Path::new("data/test1_01");
    let g1 = Grid::from_file(in_path).translate(0, 1);
    let g2 = Grid::from_file(exp_path);

    assert_eq!(g1.as_string(),
               g2.as_string());
}

#[test]
fn xy_translate() {
    // Translate the grid by (2, 3)
    let in_path = Path::new("data/test1");
    let exp_path = Path::new("data/test1_23");
    let g1 = Grid::from_file(in_path).translate(2, 3);
    let g2 = Grid::from_file(exp_path);

    assert_eq!(g1.as_string(),
               g2.as_string());
}

