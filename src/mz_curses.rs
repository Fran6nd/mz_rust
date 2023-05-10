mod mz_lib;
fn main() {
    let mut mz = mz_lib::Maze::new(10, 10);
    mz.print();
}
