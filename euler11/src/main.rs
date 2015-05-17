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
    fn new() -> Grid {
        Grid { 
            cells: Vec::new()
        }
    }

    fn from_file(path: &Path) -> Grid {
        let mut g = Grid::new();
        let file = File::open(path).unwrap();
        let buf = BufReader::new(file);
        for line in buf.lines() {
            let mut row = Vec::new();
            for cell in line.unwrap().split(" ") {
                println!("Cell: {}", cell);
                row.push(cell.parse::<u32>().unwrap());
            }
            g.cells.push(row);
        }
        //let mut file = File::open(path).unwrap();
        //let mut data = String::new();
        //file.read_to_string(&mut data).unwrap();
        // println!("We read data: {}", data);
        // ## Need to put the data into the cells...
        g
    }

    fn solve(&self) {
        println!("Grid.solve not implemented yet");
    }
}
