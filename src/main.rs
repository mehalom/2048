extern crate rand;
extern crate termion;

use self::termion::raw::IntoRawMode;
use std::io::{Write, stdout};

mod game;
use game::{Status,Game};

fn main() {
    let mut my_game = Game::new();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}{}",termion::clear::All, termion::cursor::Hide, termion::cursor::Goto(1, 1)).unwrap();
    my_game.add();
    my_game.add();
    println!("2048 in Rust v0.0.1-alpha");
    my_game.print();
    loop {
        match my_game.inp() {
            Status::Continue => {
                my_game.add();
                clean_last(7);
                my_game.print();
                if !my_game.try() {
                    println!("\rGame over!");
                    break;
                }
            },
            Status::Help => println!("\rUse arrows or WASD to move and q or CTRL + C to exit"),
            Status::Exit => break,
            Status::Impossible => continue,

        }
    }
    println!("\r");
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
fn clean_last(num:u16) {
    for i in 1..num {
        write!(stdout().into_raw_mode().unwrap(), "{}{}", termion::cursor::Goto(1, i), termion::clear::CurrentLine).unwrap();
    }
    write!(stdout().into_raw_mode().unwrap(), "{}", termion::cursor::Goto(1, 1)).unwrap();

}
