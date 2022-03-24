use crate::game_types::*;

pub struct GameContainer {
    pub history: GameHistory,
}

impl GameContainer {
    pub fn new() -> tetra::Result<GameContainer> {
        let history = GameHistory::new_game()?;
        Ok(GameContainer { history })
    }
    pub fn execute_move(&mut self, mv: ChessMove) -> Option<ChessMove> {
        self.history.execute_move(mv);
        Some(mv)
    }
    pub fn get_board(&mut self) -> BoardState {
        self.history.board_states.last_mut().unwrap().clone()
    }
}
