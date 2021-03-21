pub struct GameContainer {
    history: GameHistory, // data
}

impl GameContainer {
    pub fn new() -> tetra::Result<GameContainer> {
        let history = GameHistory::new_game()?;
        Ok(GameContainer { history })
    }
    pub fn current_pieces(&mut self) -> tetra::Result<&Vec<Piece>> {
        self.history.board_states.last_mut();
        Ok(&self.history.board_states.last_mut().unwrap().pieces)
    }
}
pub struct GameHistory {
    board_states: Vec<BoardState>,
}
impl GameHistory {
    fn new_game() -> tetra::Result<GameHistory> {
        let board_states = vec![BoardState::default_board()?];
        Ok(GameHistory { board_states })
    }
}
pub struct BoardState {
    pieces: Vec<Piece>,
}
impl BoardState {
    fn default_board() -> tetra::Result<BoardState> {
        let mut pieces = Vec::new();
        let mut p = |i| pieces.push(i);
        for i in 0..8 {
            p(Piece(i, 1, PieceType::PAWN, PlayerColor::WHITE));
        }
        p(Piece(0, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece(7, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece(1, 0, PieceType::BISHOP, PlayerColor::WHITE));
        p(Piece(6, 0, PieceType::BISHOP, PlayerColor::WHITE));
        p(Piece(2, 0, PieceType::KNIGHT, PlayerColor::WHITE));
        p(Piece(5, 0, PieceType::KNIGHT, PlayerColor::WHITE));
        p(Piece(3, 0, PieceType::KING, PlayerColor::WHITE));
        p(Piece(4, 0, PieceType::QUEEN, PlayerColor::WHITE));
        for i in 0..8 {
            p(Piece(i, 6, PieceType::PAWN, PlayerColor::BLACK));
        }
        p(Piece(0, 7, PieceType::ROOK, PlayerColor::BLACK));
        p(Piece(7, 7, PieceType::ROOK, PlayerColor::BLACK));
        p(Piece(1, 7, PieceType::BISHOP, PlayerColor::BLACK));
        p(Piece(6, 7, PieceType::BISHOP, PlayerColor::BLACK));
        p(Piece(2, 7, PieceType::KNIGHT, PlayerColor::BLACK));
        p(Piece(5, 7, PieceType::KNIGHT, PlayerColor::BLACK));
        p(Piece(3, 7, PieceType::KING, PlayerColor::BLACK));
        p(Piece(4, 7, PieceType::QUEEN, PlayerColor::BLACK));
        Ok(BoardState { pieces })
    }
}
pub struct Piece(pub u8, pub u8, pub PieceType, pub PlayerColor);
#[derive(PartialEq)]
pub enum PieceType {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    KING,
    QUEEN,
}
#[derive(PartialEq)]
pub enum PlayerColor {
    BLACK,
    WHITE,
}
