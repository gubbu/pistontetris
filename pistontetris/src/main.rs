extern crate piston_window;
mod gamewindow;

const BLOCKSIZE: f64 = 10.0;
const GAMEWIDTH: usize = 50;
const GAMEHEIGHT: usize = 60;
const STARTGAME: Tetrisgame = Tetrisgame {
    field: [[[0.0; 3]; GAMEWIDTH]; GAMEHEIGHT],
    currentshapeindex: 0,
    coordinate: (0, 0),
    dirx: 0,
    currentcolor: [1.0; 3],
    timer: 0.0,
};
const SECONDSPERTICK: f64 = 0.125;
//16*16 Shapes:
const SHAPES: [[u16; 16]; 2] = [
    [
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
        0b0000000110000000,
    ],
    [
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        0b1000000000000000,
        !0,
    ],
];
//simple demo of the two modules gamewindow and fontrender
struct Tetrisgame {
    //the game field: a list of colored blocks:
    field: [[[f32; 3]; GAMEWIDTH]; GAMEHEIGHT],
    currentshapeindex: usize,
    dirx: i32,
    coordinate: (i32, i32),
    currentcolor: [f32; 3],
    timer: f64,
}

//the possible reactions when a tetris Block moves
#[derive(PartialEq)]
enum Reaction {
    //it halts and gets added to the obstacle list
    Halt,
    //it gets blocked by the left or right screen borders
    BlockX,
    //it can move unhindered
    Normalmove,
}

impl Tetrisgame {
    //returns true if the given move is possible
    fn mov(&self, dirx: i32, diry: i32) -> bool {
        for (y, row) in SHAPES[self.currentshapeindex].iter().enumerate() {
            for x in (0..16) {
                if row & (1 << x) != 0 {
                    let x: i32 = 15i32 - (x as i32) + self.coordinate.0 + dirx;
                    let ycoor: i32 = y as i32 + self.coordinate.1 + diry;
                    if ycoor < 0 || ycoor >= GAMEHEIGHT as i32 {
                        return false;
                    } else if x < 0 || x >= GAMEWIDTH as i32 {
                        return false;
                    } else if self.field[ycoor as usize][x as usize] != [0.0; 3] {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn tick(&mut self, dirx: i32) {
        //can be triggered by a movement that only was created by x-Axis movement
        if self.mov(1, 0) {
            self.coordinate.0 += 1;
        }
        if self.mov(0, 1) {
            self.coordinate.1 += 1;
        } else {
            self.addobstacle();
            self.coordinate = (0, 0);
            self.currentshapeindex += 1;
            self.currentshapeindex %= SHAPES.len();
        }
    }

    fn addobstacle(&mut self) {
        println!("adding obstacle");
        for (y, row) in SHAPES[self.currentshapeindex].iter().enumerate() {
            for x in (0..16).rev() {
                if row & (1 << x) != 0 {
                    let x: usize = (15 - (x as i32) + self.coordinate.0) as usize;
                    let ycoor: usize = y + self.coordinate.1 as usize;
                    self.field[ycoor][x] = self.currentcolor;
                    //println!("setting {}/{}",)
                }
            }
        }
    }
}

//const MESSAGE: String = String::from("HELLO WORLD!+-*/ WORLD: HOW ARE YOU\nI AM NOT OK.");
//implementing the gametrait for Tetrisgame.
impl gamewindow::Gametrait for Tetrisgame {
    fn onstart(&mut self) {
        println!("starting")
    }

    fn update(&mut self, dt: f64) {
        self.timer += dt;
        if self.timer >= SECONDSPERTICK {
            self.timer = 0.0;
            //println!("self.dirx{} {:?}", self.dirx, self.coordinate);
            self.tick(self.dirx);
            //self.dirx = 0;
        }
        //color update yay
    }

    fn render(&self, g: &mut piston_window::G2d, transform: [[f64; 3]; 2]) {
        //println!("transform {:?}", transform);
        //draw the map
        for (y_coor, row) in self.field.iter().enumerate() {
            for (x_coor, color) in row.iter().enumerate() {
                piston_window::rectangle(
                    [color[0], color[1], color[2], 1.0],
                    [
                        (x_coor as f64) * BLOCKSIZE,
                        (y_coor as f64) * BLOCKSIZE,
                        BLOCKSIZE * 0.5,
                        BLOCKSIZE * 0.5,
                    ],
                    transform,
                    g,
                );
                //println!("drawing at {} {}", (x_coor as f64) * BLOCKSIZE,
                //       (y_coor as f64) * BLOCKSIZE);
            }
        }
        //draw the current shape
        for (y, row) in SHAPES[self.currentshapeindex].iter().enumerate() {
            for x in 0..16 {
                if (row & (1 << x)) != 0 {
                    let x_coor: f64 = (((15 - x) as i32 + self.coordinate.0) as f64) * BLOCKSIZE;
                    let y_coor: f64 = (((y as i32) + self.coordinate.1) as f64) * BLOCKSIZE;
                    piston_window::rectangle(
                        [
                            self.currentcolor[0],
                            self.currentcolor[1],
                            self.currentcolor[2],
                            1.0,
                        ],
                        [x_coor, y_coor, BLOCKSIZE * 0.5, BLOCKSIZE * 0.5],
                        transform,
                        g,
                    );
                }
            }
        }
        //piston_window::clear([0.0; 4], g);
    }

    fn mousedown(&mut self, down: bool, isleft: bool, x: f64, y: f64) {
        if !down && isleft {
            println!("left mousebutton is released at {}/{}", x, y);
        }
    }

    fn mousemotions(&mut self, _x: f64, _y: f64) {}

    fn shouldquit(&self) -> bool {
        return false;
    }
    fn onquit(&mut self) {
        println!("QUITING!");
    }
    fn keyboard(&mut self, ispressed: bool, keychar: char) {
        if !ispressed {
            if keychar == 'A' {
                println!("A is released");
                self.dirx = -1;
            } else if keychar == 'D' {
                println!("D is released");
                self.dirx = 1;
            }
        }
    }
}

fn main() {
    gamewindow::makegame(
        "hello world",
        [
            GAMEWIDTH as u32 * BLOCKSIZE as u32,
            GAMEHEIGHT as u32 * BLOCKSIZE as u32,
        ],
        15,
        15,
        &mut Tetrisgame { ..STARTGAME },
    );
}
