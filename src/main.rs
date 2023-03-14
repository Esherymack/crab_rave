use rand::Rng;
use terminal_size::{Width, Height, terminal_size};
use clearscreen;
use crossterm::cursor::MoveTo;
use std::{thread, time};
// use console::Term;

#[derive(Debug)]
struct Coordinate {
    x: u16,
    y: u16
}

#[derive(Debug)]
struct Crab {
    crab: String,
    coordinate: Coordinate
}

impl Crab {
    fn walk_up(&mut self, max: u16) {
        if self.coordinate.y <= 0 {
           self.coordinate.y = max - 1; 
        }
        else {
            self.coordinate.y -= 1
        }
    }

    fn walk_down(&mut self, max: u16) {
        if self.coordinate.y >= max {
            self.coordinate.y = 0;
        }
        else {
            self.coordinate.y += 1;
        }
    }
    
    fn print_crab(&self) {
        println!("{}{}", MoveTo(self.coordinate.x, self.coordinate.y), self.crab);
    }
}

fn main() {
    // let stdout = Term::buffered_stdout();
    let size = terminal_size();
    let mut rng = rand::thread_rng();
    let crab = "🦀";
    let mut crab_vec:Vec<Crab> = Vec::new();
    let delay = time::Duration::from_secs(1);

    clearscreen::clear().expect("Failed to clear screen");

    if let Some((Width(w), Height(h))) = size {
        for i in 0..10 {
            let x = rng.gen_range(1..w);
            let y = rng.gen_range(1..h);
            let coord = Coordinate { x: x, y: y };
            let crab = Crab { crab: crab.to_string(), coordinate: coord };
            crab_vec.push(crab);
        }

        loop {
            for i in 0..10 {
                crab_vec[i].print_crab();
                if i % 2 == 0 {
                    crab_vec[i].walk_up(h);
                }
                else {
                    crab_vec[i].walk_down(h);
                }
            }
            thread::sleep(delay);
            clearscreen::clear().expect("failed to clear screen");
            // if let Ok(character) = stdout.read_char() {
            //     match character {
            //         'q' => break,
            //         _ => continue,
            //     }
            // }
        }
    }
}