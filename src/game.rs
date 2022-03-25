use crate::game_types::*;

pub struct GameContainer {
    pub history: GameHistory,
}

impl GameContainer {
    pub fn new() -> GameContainer {
        let history = GameHistory::new(vec![GameContainer::default_board().unwrap()], None);
        GameContainer { history: history }
    }
    pub fn execute_move(&mut self, mv: ChessMove) -> Option<ChessMove> {
        self.history.execute_move(mv);
        Some(mv)
    }
    pub fn get_board(&mut self) -> BoardState {
        self.history.board_states.last_mut().unwrap().clone()
    }
    #[allow(dead_code)]
    fn default_board() -> tetra::Result<BoardState> {
        let mut pieces = Vec::new();
        let mut p = |i| pieces.push(i);
        for i in 0..8 {
            p(Piece::new(i, 1, PieceType::PAWN, PlayerColor::WHITE));
        }
        p(Piece::new(0, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece::new(7, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece::new(2, 0, PieceType::BISHOP, PlayerColor::WHITE));
        p(Piece::new(5, 0, PieceType::BISHOP, PlayerColor::WHITE));
        p(Piece::new(1, 0, PieceType::KNIGHT, PlayerColor::WHITE));
        p(Piece::new(6, 0, PieceType::KNIGHT, PlayerColor::WHITE));
        p(Piece::new(4, 0, PieceType::KING, PlayerColor::WHITE));
        p(Piece::new(3, 0, PieceType::QUEEN, PlayerColor::WHITE));
        for i in 0..8 {
            p(Piece::new(i, 6, PieceType::PAWN, PlayerColor::BLACK));
        }
        p(Piece::new(0, 7, PieceType::ROOK, PlayerColor::BLACK));
        p(Piece::new(7, 7, PieceType::ROOK, PlayerColor::BLACK));
        p(Piece::new(2, 7, PieceType::BISHOP, PlayerColor::BLACK));
        p(Piece::new(5, 7, PieceType::BISHOP, PlayerColor::BLACK));
        p(Piece::new(1, 7, PieceType::KNIGHT, PlayerColor::BLACK));
        p(Piece::new(6, 7, PieceType::KNIGHT, PlayerColor::BLACK));
        p(Piece::new(4, 7, PieceType::KING, PlayerColor::BLACK));
        p(Piece::new(3, 7, PieceType::QUEEN, PlayerColor::BLACK));
        Ok(BoardState::create(
            pieces,
            PlayerColor::WHITE,
            false,
            false,
            false,
            false,
        ))
    }
    #[allow(dead_code)]
    fn test_board_2() -> tetra::Result<BoardState> {
        let mut pieces = Vec::new();
        let mut p = |i| pieces.push(i);
        p(Piece::new(1, 4, PieceType::KNIGHT, PlayerColor::WHITE));
        p(Piece::new(6, 4, PieceType::KING, PlayerColor::WHITE));
        p(Piece::new(5, 3, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece::new(3, 1, PieceType::QUEEN, PlayerColor::WHITE));
        p(Piece::new(6, 2, PieceType::KING, PlayerColor::BLACK));
        p(Piece::new(4, 4, PieceType::BISHOP, PlayerColor::BLACK));
        p(Piece::new(6, 6, PieceType::PAWN, PlayerColor::BLACK));
        Ok(BoardState::create(
            pieces,
            PlayerColor::WHITE,
            false,
            false,
            false,
            false,
        ))
    }
    fn test_board_1() -> tetra::Result<BoardState> {
        let mut pieces = Vec::new();
        let mut p = |i| pieces.push(i);
        p(Piece::new(1, 4, PieceType::KNIGHT, PlayerColor::WHITE));
        p(Piece::new(6, 4, PieceType::KING, PlayerColor::BLACK));
        p(Piece::new(0, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece::new(7, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece::new(3, 2, PieceType::QUEEN, PlayerColor::WHITE));
        p(Piece::new(4, 0, PieceType::KING, PlayerColor::WHITE));
        p(Piece::new(5, 3, PieceType::BISHOP, PlayerColor::BLACK));
        p(Piece::new(6, 6, PieceType::PAWN, PlayerColor::BLACK));
        Ok(BoardState::create(
            pieces,
            PlayerColor::WHITE,
            false,
            false,
            false,
            false,
        ))
    }
}
