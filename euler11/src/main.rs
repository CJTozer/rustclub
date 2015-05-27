extern crate euler11;

use std::path::Path;
use euler11::Grid;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let data_path = Path::new("data/grid");
    let grid: Grid = Grid::from_file(data_path);
    println!("Solution: {}", grid.solve());
}

