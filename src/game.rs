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
    pub fn Left(&mut self) {
        for mut row in &mut self.Board {
            Game::Shift(row);
        }
    }
    // pub fn Right(&self) -> Game;
    fn Shift(row: &mut Vec<Max_num>){
        let mut flag: bool = true;
        while flag == true {
            flag = false;
            for i in 0..row.len() - 1 {
                // println!("{:?}", row);
                // println!("{} {} {}", i, row[i], row[i+1]);
                if (row[i] == row[i+1]) & (row[i] != 0) {
                    row[i] *= 2;
                    row.remove(i+1);
                    row.push(0);
                    flag = true;
                    break;
                } else if (row[i] == 0) & (row[i+1] != 0) {
                    row.remove(i);
                    row.push(0);
                    flag = true;
                    break;
                }
            }
        }
    }


}
