pub mod maze;

fn main() {
    let m = maze::Maze::new(11);

    m.print();
}
