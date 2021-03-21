#[allow(dead_code)]
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
        for i in 0..8 {
            pieces.push(Piece(i, 1, PieceType::PAWN, PlayerColor::WHITE));
        }
        Ok(BoardState { pieces })
    }
}
pub struct Piece(pub u8, pub u8, pub PieceType, pub PlayerColor);
#[allow(dead_code)]
#[derive(PartialEq)]
pub enum PieceType {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    KING,
    QUEEN,
}
#[allow(dead_code)]
#[derive(PartialEq)]
pub enum PlayerColor {
    BLACK,
    WHITE,
}
