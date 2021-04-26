pub struct GameHistory {
    pub board_states: Vec<BoardState>,
    pub moves: Vec<ChessMove>,
    pub initial_p_to_move: PlayerColor,
}
impl GameHistory {
    pub fn new_game() -> tetra::Result<GameHistory> {
        // let board_states = vec![BoardState::default_board()?];
        let board_states = vec![BoardState::test_board_1()?];
        let moves = Vec::new();
        let initial_p_to_move = PlayerColor::WHITE;
        Ok(GameHistory {
            board_states,
            moves,
            initial_p_to_move,
        })
    }
    /**
     * Assumes the move is already checked
     */
    pub fn execute_move(&mut self, mv: ChessMove) {
        self.moves.push(mv);
        let mut new_state = self.board_states.last_mut().unwrap().clone();
        let index = new_state.pieces.iter().position(|x| *x == mv.from).unwrap();
        new_state.pieces.remove(index);
        // FIXME: mv.to can be other piece, FIXME for the FIXME: probably isn't the case
        new_state.pieces.push(mv.to);
        self.board_states.push(new_state);
    }
}
#[derive(Clone)]
pub struct BoardState {
    pub pieces: Vec<Piece>,
}
impl BoardState {
    #[allow(dead_code)]
    fn test_board_2() -> tetra::Result<BoardState> {
        let mut pieces = Vec::new();
        let mut p = |i| pieces.push(i);
        p(Piece(1, 4, PieceType::KNIGHT, PlayerColor::WHITE));
        p(Piece(6, 4, PieceType::KING, PlayerColor::WHITE));
        p(Piece(5, 3, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece(3, 1, PieceType::QUEEN, PlayerColor::WHITE));
        p(Piece(6, 2, PieceType::KING, PlayerColor::BLACK));
        p(Piece(4, 4, PieceType::BISHOP, PlayerColor::BLACK));
        p(Piece(6, 6, PieceType::PAWN, PlayerColor::BLACK));
        Ok(BoardState { pieces })
    }
    fn test_board_1() -> tetra::Result<BoardState> {
        let mut pieces = Vec::new();
        let mut p = |i| pieces.push(i);
        p(Piece(1, 4, PieceType::KNIGHT, PlayerColor::WHITE));
        p(Piece(6, 4, PieceType::KING, PlayerColor::BLACK));
        p(Piece(0, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece(7, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece(3, 2, PieceType::QUEEN, PlayerColor::WHITE));
        p(Piece(4, 0, PieceType::KING, PlayerColor::WHITE));
        p(Piece(5, 3, PieceType::BISHOP, PlayerColor::BLACK));
        p(Piece(6, 6, PieceType::PAWN, PlayerColor::BLACK));
        Ok(BoardState { pieces })
    }
    #[allow(dead_code)]
    fn default_board() -> tetra::Result<BoardState> {
        let mut pieces = Vec::new();
        let mut p = |i| pieces.push(i);
        for i in 0..8 {
            p(Piece(i, 1, PieceType::PAWN, PlayerColor::WHITE));
        }
        p(Piece(0, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece(7, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(Piece(2, 0, PieceType::BISHOP, PlayerColor::WHITE));
        p(Piece(5, 0, PieceType::BISHOP, PlayerColor::WHITE));
        p(Piece(1, 0, PieceType::KNIGHT, PlayerColor::WHITE));
        p(Piece(6, 0, PieceType::KNIGHT, PlayerColor::WHITE));
        p(Piece(4, 0, PieceType::KING, PlayerColor::WHITE));
        p(Piece(3, 0, PieceType::QUEEN, PlayerColor::WHITE));
        for i in 0..8 {
            p(Piece(i, 6, PieceType::PAWN, PlayerColor::BLACK));
        }
        p(Piece(0, 7, PieceType::ROOK, PlayerColor::BLACK));
        p(Piece(7, 7, PieceType::ROOK, PlayerColor::BLACK));
        p(Piece(2, 7, PieceType::BISHOP, PlayerColor::BLACK));
        p(Piece(5, 7, PieceType::BISHOP, PlayerColor::BLACK));
        p(Piece(1, 7, PieceType::KNIGHT, PlayerColor::BLACK));
        p(Piece(6, 7, PieceType::KNIGHT, PlayerColor::BLACK));
        p(Piece(4, 7, PieceType::KING, PlayerColor::BLACK));
        p(Piece(3, 7, PieceType::QUEEN, PlayerColor::BLACK));
        Ok(BoardState { pieces })
    }
}
#[derive(Clone, Copy, PartialEq)]
pub struct Piece(pub i8, pub i8, pub PieceType, pub PlayerColor);
#[derive(Clone, Copy)]
pub struct ChessMove {
    pub from: Piece,
    pub to: Piece,
}
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PieceType {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    KING,
    QUEEN,
}
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerColor {
    BLACK,
    WHITE,
}
