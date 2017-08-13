use std::collections::HashSet;
use std::vec::Vec;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Stone { Black, White }


pub const SIZE : usize = 19;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Board([[Option<Stone>; SIZE]; SIZE]);

impl Board {
    pub fn empty() -> Board {
        Board([[None; SIZE];SIZE])
    }

    pub fn is_valid_position(&self, pos : Position) -> bool {
        // No need to compare to 0, x and y are unsigned
        pos.x < SIZE && pos.y < SIZE
    }

    fn get_neighbours(&self, pos: Position) -> HashSet<Position> {
        if !self.is_valid_position(pos) {
            return HashSet::new();
        }
        let mut possible : HashSet<Position> = HashSet::new();
        possible.insert(Position { x: pos.x.wrapping_sub(1), y: pos.y } );
        possible.insert(Position { x: pos.x+1, y: pos.y } );
        possible.insert(Position { x: pos.x, y: pos.y.wrapping_sub(1) } );
        possible.insert(Position { x: pos.x, y: pos.y+1 } );
        return possible.drain().filter(|&p| self.is_valid_position(p)).collect();
    }

    pub fn get(&self, pos: Position) -> Option<Stone> {
        if !self.is_valid_position(pos) {
            None
        } else {
            self.0[pos.y][pos.x]
        }
    }

    fn set(&mut self, pos: Position, stone: Option<Stone>) -> () {
        self.0[pos.y][pos.x] = stone
    }

    pub fn matrix(&self) -> [[Option<Stone>; SIZE]; SIZE] {
        self.0
    }

    fn get_group(&self, pos: Position) -> HashSet<Position> {
        let stone = self.get(pos);
        let mut group : HashSet<Position> = HashSet::new();
        let mut addition : HashSet<Position> = HashSet::new();
        addition.insert(pos);

        while !addition.is_empty() {
            group = group.union(&addition).cloned().collect();
            addition = addition.iter()
                .flat_map(|&p| self.get_neighbours(p))
                .filter(|&p| !group.contains(&p))
                .filter(|&p| self.get(p) == stone)
                .collect();
        }
        group
    }

    fn get_liberties(&self, pos: Position) -> HashSet<Position> {
        self.get_neighbours(pos).iter().filter(|&&n| self.get(n) == None).cloned().collect()
    }

    fn get_group_liberties(&self, group: &HashSet<Position>) -> HashSet<Position> {
        group.iter().flat_map(|&p| self.get_liberties(p)) .collect()
    }

    pub fn put(&self, pos: Position, stone: Stone) -> Result<Board,IllegalMove> {
        if !self.is_valid_position(pos) {
            return Err(IllegalMove::OutsideBoard)
        }
        if self.get(pos) != None {
            return Err(IllegalMove::Occupied)
        }

        let mut new_board = Board(self.matrix());

        // Place the stone
        new_board.set(pos, Some(stone));

        // Capture any surrounded groups
        for adj in new_board.get_neighbours(pos) {
            let adj_g = new_board.get_group(adj);
            if new_board.get_group_liberties(&adj_g).is_empty() {
                for p in adj_g {
                    new_board.set(p, None);
                }
            }
        }

        // If still no liberties, it's suicidal
        if self.get_group_liberties(&self.get_group(pos)).is_empty() {
            return Err(IllegalMove::Suicidal)
        }
        Ok(new_board)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Move {
    Placement(Position),
    Pass,
}
#[derive(Debug)]
pub enum IllegalMove {
    OutsideBoard,
    Occupied,
    Suicidal,
    Ko,
}


#[derive(PartialEq, Eq, Hash)]
pub struct Game {
    history : Vec<(Move, Board)>,
    white_captured : u16,
    black_captured : u16,
}

impl Game {
    pub fn new() -> Game {
        Game { history : Vec::new(),
               white_captured : 0,
               black_captured : 0,
        }
    }

    pub fn current_board(&self) -> Board {
        match self.history.last() {
            None => Board::empty(),
            Some(&(_, b)) => b,
        }
    }

    pub fn current_player(&self) -> Stone {
        match self.history.len() % 2 {
            0 => Stone::Black,
            _ => Stone::White,
        }
    }

    pub fn has_finished(&self) -> bool {
        // The game ends when the last two moves are passes
        self.history.iter().rev().take(2).filter(|&&(m,_)| m == Move::Pass).count() == 2
    }

    pub fn make_move(&mut self, m : Move) -> Result<(), IllegalMove> {
        let last_board = self.current_board();
        let board = match m {
            Move::Pass => last_board,
            Move::Placement(pos) => {
                let new_board = last_board.put(pos, self.current_player())?;
                if self.history.iter().any(|&(_, b)| b == new_board) {
                    return Err(IllegalMove::Ko)
                }
                new_board
            },
        };
        self.history.push((m, board));
        Ok(())
    }
}
