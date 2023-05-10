use rand::Rng;

#[derive(Copy, Clone)]
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
    pub fn up_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    pub fn down_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
    pub fn up_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    pub fn down_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y + 1,
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
        if (self.get_at(&pos.up_left()) == ' ') {
            output = output + 1;
        }
        if (self.get_at(&pos.down_left()) == ' ') {
            output = output + 1;
        }
        if (self.get_at(&pos.up_right()) == ' ') {
            output = output + 1;
        }
        if (self.get_at(&&pos.down_right()) == ' ') {
            output = output + 1;
        }

        output
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
        let mut possibilities: Vec<Point> = Vec::new();
        if (pos.x % 2) == 0 {
            possibilities.push(pos.up());
            possibilities.push(pos.down());
        }
        if (pos.y % 2) == 0 {
            possibilities.push(pos.right());
            possibilities.push(pos.left());
        }
        /* Let's put all possibilities in a random order. */
        let mut rng = rand::thread_rng();
        /* Empty dest: */
        let mut possibilities_randomized: Vec<Point> = Vec::new();
        /* While something in possibilities, we move an element randomly to possibilities_randomized. */
        while (possibilities.len() > 0) {
            let index = rng.gen_range(0..possibilities.len());
            possibilities_randomized.push(possibilities[index]);
            possibilities.remove(index);
        }
        /* Now possibilities are randomly sorted. */
        for tt in possibilities_randomized {
            if self.in_maze(&tt) {
                if self.get_at(&tt) != ' ' {
                    if self.how_many_path_neighbourgh(&tt) < 3 {
                        self.path(&tt);
                    }
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
        self.path(&Point { x: 4, y: 4 });
        for i in 0..self.data.len() {
            if self.data[i] == '#' {
                print!("{}", self.data[i]);
            } else {
                print!("+");
            }

            if ((i + 1) as i32 % self.width == 0) {
                println!("");
            }
        }
    }
}
