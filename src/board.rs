use std::collections::HashSet;

const SIZE : usize = 19;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Stone { Black, White }

impl Copy for Stone { }

type Board = [[Option<Stone>; SIZE]; SIZE];

type Position = (usize, usize);

enum IllegalMove {
    OutOfRange,
    Occupied,
    Suicidal,
    Ko,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Game {
    previous_board : Board,
    current_board : Board,
    current_player : Stone,
    white_captured : u16,
    black_captured : u16,
}

impl Copy for Game { }

fn format_board(board : &Board) -> String {
    let mut ret : String = String::new();
    for line in board {
        for cell in line {
            ret += " ";
            ret += match *cell {
                None => ".",
                Some(Black) => "*",
                Some(White) => "o",
            };
        }
        ret += "\n";
    }
    ret
}

fn get_neighbours(position : Position) -> HashSet<Position> {
    if !is_valid_position(&position) {
        return HashSet::new();
    }
    let (x, y) = position;
    let mut possible : HashSet<Position> = HashSet::new();
    possible.insert((x.wrapping_sub(1), y));
    possible.insert((x+1, y));
    possible.insert((x, y.wrapping_sub(1)));
    possible.insert((x, y+1));
    let ret : HashSet<Position> = possible.drain().filter(is_valid_position).collect();
    ret
}

fn is_valid_position(position : &Position) -> bool {
    let x = position.0;
    let y = position.1;
    x < SIZE && y < SIZE 
}

fn make_placement(game : &Game, position : Position) -> Result<Game,IllegalMove> {
    if !is_valid_position(&position) {
        return Err(IllegalMove::OutOfRange)
    }
    let (x, y) = position;
    if game.current_board[y][x] != None {
        return Err(IllegalMove::Occupied)
    }
    Ok(game.clone())
}
