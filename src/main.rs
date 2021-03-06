/*
    2048 is a small Rust implementation of famous game by Gabriele Cirulli
    (See <https://github.com/gabrielecirulli/2048> for more details)

    Copyright (C) 2018  Eugene Lomov <eugene.v.lomov@gmail.com>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate rand;
extern crate termion;

use self::termion::raw::IntoRawMode;
use std::io::{stdout, Write};

mod game;
use game::{Game, Status};

fn main() {
    let mut my_game = Game::new();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Hide,
        termion::cursor::Goto(1, 1)
    ).unwrap();
    my_game.best_read();
    my_game.add();
    my_game.add();
    println!("2048 in Rust v0.2.0");
    my_game.print();
    loop {
        match my_game.inp() {
            Status::Continue => {
                my_game.add();
                clean_last(10);
                my_game.print();
                if !my_game.try() {
                    println!("\rGame over!\n\rYou made {} moves.", my_game.moves);
                    my_game.if_best();
                    break;
                }
            }
            Status::Help => {
                println!("\rUse arrows or WASD to move\n\rq or CTRL + C to exit\n\rb to show stats")
            }
            Status::Exit => break,
            Status::Impossible => continue,
        }
    }
    println!("\r");
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
fn clean_last(num: u16) {
    for i in 1..num {
        write!(
            stdout().into_raw_mode().unwrap(),
            "{}{}",
            termion::cursor::Goto(1, i),
            termion::clear::CurrentLine
        ).unwrap();
    }
    write!(
        stdout().into_raw_mode().unwrap(),
        "{}",
        termion::cursor::Goto(1, 1)
    ).unwrap();
}
