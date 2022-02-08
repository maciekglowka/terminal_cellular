use rand::{thread_rng, Rng};
use std::cmp::{min, max};
use std::io::Write;

pub struct Board {
    width: usize,
    height: usize,
    pub tiles: Vec<u8>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let v = vec![0; width * height];
        Board{width: width, height: height, tiles: v}
    }
    fn get_tile(self: &Self, x: usize, y: usize) -> u8 {
        self.tiles[x + y * self.width]
    }
    fn set_tile(self: &mut Self, x: usize, y: usize, val: u8) {
        self.tiles[x + y * self.width] = val;
    }
    pub fn randomize(self: &mut Self, num: u32, gen: u32) {
        let mut rng = thread_rng();
        self.tiles = self.tiles.iter().map(|_| if rng.gen_ratio(num, gen) {1} else {0}).collect();
    }
    fn count_neighbours(self: &Self, x: isize, y:isize) -> u8 {
        let mut count = 0;

        for nx in max(0, x-1)..min(x+2, self.width as isize) {
            for ny in max(0, y-1)..min(y+2, self.height as isize) {
                if nx==x && ny==y { continue };
                count += self.get_tile(nx as usize, ny as usize);
            }
        }
        count
    }
    fn replace_tiles(self: &mut Self, new_tiles: Vec<u8>) {
        if new_tiles.len() != self.tiles.len() {
            panic!("Wrong tile count!")
        }
        self.tiles = new_tiles;
    }            
}


pub struct Terminal {
    width: usize,
    height: usize,
    buffer: Vec<char>,
    color: u8
}

impl Terminal {
    pub fn new(width: usize, height: usize, color: u8) -> Terminal {
        let v = vec![' '; width * height];
        Terminal{width: width, height: height, buffer: v, color: color}
    }

    fn draw_buffer(self: &mut Self, data: &Vec<char>) {
        if data.len() != self.buffer.len() {
            panic!("Wrong buffer data count!")
        }
        let mut print_buffer = Vec::<u16>::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = x + y * self.width;
                if self.buffer[idx] == data[idx] { continue };
                let out = format!("\x1B[{};{}H{}",y, x, data[idx] as char);

                print_buffer.extend(out.encode_utf16());
            }
        }

        if print_buffer.len() == 0 { return }
        let s = String::from_utf16(&print_buffer).expect("Cannot create buffer string!");
        self.buffer = data.clone();

        println!("\x1b[38;5;{}m", self.color);
        print!("{}", s);
        std::io::stdout().flush().unwrap();
    }
    pub fn print_tiles(self: &mut Self, tiles: &Vec<u8>) {
        let data: Vec<char> = tiles.into_iter().map(|x| if *x == 1 {'\u{2022}'} else {' '}).collect();
        self.draw_buffer(&data);
    }
}

pub fn close_terminal(height: usize) {
    // reset style
    println!("\x1b[0m");
    // show cursor
    print!("\x1B[?25h\n");
    print!("\x1B[{};0H ", height);
}

pub fn clear_terminal() {
    // clear
    print!("\x1B[2J\n");
    // hide cursor
    print!("\x1B[?25l\n");
}

pub fn automata_step(board: &mut Board, birth: [u8; 9], survive: [u8; 9]) {
    let mut temp_board = Board::new(board.width, board.height);
    for x in 0..board.width {
        for y in 0..board.height {
            let n = board.count_neighbours(x as isize, y as isize);
            if board.get_tile(x, y) == 0 {
                temp_board.set_tile(x, y, birth[n as usize]);
            } else {
                temp_board.set_tile(x, y, survive[n as usize]);
            }
        }
    }
    board.replace_tiles(temp_board.tiles);
}

pub fn rule_from_string(s: &String) -> [u8; 9] {
    let mut rule = [0; 9];
    
    for i in 0..s.len() {
        let d = s[i..i+1].parse::<usize>().expect("Rules can only contain digits!");
        if rule[d] == 0 {
            rule[d] = 1;
        } else {
            panic!("Incorrect rule: {}!", s);
        }
    }

    rule
}
