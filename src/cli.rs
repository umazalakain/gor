use std::io;

use super::board::*;

const COLS : &'static [&'static str] = &["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
const ROWS : &'static [&'static str] = &["1","2","3","4","5","6","7","8","9","10","11","12","13","14","15","16","17","18","19","20","21","22","23","24","25","26"];

fn format_board(board : &Board) -> String {
    let mut ret : String = String::new();
    for (i, line) in board.iter().enumerate() {
        for cell in line {
            ret += " ";
            ret += match *cell {
                None => ".",
                Some(Stone::Black) => "*",
                Some(Stone::White) => "o",
            };
        }
        ret += &format!("{}\n", ROWS[i]);
    }
    for j in 0..board.len() {
        ret += &format!(" {}", COLS[j]);
    }
    ret
}


pub fn play() {
    let mut game = Game::new();
    while !game.has_finished() {
        println!("{}", &format_board(&game.current_board()));
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        game.make_move(Move::Pass);
    }
}
