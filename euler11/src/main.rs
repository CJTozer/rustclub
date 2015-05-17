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
    fn new() -> Grid {
        Grid { 
            cells: Vec::new()
        }
    }

    fn from_file(data_file: String) -> Grid {
        let mut g = Grid::new();
        let mut file = File::open(data_file).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        println!("We read data: {}", data);
        // ## Need to put the data into the cells...
        g
    }

    fn solve(&self) {
        println!("Grid.solve not implemented yet");
    }
}
