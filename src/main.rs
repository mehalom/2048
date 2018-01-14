mod game;
extern crate console;
extern crate rand;
extern crate termion;
use self::termion::raw::IntoRawMode;
use std::io::{Write, stdout};

// TODO исправить тип вывода функции ходов

fn main() {
    let mut my_game = game::Game::new();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}{}",termion::clear::All, termion::cursor::Hide, termion::cursor::Goto(1, 1)).unwrap();
    my_game.add();
    my_game.add();
    println!("2048 in Rust v0.0.1");
    my_game.print();
    let mut result: (bool, u8) = (true, 0);
    while result.1 == 0 {
        result = my_game.inp();
        while (result.1 == 2) | (result.0 == false) {
            result = my_game.inp();
            clean_last(6);
            my_game.print();
            println!("\r Use arrows or WASD to move and q or CTRL + C to exit");
        }
        my_game.add();
        clean_last(7);
        my_game.print();
        if !my_game.try() {
            println!("\rGame over!");
            break;
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
