use std::io;
use std::io::Write;

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


fn parse_move(input : String) -> Result<Move, ()> {
    let trimmed = String::from(input.trim());
    if trimmed.to_lowercase() == "pass" {
        return Ok(Move::Pass);
    }
    let split : Vec<&str> = trimmed.split(' ').collect();
    if split.len() != 2 {
        return Err(());
    }
    let col = COLS.iter().position(|&s| s == split[0].to_uppercase());
    let row = ROWS.iter().position(|&s| s == split[1]);
    match (col, row) {
        (Some(c), Some(r)) => Ok(Move::Placement((c, r))),
        _ => Err(()),
    }
}

const USAGE : &'static str = &"Use \"pass\" to pass, \"j 10\" for placing a stone.\n";

pub fn play() {
    let mut game = Game::new();
    println!("{}", USAGE);
    while !game.has_finished() {
        println!("{}\n", &format_board(&game.current_board()));
        print!("{:?}: ", game.current_player());
        io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match parse_move(input) {
            Err(_) => println!("{}", USAGE),
            Ok(m) => match game.make_move(m) {
                Err(err) => println!("{:?}", err),
                Ok(_) => {},
            },
        }
    }
}
