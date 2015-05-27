extern crate euler11;

use std::path::Path;
use euler11::Grid;

#[test]
#[should_panic]
fn strings_differ() {
    // Check that the two test grids don't start off the same
    let in_path = Path::new("data/test1");
    let exp_path = Path::new("data/test1_a");
    let g1 = Grid::from_file(in_path);
    let g2 = Grid::from_file(exp_path);

    assert_eq!(g1.as_string(),
               g2.as_string());
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

