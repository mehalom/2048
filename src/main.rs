mod game;
fn main() {
    // let mut my_game = game::Game::new();
    let mut my_game = game::Game {
        Board : vec![vec![4 as game::Max_num, 4 as game::Max_num, 2 as game::Max_num, 4 as game::Max_num]; 4],
        Score : 0,
    };
    my_game.print();
    println!("");
    // my_game.Left();
    my_game.Right();
    my_game.print();
    // let mut test:Vec<u16> = vec![0,2,2,2,2,4,4,8,2,4,8];
    // println!("{:?}", test);
    // game::Game::Shift(&mut test);
    // println!("{:?}", test);
}
