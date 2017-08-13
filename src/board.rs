use std::collections::HashSet;
use std::vec::Vec;

const SIZE : usize = 19;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Stone { Black, White }

type Board = [[Option<Stone>; SIZE]; SIZE];

static EMPTY_BOARD : Board = [[None; SIZE];SIZE];

type Position = (usize, usize);

enum IllegalMove {
    OutsideBoard,
    Occupied,
    Suicidal,
    Ko,
}

#[derive(PartialEq, Eq, Hash)]
struct Game {
    history : Vec<Board>,
    white_captured : u16,
    black_captured : u16,
}

impl Game {
    fn current_board(&self) -> Board {
        match self.history.last() {
            None => EMPTY_BOARD,
            Some(&b) => b,
        }
    }

    fn current_player(&self) -> Stone {
        match self.history.len() % 2 {
            0 => Stone::Black,
            _ => Stone::White,
        }
    }
}

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
    return possible.drain().filter(is_valid_position).collect();
}

fn is_valid_position(position : &Position) -> bool {
    let x = position.0;
    let y = position.1;
    x < SIZE && y < SIZE 
}

fn put_stone(board : Board, position : Position) -> Result<Board,IllegalMove> {
    if !is_valid_position(&position) {
        return Err(IllegalMove::OutsideBoard)
    }
    let (x, y) = position;
    if board[y][x] != None {
        return Err(IllegalMove::Occupied)
    }
    // TODO: Check suicidal
    // TODO: Actually place the stone
    Ok(board)
}

fn perform_turn(game : &mut Game, position: Position) -> Result<&mut Game, IllegalMove> {
    match put_stone(game.current_board(), position) {
        Err(e) => Err(e),
        Ok(b) => {
            if game.history.contains(&b) {
                Err(IllegalMove::Ko)
            } else {
                game.history.push(b);
                Ok(game)
            }
        },
    }
}
