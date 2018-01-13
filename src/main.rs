mod game;
extern crate console;
extern crate rand;
extern crate termion;
use console::Term;
use self::termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
fn main() {
    let mut my_game = game::Game::new();
    let term = Term::stdout();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    my_game.add();
    my_game.add();
    clean(&term);
    println!("2048 in Rust v0.0.1");
    my_game.print();
    let mut result: (bool, u8) = (true, 0);
    while result.1 == 0 {
        result = my_game.inp();
        while (result.1 == 2) | (result.0 == false) {
            result = my_game.inp();
            term.clear_last_lines(6);
            my_game.print();
            println!("\r Use arrows to move and q to exit");
        }
        my_game.add();
        term.clear_last_lines(6);
        my_game.print();
        if !my_game.try() {
            println!("\rGame over!");
            break;
        }
    }
    println!("\r");
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
fn clean(term: &Term) {
    for _ in 0..20 {
        println!("");
    }
    term.clear_last_lines(20);
}
