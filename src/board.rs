use std::collections::HashSet;
use std::vec::Vec;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Stone { Black, White }


const SIZE : usize = 19;
type Board = [[Option<Stone>; SIZE]; SIZE];
const EMPTY_BOARD : Board = [[None; SIZE];SIZE];


type Position = (usize, usize);
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Move {
    Placement(Position),
    Pass,
}
enum IllegalMove {
    OutsideBoard,
    Occupied,
    Suicidal,
    Ko,
}


#[derive(PartialEq, Eq, Hash)]
struct Game {
    history : Vec<(Move, Board)>,
    white_captured : u16,
    black_captured : u16,
}

impl Game {
    fn current_board(&self) -> Board {
        match self.history.last() {
            None => EMPTY_BOARD,
            Some(&(_, b)) => b,
        }
    }

    fn current_player(&self) -> Stone {
        match self.history.len() % 2 {
            0 => Stone::Black,
            _ => Stone::White,
        }
    }

    fn has_finished(&self) -> bool {
        // The game ends when the last two moves are passes
        self.history.iter().rev().take(2).all(|&(m,_)| m == Move::Pass)
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
    // No need to compare to 0, x and y are unsigned
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

fn make_move(game : &mut Game, m: Move) -> Result<(), IllegalMove> {
    let last_board = game.current_board();
    let board = match m {
        Move::Pass => last_board,
        Move::Placement(position) => {
            let new_board = try!(put_stone(last_board, position));
            if game.history.iter().any(|&(_, b)| b == new_board) {
                return Err(IllegalMove::Ko)
            }
            new_board
        },
    };
    game.history.push((m, board));
    Ok(())
}
