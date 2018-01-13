mod game;
extern crate rand;
extern crate console;
use console::Term;
fn main() {
    println!("2048 in Rust v0.0.1");
    let mut my_game = game::Game::new();
    let term = Term::stdout();
    my_game.add();
    my_game.add();
    clean(&term);
    my_game.print();
    loop {
        while ! my_game.inp() {
            term.clear_last_lines(7);
            my_game.print();
        }
        my_game.add();
        term.clear_last_lines(6);
        my_game.print();

    }
}
fn clean(term: &Term) {
    for _ in 0..20 {
        println!("");
    }
    term.clear_last_lines(20);
}
