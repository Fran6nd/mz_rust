mod mz_lib;
fn main() {
    let mut mz = mz_lib::Maze::new(20, 20);
    mz.print();
}
