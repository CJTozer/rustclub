use std::fs::File;
use std::io::Read;

fn main() {
    // ## This should look like:
    // ## - let mut grid = Grid::new("data/grid");
    // ## - grid.solve();
    let mut grid: Grid;
    let mut data = String::new();
    read_data(&mut data);
}

fn read_data(data: &mut String) {
    // ## unwrap() acts on Result; means panic on fail...
    // ## ...but not with a terribly useful output
    let mut file = File::open("data/grid").unwrap();
    file.read_to_string(data).unwrap();
    println!("We read data: {}", data);
}

struct Grid {
    cells: Vec<Vec<u32>>,
}

impl Grid {
    fn new(file: String) -> Grid {
        Grid { cells: Vec::new() }
    }

    fn solve(&self) {
        println!("Grid.solve not implemented yet");
    }
}
