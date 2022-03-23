use tetra::math::Vec2;
#[inline(always)]
pub fn is_within_chessboard(p: Vec2<i8>) -> bool {
    // TODO: cache
    !(p.x >= 8 || p.y >= 8 || p.x < 0 || p.y < 0)
}
pub struct GameHistory {
    pub board_states: Vec<BoardState>,
    pub moves: Vec<ChessMove>,
}
impl GameHistory {
    pub fn new_game() -> tetra::Result<GameHistory> {
        // let board_states = vec![BoardState::default_board()?];
        let board_states = vec![BoardState::test_board_1()?];
        let moves = Vec::new();
        Ok(GameHistory {
            board_states,
            moves,
        })
    }
    /**
     * Assumes the move is already checked
     */
    pub fn on_piece_taken(&mut self) -> () {
        // TODO: print something nice to the screen
        println!("I've taken a piece");
    }
    // TODO: sync BoardState.color with GameHistory.initial_p_to_move
    pub fn execute_move(&mut self, mv: ChessMove) {
        self.moves.push(mv);
        let mut new_state = self.board_states.last_mut().unwrap().clone();
        match new_state.pieces.iter().position(|x| *x == mv.from) {
            Some(k) => {
                new_state.pieces.remove(k);
            }
            None => {
                // TODO: cheater or bug, do something fun
            }
        }
        match new_state
            .pieces
            .iter()
            .position(|x| x.x == mv.to.x && x.y == mv.to.y)
        {
            Some(k) => {
                new_state.pieces.remove(k);
                self.on_piece_taken()
            }
            None => {}
        }
        new_state.pieces.push(mv.to);

        if new_state.player_to_move == PlayerColor::WHITE {
            new_state.player_to_move = PlayerColor::BLACK;
        } else {
            new_state.player_to_move = PlayerColor::WHITE;
        }

        self.board_states.push(new_state);
    }
}
#[derive(PartialEq)]
pub enum MovePlausibility {
    MOVE,
    TAKES,
    IMPOSSIBLE,
}
#[derive(Clone)]
pub struct BoardState {
    pub pieces: Vec<Piece>,
    pub player_to_move: PlayerColor,
    pub white_can_castle_q: bool,
    pub white_can_castle_k: bool,
    pub black_can_castle_q: bool,
    pub black_can_castle_k: bool,
}
impl BoardState {
    pub fn create(
        pieces: Vec<Piece>,
        player_to_move: PlayerColor,
        white_can_castle_q: bool,
        white_can_castle_k: bool,
        black_can_castle_q: bool,
        black_can_castle_k: bool,
    ) -> BoardState {
        BoardState {
            pieces: pieces,
            player_to_move: player_to_move.to_owned(),
            white_can_castle_q: white_can_castle_q,
            white_can_castle_k: white_can_castle_k,
            black_can_castle_q: black_can_castle_q,
            black_can_castle_k: black_can_castle_k,
        }
    }
    #[allow(dead_code)]
    fn test_board_2() -> tetra::Result<BoardState> {
        let mut pieces = Vec::new();
        let mut p = |i| pieces.push(i);
        p(construct_piece(1, 4, PieceType::KNIGHT, PlayerColor::WHITE));
        p(construct_piece(6, 4, PieceType::KING, PlayerColor::WHITE));
        p(construct_piece(5, 3, PieceType::ROOK, PlayerColor::WHITE));
        p(construct_piece(3, 1, PieceType::QUEEN, PlayerColor::WHITE));
        p(construct_piece(6, 2, PieceType::KING, PlayerColor::BLACK));
        p(construct_piece(4, 4, PieceType::BISHOP, PlayerColor::BLACK));
        p(construct_piece(6, 6, PieceType::PAWN, PlayerColor::BLACK));
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
        p(construct_piece(1, 4, PieceType::KNIGHT, PlayerColor::WHITE));
        p(construct_piece(6, 4, PieceType::KING, PlayerColor::BLACK));
        p(construct_piece(0, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(construct_piece(7, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(construct_piece(3, 2, PieceType::QUEEN, PlayerColor::WHITE));
        p(construct_piece(4, 0, PieceType::KING, PlayerColor::WHITE));
        p(construct_piece(5, 3, PieceType::BISHOP, PlayerColor::BLACK));
        p(construct_piece(6, 6, PieceType::PAWN, PlayerColor::BLACK));
        Ok(BoardState::create(
            pieces,
            PlayerColor::WHITE,
            false,
            false,
            false,
            false,
        ))
    }
    pub fn get_piece_at_square(&self, pos: Vec2<i8>) -> Option<Piece> {
        for l in &self.pieces {
            if l.x == pos.x && l.y == pos.y {
                return Some(*l);
            }
        }
        None
    }
    fn evaluate_can_king_side_castle(&self, player_color: PlayerColor) -> bool {
        // TODO: test
        let y = match player_color {
            PlayerColor::WHITE => 0,
            PlayerColor::BLACK => 7,
        };

        if player_color == PlayerColor::WHITE && self.white_can_castle_k == false {
            return false;
        }
        if player_color == PlayerColor::BLACK && !self.black_can_castle_k {
            return false;
        }
        // TODO: check for attacked squares.
        if self.white_can_castle_q
            && self.get_piece_at_square(Vec2::new(4, y)).is_some()
            && self
                .get_piece_at_square(Vec2::new(4, y))
                .unwrap()
                .piece_type
                == PieceType::KING
            && self.get_piece_at_square(Vec2::new(7, y)).is_some()
            && self
                .get_piece_at_square(Vec2::new(7, y))
                .unwrap()
                .piece_type
                == PieceType::ROOK
            && self.get_piece_at_square(Vec2::new(5, y)).is_none()
            && self.get_piece_at_square(Vec2::new(6, y)).is_none()
        {
            return true;
        }
        false
    }
    fn evaluate_can_queen_side_castle(&self, player_color: PlayerColor) -> bool {
        // TODO: test
        let mut y = 0;
        if player_color == PlayerColor::BLACK {
            y = 7;
        }

        if player_color == PlayerColor::WHITE && self.white_can_castle_q == false {
            return false;
        }
        if player_color == PlayerColor::BLACK
        // TODO: replace with history.now() or GameContainer::position()
            && self.black_can_castle_q
                == false
        {
            return false;
        }

        // TODO: check for attacked squares.
        let piece_there_ey = self.get_piece_at_square(Vec2::new(4, y));
        let piece_there_ay = self.get_piece_at_square(Vec2::new(0, y));
        if self.white_can_castle_q
            && piece_there_ey.is_some()
            && piece_there_ey.unwrap().piece_type == PieceType::KING
            && piece_there_ay.is_some()
            && piece_there_ay.unwrap().piece_type == PieceType::ROOK
            && self.get_piece_at_square(Vec2::new(1, y)).is_none()
            && self.get_piece_at_square(Vec2::new(2, y)).is_none()
        {
            return true;
        }
        false
    }
    pub fn get_move_position_plausibility(&self, p: Vec2<i8>) -> MovePlausibility {
        // TODO: use 2D array to precompute unoccupied squares
        // FIXME: retarded clone() usage
        if !is_within_chessboard(p) {
            return MovePlausibility::IMPOSSIBLE;
        }
        if let Some(piece) = self.get_piece_at_square(p) {
            if piece.color == self.player_to_move {
                return MovePlausibility::IMPOSSIBLE;
            }
            return MovePlausibility::TAKES;
        }

        MovePlausibility::MOVE
    }
    pub fn is_check(&self) -> bool {
        // TODO:
        false
    }
    #[allow(dead_code)]
    fn default_board() -> tetra::Result<BoardState> {
        let mut pieces = Vec::new();
        let mut p = |i| pieces.push(i);
        for i in 0..8 {
            p(construct_piece(i, 1, PieceType::PAWN, PlayerColor::WHITE));
        }
        p(construct_piece(0, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(construct_piece(7, 0, PieceType::ROOK, PlayerColor::WHITE));
        p(construct_piece(2, 0, PieceType::BISHOP, PlayerColor::WHITE));
        p(construct_piece(5, 0, PieceType::BISHOP, PlayerColor::WHITE));
        p(construct_piece(1, 0, PieceType::KNIGHT, PlayerColor::WHITE));
        p(construct_piece(6, 0, PieceType::KNIGHT, PlayerColor::WHITE));
        p(construct_piece(4, 0, PieceType::KING, PlayerColor::WHITE));
        p(construct_piece(3, 0, PieceType::QUEEN, PlayerColor::WHITE));
        for i in 0..8 {
            p(construct_piece(i, 6, PieceType::PAWN, PlayerColor::BLACK));
        }
        p(construct_piece(0, 7, PieceType::ROOK, PlayerColor::BLACK));
        p(construct_piece(7, 7, PieceType::ROOK, PlayerColor::BLACK));
        p(construct_piece(2, 7, PieceType::BISHOP, PlayerColor::BLACK));
        p(construct_piece(5, 7, PieceType::BISHOP, PlayerColor::BLACK));
        p(construct_piece(1, 7, PieceType::KNIGHT, PlayerColor::BLACK));
        p(construct_piece(6, 7, PieceType::KNIGHT, PlayerColor::BLACK));
        p(construct_piece(4, 7, PieceType::KING, PlayerColor::BLACK));
        p(construct_piece(3, 7, PieceType::QUEEN, PlayerColor::BLACK));
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
#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub x: i8,
    pub y: i8,
    pub piece_type: PieceType,
    pub color: PlayerColor,
}

pub fn construct_piece(x: i8, y: i8, piece_type: PieceType, color: PlayerColor) -> Piece {
    Piece {
        x: x,
        y: y,
        piece_type,
        color: color,
    }
}
#[derive(Clone, Copy)]
pub struct ChessMove {
    pub from: Piece,
    pub to: Piece,
}
impl ChessMove {
    pub fn new(from: Piece, to: Piece) -> ChessMove {
        ChessMove { from: from, to: to }
    }
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
impl PlayerColor {
    pub fn opposite(color: PlayerColor) -> PlayerColor {
        if color == PlayerColor::WHITE {
            return PlayerColor::BLACK;
        }
        PlayerColor::WHITE
    }
}
