use crate::game_types::*;
#[derive(Clone, Copy)]
pub struct Engine {}
impl Engine {
    pub fn new() -> Engine {
        Engine {}
    }
    pub fn make_move(&mut self, board_state: BoardState) -> ChessMove {
        let moves = board_state.get_all_legal_moves();
        assert!(moves.len() != 0);
        return moves.last().unwrap().clone();
    }
}
