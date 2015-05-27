extern crate euler11;

use std::path::Path;
use euler11::Grid;

#[test]
fn solve_the_problem() {
    let path = Path::new("data/grid");
    let g = Grid::from_file(path);

    assert_eq!(70600674, g.solve());
}
