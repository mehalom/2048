mod game;
extern crate console;
extern crate rand;
use console::Term;
fn main() {
    let mut my_game = game::Game::new();
    let term = Term::stdout();
    my_game.add();
    my_game.add();
    clean(&term);
    println!("2048 in Rust v0.0.1");
    my_game.print();
    loop {
        while !my_game.inp() {
            term.clear_last_lines(7);
            my_game.print();
        }
        my_game.add();
        term.clear_last_lines(6);
        my_game.print();
        if !my_game.try() {
            break;
        }
    }
    println!("Game over!");
}
fn clean(term: &Term) {
    for _ in 0..20 {
        println!("");
    }
    term.clear_last_lines(20);
}
