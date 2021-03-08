use std::fmt;

use crate::abstractions::Environment;

/// Identity of tic tac toe players
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AgentId {
    X,
    O,
}

/// Display trait for tic tac toe players.
impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgentId::O => write!(f, "O"),
            AgentId::X => write!(f, "X"),
        }
    }
}

/// Actions will be a number from 0 to 8 representing the position on the tic tac toe board.
pub type Action = u8;

/// Representation of the tic tac toe board
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Board {
    moves_x: u16,  // As a binary string. Puts a 1 in the positions where X moved
    moves_o: u16,  // As a binary string. Puts a 1 in the positions where Y moved
    turn: AgentId, // Player that will make the next move
}

/// Display trait for tic tac toe board.
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x = self.moves_x;
        let mut o = self.moves_o;

        let mut x_pos;
        let mut o_pos;

        for _ in 0..3 {
            for _ in 0..3 {
                x_pos = (x & 1) == 1;
                o_pos = (o & 1) == 1;

                x = x >> 1;
                o = o >> 1;

                match (x_pos, o_pos) {
                    (true, false) => write!(f, "| {} |", "X").ok(),
                    (false, true) => write!(f, "| {} |", "O").ok(),
                    (false, false) => write!(f, "| {} |", " ").ok(),
                    (true, true) => write!(f, "| {} |", "?").ok(),
                };
            }
            write!(f, "\n").ok();
        }

        write! {f, "End of board"}
    }
}

/// Struct to represent current occupied elements in a board after a given action
pub struct NextAction {
    board_state: u16, // As a binary string. Puts a 1 in the occupied possitions (starting at current)
    current: Action,
}

/// Implements occupied
impl NextAction {
    /// Initializes structure based on a given board.
    fn new(board: &Board) -> Self {
        let board_state = board.moves_o | board.moves_x;
        let current = if board.is_terminal() { 9 } else { 0 };
        NextAction {
            board_state,
            current,
        }
    }
}

/// Implements iterator for next action
impl Iterator for NextAction {
    type Item = Action;

    /// Define sequence to iterate
    fn next(&mut self) -> Option<Self::Item> {
        while self.board_state & 1 == 1 {
            self.board_state = self.board_state >> 1;
            self.current += 1;
        }

        let output = if self.current > 8 {
            None
        } else {
            Some(self.current)
        };

        self.board_state = self.board_state >> 1;
        self.current += 1;

        return output;
    }
}

/// Implementation of environment for tic tac toe board.
impl Environment<Action, AgentId> for Board {
    type ActionIter = NextAction;

    /// Initializes an empty tic tac toe board.
    fn initial_state() -> Self {
        Board {
            moves_x: 0,
            moves_o: 0,
            turn: AgentId::X,
        }
    }

    /// Updates the board by filling the position given by action.
    /// Returns true iff the board was updated by the action.
    fn update(&mut self, a: &Action) -> bool {
        if !self.is_valid(a) {
            return false;
        } else {
            let m = 1 << a;
            if self.turn == AgentId::X {
                self.moves_x |= m;
                self.turn = AgentId::O
            } else {
                self.moves_o |= m;
                self.turn = AgentId::X
            }
            return true;
        }
    }

    /// Returns a board with what would happen if action 'a' were performed.
    fn what_if(&self, a: &Action) -> Self {
        let mut board = self.clone();
        board.update(a);
        return board;
    }

    /// Produces a list of valid actions in the current board.
    fn valid_actions(&self) -> Self::ActionIter {
        let next_action = NextAction::new(self);
        return next_action;
    }

    /// Returns true iff the action 'a' is valid in the current board.
    fn is_valid(&self, &a: &Action) -> bool {
        let a_bounded = a <= 8;
        let x_empty = !(((self.moves_x >> a) & 1) == 1);
        let y_empty = !(((self.moves_o >> a) & 1) == 1);
        return a_bounded & x_empty & y_empty;
    }

    /// Returns true iff the board is in a terminal position.
    fn is_terminal(&self) -> bool {
        if is_winning(self.moves_x) {
            return true;
        } else if is_winning(self.moves_o) {
            return true;
        } else if is_filled(&self) {
            return true;
        } else {
            return false;
        }
    }

    /// Returns the agentId of the player for the next move.
    fn turn(&self) -> AgentId {
        return self.turn;
    }

    /// It returns Some(agentId) with agentId  of the player who won the game.
    /// If no player had won, it returns None
    fn winner(&self) -> Option<AgentId> {
        if is_winning(self.moves_x) {
            return Some(AgentId::X);
        } else if is_winning(self.moves_o) {
            return Some(AgentId::O);
        } else {
            return None;
        }
    }
}

/// Checks whether one of the players has a winning position.
fn is_winning(position: u16) -> bool {
    // Binary representation of positions that win the game.
    let winning_masks = vec![
        0b111u16,
        0b111000u16,
        0b111000000u16,
        0b1001001u16,
        0b10010010u16,
        0b100100100u16,
        0b100010001u16,
        0b1010100u16,
    ];

    for mask in winning_masks {
        if position & mask == mask {
            return true;
        }
    }
    return false;
}

/// Checks whether the whole board is filled
fn is_filled(board: &Board) -> bool {
    let full = 0b111111111u16;
    let fill = (board.moves_x | board.moves_o) & full;
    return fill == full;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Plays a manual game and check that the board updates accordingly.
    fn manual_game() {
        let mut board = Board::initial_state();
        assert_eq!(board.moves_x, 0);
        assert_eq!(board.moves_o, 0);
        assert_eq!(board.turn(), AgentId::X);

        assert_eq!(board.update(&&4), true);
        assert_eq!(board.moves_x, 0b10000);
        assert_eq!(board.turn, AgentId::O);

        assert_eq!(board.update(&5), true);
        assert_eq!(board.moves_o, 0b100000);
        assert_eq!(board.turn, AgentId::X);

        assert_eq!(board.update(&0), true);
        assert_eq!(board.moves_x, 0b10001);
        assert_eq!(board.turn, AgentId::O);

        assert_eq!(board.update(&0), false);

        assert_eq!(board.update(&1), true);
        assert_eq!(board.moves_o, 0b100010);
        assert_eq!(board.turn, AgentId::X);

        assert_eq!(board.update(&8), true);
        assert_eq!(board.moves_x, 0b100010001);
        assert_eq!(board.turn, AgentId::O);

        assert_eq!(is_filled(&board), false);
        assert_eq!(is_winning(board.moves_o), false);
        assert_eq!(is_winning(board.moves_x), true);
        assert_eq!(board.is_terminal(), true);
    }
}
