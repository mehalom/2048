type Max_num = u16;
pub struct Game {
    Board: Vec<Vec<Max_num>>,
    Score: u64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            Board : vec![vec![0 as Max_num; 4]; 4],
            Score : 0,
        }
    }
    // pub fn Up(&mut self) -> Game;
    // pub fn Down(&mut self) -> Game;
    pub fn Left(&mut self) -> u64{
        let mut score: u64 = 0;
        for mut row in &mut self.Board {
            score += Game::Shift(row);
        }
        return score;
    }
    // pub fn Right(&mut self) -> u64 {
    //     let mut score: u64 = 0;
    //     for mut row in &mut self.Board {
    //         let mut temp: Vec<Max_num> = Vec::new();
    //         for elem in row {
    //             temp.insert(0, *elem);
    //         }
    //         score += Game::Shift(&mut temp);
    //         for i in 0..row.len() {
    //             row[i] = temp[i];
    //         }
    //     }
    //     return score;
    }
    fn Shift(row: &mut Vec<Max_num>) -> u64{
        let mut score: u64 = 0;
        for i in 0..row.len() - 1 {
            println!("{:?}", row);
            if (row[i] == 0) & (row[i+1] != 0) {
                row.remove(i);
                row.push(0);
            }
        }
        println!("");
        for i in 0..row.len() - 1 {
            println!("{:?}", row);
            if (row[i] == row[i+1]) & (row[i] != 0) {
                row[i] *= 2;
                score += row[i] as u64;
                row.remove(i+1);
                row.push(0);
            }
        }
        return score;
    }
}
