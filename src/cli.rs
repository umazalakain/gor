use std::io;

use super::board::*;


fn format_board(board : &Board) -> String {
    let mut ret : String = String::new();
    for line in board {
        for cell in line {
            ret += " ";
            ret += match *cell {
                None => ".",
                Some(Stone::Black) => "*",
                Some(Stone::White) => "o",
            };
        }
        ret += "\n";
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
