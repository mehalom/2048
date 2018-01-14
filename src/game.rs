use rand;
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::color;
use std::io::{Write, stdout, stdin};

pub type MaxNum = u16;

pub struct Game {
    pub board: Vec<Vec<MaxNum>>,
    pub score: u64,
}

impl Game {
    pub fn print(&self) {
        println!("\rScore: {}", self.score);
        for row in &self.board {
            print!("\r");
            for elem in row.iter() {
                if *elem == 0 {
                    print!("*\t");
                } else {
                    match *elem {
                        2 => print!("{}{}\t{}", color::Fg(color::White), elem, color::Fg(color::Reset)),
                        4 => print!("{}{}\t{}", color::Fg(color::Red), elem, color::Fg(color::Reset)),
                        8 => print!("{}{}\t{}", color::Fg(color::Green), elem, color::Fg(color::Reset)),
                        16 => print!("{}{}\t{}", color::Fg(color::Yellow), elem, color::Fg(color::Reset)),
                        32 => print!("{}{}\t{}", color::Fg(color::Blue), elem, color::Fg(color::Reset)),
                        64 => print!("{}{}\t{}", color::Fg(color::Magenta), elem, color::Fg(color::Reset)),
                        128 => print!("{}{}\t{}", color::Fg(color::Cyan), elem, color::Fg(color::Reset)),
                        256 => print!("{}{}\t{}", color::Fg(color::LightRed), elem, color::Fg(color::Reset)),
                        512 => print!("{}{}\t{}", color::Fg(color::LightGreen), elem, color::Fg(color::Reset)),
                        1024 => print!("{}{}\t{}", color::Fg(color::LightYellow), elem, color::Fg(color::Reset)),
                        _ => print!("{}{}\t{}", color::Fg(color::LightMagenta), elem, color::Fg(color::Reset)),
                    };

                }
            }
            println!("");
        }
    }
    pub fn new() -> Game {
        Game {
            board: vec![vec![0 as MaxNum; 4]; 4],
            score: 0,
        }
    }
    fn up(&mut self) -> bool {
        let mut change: bool = false;
        for i in 0..self.board.len() {
            let mut temp: Vec<MaxNum> = Vec::new();
            for row in &self.board {
                temp.push(row[i]);
            }
            let ans = Game::shift(&mut temp);
            self.score += ans.0;
            change = change | ans.1;
            for j in 0..self.board.len() {
                self.board[j][i] = temp[j];
            }
        }
        return change;
    }
    fn down(&mut self) -> bool {
        let mut change: bool = false;
        for i in 0..self.board.len() {
            let mut temp: Vec<MaxNum> = Vec::new();
            for row in &self.board {
                temp.insert(0, row[i]);
            }
            let ans = Game::shift(&mut temp);
            self.score += ans.0;
            change = change | ans.1;
            for j in 0..self.board.len() {
                self.board[j][i] = temp[self.board.len() - 1 - j];
            }
        }
        return change;
    }
    fn left(&mut self) -> bool {
        let mut change: bool = false;
        for mut row in &mut self.board {
            let ans = Game::shift(row);
            self.score += ans.0;
            change = change | ans.1;
        }
        return change;
    }
    fn right(&mut self) -> bool {
        let mut change: bool = false;
        for mut row in &mut self.board {
            let mut temp: Vec<MaxNum> = Vec::new();
            for elem in row.iter() {
                temp.insert(0, *elem);
            }
            let ans = Game::shift(&mut temp);
            self.score += ans.0;
            change = change | ans.1;
            row.clear();
            for i in 0..temp.len() {
                row.insert(0, temp[i]);
            }
        }
        return change;
    }
    fn shift(row: &mut Vec<MaxNum>) -> (u64, bool) {
        let mut score: u64 = 0;
        let mut flag: bool = true;
        let mut change: bool = false;
        while flag == true {
            flag = false;
            for i in 0..row.len() - 1 {
                if (row[i] == 0) & (row[i + 1] != 0) {
                    row.remove(i);
                    row.push(0);
                    flag = true;
                    change = true;
                }
            }
        }
        for i in 0..row.len() - 1 {
            if (row[i] == row[i + 1]) & (row[i] != 0) {
                row[i] *= 2;
                score += row[i] as u64;
                row.remove(i + 1);
                row.push(0);
                change = true;
            }
        }
        // println!("{:?}", row);
        // println!("");
        return (score, change);
    }
    pub fn add(&mut self) {
        let mut counter = rand::random::<u8>();
        loop {
            for mut row in &mut self.board {
                for elem in &mut row.iter_mut() {
                    if (*elem == 0) & (counter == 0) {
                        *elem = 2;
                        return;
                    } else if *elem == 0 {
                        counter -= 1;
                    }
                }
            }
        }
    }
    pub fn inp(&mut self) -> (bool,u8) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut answer:bool = true;
        let mut cnt:u8 = 0;
        stdout.flush().unwrap();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') | Key::Ctrl('c') => cnt = 1,
                Key::Left | Key::Char('a') => answer = self.left(),
                Key::Right | Key::Char('d') => answer = self.right(),
                Key::Up | Key::Char('w') => answer = self.up(),
                Key::Down | Key::Char('s') => answer = self.down(),
                _ => cnt = 2,
            };
            stdout.flush().unwrap();
            break;
        }
        return (answer, cnt);
    }
    pub fn try(&self) -> bool {
        for mut row in &self.board {
            let mut temp: Vec<MaxNum> = Vec::new();
            for elem in row.iter() {
                temp.insert(0, *elem);
            }
            if Game::shift(&mut temp).1 {
                return true;
            }
        }
        for mut row in &self.board {
            let mut temp: Vec<MaxNum> = Vec::new();
            for elem in row.iter() {
                temp.push(*elem);
            }
            if Game::shift(&mut temp).1 {
                return true;
            }
        }
        for i in 0..self.board.len() {
            let mut temp: Vec<MaxNum> = Vec::new();
            for row in &self.board {
                temp.insert(0, row[i]);
            }
            if Game::shift(&mut temp).1 {
                return true;
            }
        }
        for i in 0..self.board.len() {
            let mut temp: Vec<MaxNum> = Vec::new();
            for row in &self.board {
                temp.push(row[i]);
            }
            if Game::shift(&mut temp).1 {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use game::*;
    #[test]
    fn shift() {
        let mut test_vec: Vec<MaxNum> = vec![0, 0, 2, 4, 4, 4, 4, 8, 0, 8, 8];
        let ans = Game::shift(&mut test_vec);
        assert_eq!(ans.0, 32);
        assert_eq!(ans.1, true);
        assert_eq!(test_vec, vec![2, 8, 8, 16, 8, 0, 0, 0, 0, 0, 0]);
    }
    #[test]
    fn up() {
        let mut my_game = Game {
            board: vec![
                vec![4 as MaxNum, 4 as MaxNum, 4 as MaxNum, 4 as MaxNum],
                vec![2 as MaxNum, 4 as MaxNum, 4 as MaxNum, 4 as MaxNum],
                vec![4 as MaxNum, 2 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![4 as MaxNum, 4 as MaxNum, 2 as MaxNum, 0 as MaxNum],
            ],
            score: 0,
        };
        let answer = Game {
            board: vec![
                vec![4 as MaxNum, 8 as MaxNum, 8 as MaxNum, 8 as MaxNum],
                vec![2 as MaxNum, 2 as MaxNum, 2 as MaxNum, 0 as MaxNum],
                vec![8 as MaxNum, 4 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![0 as MaxNum, 0 as MaxNum, 0 as MaxNum, 0 as MaxNum],
            ],
            score: 32,
        };
        assert_eq!(my_game.up(), true);
        assert_eq!(my_game.board, answer.board);
        assert_eq!(my_game.score, answer.score);
    }
    #[test]
    fn left() {
        println!("");
        let mut my_game = Game {
            board: vec![
                vec![4 as MaxNum, 4 as MaxNum, 4 as MaxNum, 4 as MaxNum],
                vec![2 as MaxNum, 4 as MaxNum, 4 as MaxNum, 4 as MaxNum],
                vec![4 as MaxNum, 2 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![4 as MaxNum, 4 as MaxNum, 2 as MaxNum, 0 as MaxNum],
            ],
            score: 0,
        };
        let answer = Game {
            board: vec![
                vec![8 as MaxNum, 8 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![2 as MaxNum, 8 as MaxNum, 4 as MaxNum, 0 as MaxNum],
                vec![4 as MaxNum, 2 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![8 as MaxNum, 2 as MaxNum, 0 as MaxNum, 0 as MaxNum],
            ],
            score: 32,
        };
        my_game.left();
        assert_eq!(my_game.board, answer.board);
        assert_eq!(my_game.score, answer.score);
    }
    #[test]
    fn right() {
        let mut my_game = Game {
            board: vec![
                vec![4 as MaxNum, 4 as MaxNum, 4 as MaxNum, 4 as MaxNum],
                vec![2 as MaxNum, 4 as MaxNum, 4 as MaxNum, 4 as MaxNum],
                vec![4 as MaxNum, 2 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![4 as MaxNum, 4 as MaxNum, 2 as MaxNum, 0 as MaxNum],
            ],
            score: 0,
        };
        let answer = Game {
            board: vec![
                vec![0 as MaxNum, 0 as MaxNum, 8 as MaxNum, 8 as MaxNum],
                vec![0 as MaxNum, 2 as MaxNum, 4 as MaxNum, 8 as MaxNum],
                vec![0 as MaxNum, 0 as MaxNum, 4 as MaxNum, 2 as MaxNum],
                vec![0 as MaxNum, 0 as MaxNum, 8 as MaxNum, 2 as MaxNum],
            ],
            score: 32,
        };
        my_game.right();
        assert_eq!(my_game.board, answer.board);
        assert_eq!(my_game.score, answer.score);
    }
    #[test]
    fn down() {
        let mut my_game = Game {
            board: vec![
                vec![4 as MaxNum, 4 as MaxNum, 4 as MaxNum, 4 as MaxNum],
                vec![2 as MaxNum, 4 as MaxNum, 4 as MaxNum, 4 as MaxNum],
                vec![4 as MaxNum, 2 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![4 as MaxNum, 4 as MaxNum, 2 as MaxNum, 0 as MaxNum],
            ],
            score: 0,
        };
        let answer = Game {
            board: vec![
                vec![0 as MaxNum, 0 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![4 as MaxNum, 8 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![2 as MaxNum, 2 as MaxNum, 8 as MaxNum, 0 as MaxNum],
                vec![8 as MaxNum, 4 as MaxNum, 2 as MaxNum, 8 as MaxNum],
            ],
            score: 32,
        };
        my_game.down();
        assert_eq!(my_game.board, answer.board);
        assert_eq!(my_game.score, answer.score);
    }
    #[test]
    fn try() {
        let test_true1 = Game {
            board: vec![
                vec![4 as MaxNum, 4 as MaxNum, 4 as MaxNum, 4 as MaxNum],
                vec![2 as MaxNum, 4 as MaxNum, 4 as MaxNum, 4 as MaxNum],
                vec![4 as MaxNum, 2 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![4 as MaxNum, 4 as MaxNum, 2 as MaxNum, 0 as MaxNum],
            ],
            score: 0,
        };
        let test_true2 = Game {
            board: vec![
                vec![4 as MaxNum, 8 as MaxNum, 8 as MaxNum, 8 as MaxNum],
                vec![2 as MaxNum, 2 as MaxNum, 2 as MaxNum, 0 as MaxNum],
                vec![8 as MaxNum, 4 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![0 as MaxNum, 0 as MaxNum, 0 as MaxNum, 0 as MaxNum],
            ],
            score: 32,
        };
        let test_true3 = Game {
            board: vec![
                vec![8 as MaxNum, 16 as MaxNum, 32 as MaxNum, 8 as MaxNum],
                vec![4 as MaxNum, 2 as MaxNum, 4 as MaxNum, 0 as MaxNum],
                vec![2 as MaxNum, 4 as MaxNum, 0 as MaxNum, 0 as MaxNum],
                vec![0 as MaxNum, 0 as MaxNum, 0 as MaxNum, 0 as MaxNum],
            ],
            score: 32,
        };
        let test_true4 = Game {
            board: vec![
                vec![4 as MaxNum, 8 as MaxNum, 16 as MaxNum, 8 as MaxNum],
                vec![2 as MaxNum, 128 as MaxNum, 2 as MaxNum, 4 as MaxNum],
                vec![8 as MaxNum, 4 as MaxNum, 32 as MaxNum, 8 as MaxNum],
                vec![0 as MaxNum, 0 as MaxNum, 8 as MaxNum, 16 as MaxNum],
            ],
            score: 32,
        };
        let test_true5 = Game {
            board: vec![
                vec![4 as MaxNum, 8 as MaxNum, 32 as MaxNum, 0 as MaxNum],
                vec![2 as MaxNum, 16 as MaxNum, 128 as MaxNum, 0 as MaxNum],
                vec![16 as MaxNum, 4 as MaxNum, 2 as MaxNum, 0 as MaxNum],
                vec![128 as MaxNum, 2 as MaxNum, 4 as MaxNum, 4 as MaxNum],
            ],
            score: 32,
        };
        let test_true6 = Game {
            board: vec![
                vec![0 as MaxNum, 0 as MaxNum, 0 as MaxNum, 8 as MaxNum],
                vec![16 as MaxNum, 2 as MaxNum, 8 as MaxNum, 4 as MaxNum],
                vec![8 as MaxNum, 4 as MaxNum, 2 as MaxNum, 8 as MaxNum],
                vec![16 as MaxNum, 32 as MaxNum, 4 as MaxNum, 16 as MaxNum],
            ],
            score: 32,
        };
        let test_false1 = Game {
            board: vec![
                vec![4 as MaxNum, 8 as MaxNum, 4 as MaxNum, 8 as MaxNum],
                vec![2 as MaxNum, 4 as MaxNum, 2 as MaxNum, 4 as MaxNum],
                vec![4 as MaxNum, 2 as MaxNum, 8 as MaxNum, 16 as MaxNum],
                vec![2 as MaxNum, 4 as MaxNum, 16 as MaxNum, 32 as MaxNum],
            ],
            score: 32,
        };
        assert_eq!(test_true1.try(), true);
        assert_eq!(test_true2.try(), true);
        assert_eq!(test_true3.try(), true);
        assert_eq!(test_true4.try(), true);
        assert_eq!(test_true5.try(), true);
        assert_eq!(test_true6.try(), true);
        assert_eq!(test_false1.try(), false);
    }
    #[test]
    fn print() {
        let test = Game {
            board: vec![
                vec![0 as MaxNum, 2 as MaxNum, 4 as MaxNum, 8 as MaxNum],
                vec![16 as MaxNum, 32 as MaxNum, 64 as MaxNum, 128 as MaxNum],
                vec![256 as MaxNum, 512 as MaxNum, 1024 as MaxNum, 2048 as MaxNum],
                vec![4096 as MaxNum, 8192 as MaxNum, 2 as MaxNum, 0 as MaxNum],
            ],
            score: 0,
        };
        test.print();
    }
}
