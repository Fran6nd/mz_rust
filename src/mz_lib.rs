struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
}
pub struct Maze {
    height: i32,
    width: i32,
    data: Vec<char>,
}
impl Maze {
    fn in_maze(&self, pos: &Point) -> bool {
        if (pos.x < 0 || pos.y < 0 || pos.x >= self.width || pos.y >= self.height) {
            return false;
        }
        true
    }
    fn how_many_path_neighbourgh(&self, pos: &Point) -> u8 {
        let mut output: u8 = 0;
        if !self.in_maze(pos) {
            println!("not in maze");
            return 0;
        }
        if (self.get_at(&pos.up()) == ' ') {
            output = output + 1;
        }
        if (self.get_at(&pos.down()) == ' ') {
            output = output + 1;
        }
        if (self.get_at(&pos.right()) == ' ') {
            output = output + 1;
        }
        if (self.get_at(&pos.left()) == ' ') {
            output = output + 1;
        }

        output
    }
    fn can_go_up(&self, pos: &Point) -> bool {
        if (!self.in_maze(&pos.up())) {
            return false;
        }
        true
    }
    fn get_at(&self, pos: &Point) -> char {
        if (self.in_maze(pos) == false) {
            return ('#');
        }
        return self.data[(pos.x * self.width + pos.y) as usize];
    }
    fn set_at(&mut self, pos: &Point, value: char) -> () {
        self.data[(pos.x * self.width + pos.y) as usize] = value;
    }
    fn path(&mut self, pos: &Point) -> bool {
        self.set_at(pos, ' ');
        let t = [&pos.up(), &pos.right(), &pos.left(), &pos.down()];
        for tt in t {
            if self.in_maze(tt) {
                if self.how_many_path_neighbourgh(tt) <= 1 {
                    println!("in_maze");
                    self.path(tt);
                }
            }
        }

        true
    }
    pub fn new(x: i32, y: i32) -> Self {
        let mut data: Vec<char> = Vec::new();
        for _x in 0..x {
            for _y in 0..y {
                data.push('#');
            }
        }
        Self {
            height: y,
            width: x,
            data: data,
        }
    }
    pub fn print(&mut self) -> () {
        self.path(&Point { x: 3, y: 3 });
        for i in 0..self.data.len() {
            print!("{}", self.data[i]);
            if ((i + 1) as i32 % self.width == 0) {
                println!("");
            }
        }
    }
}
