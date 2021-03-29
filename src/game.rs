use crate::Scene;
use tetra::math::Vec2;

pub struct GameContainer {
    history: GameHistory, // data
}

impl GameContainer {
    pub fn new() -> tetra::Result<GameContainer> {
        let history = GameHistory::new_game()?;
        Ok(GameContainer { history })
    }
    pub fn current_pieces(&mut self) -> &Vec<Piece> {
        self.history.board_states.last_mut();
        &self.history.board_states.last_mut().unwrap().pieces
    }
    pub fn get_piece_at(&mut self, pos: Vec2<i8>) -> Option<Piece> {
        let list = self.current_pieces();
        for l in list {
            if l.0 == pos.x && l.1 == pos.y {
                return Some(*l);
            }
        }
        None
    }
    fn isnt_check(&mut self, p: Piece) -> bool {
        let pcs = self.current_pieces().clone();
        // TODO: use 2D array to precompute attacked squares
        let mut legal_moves = Vec::<Piece>::new();
        for piece in pcs {
            if piece.2 == PieceType::KING || p.3 == piece.3 {
                // FIXME: king gets ignored, else stack overflow
                continue;
            }
            {
                legal_moves.append(&mut self.get_legal_moves(piece));
            }
        }
        for mv in legal_moves {
            if mv.0 == p.1 && mv.1 == p.1 && p.3 != mv.3 {
                return false;
            }
        }
        true
    }
    pub fn get_legal_moves(&mut self, p: Piece) -> Vec<Piece> {
        // FIXME: detect illegal positions
        // TODO: make this a separate method
        let pcs = self.current_pieces().clone();
        let check_boundaries = |p: Vec2<i8>| {
            // TODO: cache
            if p.x >= 8 || p.y >= 8 || p.x < 0 || p.y < 0 {
                return false;
            }
            true
        };
        let check_move = |p: Vec2<i8>| -> bool {
            // TODO: use 2D array to precompute unoccupied squares
            if !check_boundaries(p) {
                return false;
            }
            for piece in pcs.clone() {
                if piece.0 == p.x && piece.1 == p.y {
                    return false;
                }
            }
            true
        };
        let mut moves = Vec::<Piece>::new();
        match p.2 {
            // TODO: abstract
            PieceType::BISHOP => {
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.0 + i, p.1 + i);
                    if !check_move(pos) {
                        match self.get_piece_at(pos) {
                            Some(i) => {
                                if i.3 != p.3 {
                                    moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                                }
                            }
                            None => {}
                        }
                        break;
                    } else {
                        moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.0 + i, p.1 - i);
                    if !check_move(pos) {
                        match self.get_piece_at(pos) {
                            Some(i) => {
                                if i.3 != p.3 {
                                    moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                                }
                            }
                            None => {}
                        }
                        break;
                    } else {
                        moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.0 - i, p.1 + i);
                    if !check_move(pos) {
                        match self.get_piece_at(pos) {
                            Some(i) => {
                                if i.3 != p.3 {
                                    moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                                }
                            }
                            None => {}
                        }
                        break;
                    } else {
                        moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.0 - i, p.1 - i);
                    if !check_move(pos) {
                        match self.get_piece_at(pos) {
                            Some(i) => {
                                if i.3 != p.3 {
                                    moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                                }
                            }
                            None => {}
                        }
                        break;
                    } else {
                        moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                    }
                }
            }
            PieceType::KNIGHT => {
                let positions = vec![
                    Vec2::new(p.0 + 2, p.1 + 1),
                    Vec2::new(p.0 - 2, p.1 + 1),
                    Vec2::new(p.0 + 2, p.1 - 1),
                    Vec2::new(p.0 + 1, p.1 + 2),
                    Vec2::new(p.0 - 1, p.1 + 2),
                    Vec2::new(p.0 - 1, p.1 - 2),
                    Vec2::new(p.0 + 1, p.1 - 2),
                ];
                for pos in positions {
                    if check_boundaries(pos) {
                        match self.get_piece_at(pos) {
                            Some(i) => {
                                if i.3 == p.3 {
                                    continue;
                                }
                            }
                            None => {}
                        }
                        moves.push(Piece(pos.x, pos.y, PieceType::KNIGHT, p.3));
                    }
                }
            }
            PieceType::KING => {
                // TODO: castles
                let positions = vec![
                    Vec2::new(p.0, p.1 + 1),
                    Vec2::new(p.0, p.1 - 1),
                    Vec2::new(p.0 + 1, p.1 - 1),
                    Vec2::new(p.0 + 1, p.1 + 1),
                    Vec2::new(p.0 - 1, p.1 + 1),
                    Vec2::new(p.0 - 1, p.1 - 1),
                    Vec2::new(p.0 + 1, p.1),
                    Vec2::new(p.0 - 1, p.1),
                ];
                // FIXME: what if it is check??
                // FIXME: checks are messed up
                if self.isnt_check(p) {
                    for pos in positions {
                        if self.isnt_check(Piece(pos.x, pos.y, PieceType::KING, p.3)) {
                            moves.push(Piece(pos.x, pos.y, PieceType::KING, p.3));
                        }
                    }
                }
            }
            PieceType::ROOK => {
                // TODO: abstract
                for i in 1..8 {
                    let pos = Vec2::new(p.0 + i, p.1);
                    if check_boundaries(pos) {
                        match self.get_piece_at(pos) {
                            Some(piece) => {
                                if piece.3 != p.3 {
                                    moves.push(Piece(pos.x, pos.y, PieceType::ROOK, p.3));
                                }
                                break;
                            }
                            None => moves.push(Piece(pos.x, pos.y, PieceType::ROOK, p.3)),
                        }
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.0 - i, p.1);
                    if check_boundaries(pos) {
                        match self.get_piece_at(pos) {
                            Some(piece) => {
                                if piece.3 != p.3 {
                                    moves.push(Piece(pos.x, pos.y, PieceType::ROOK, p.3));
                                }
                                break;
                            }
                            None => moves.push(Piece(pos.x, pos.y, PieceType::ROOK, p.3)),
                        }
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.0, p.1 + i);
                    if check_boundaries(pos) {
                        match self.get_piece_at(pos) {
                            Some(piece) => {
                                if piece.3 != p.3 {
                                    moves.push(Piece(pos.x, pos.y, PieceType::ROOK, p.3));
                                }
                                break;
                            }
                            None => moves.push(Piece(pos.x, pos.y, PieceType::ROOK, p.3)),
                        }
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.0, p.1 - i);
                    if check_boundaries(pos) {
                        match self.get_piece_at(pos) {
                            Some(piece) => {
                                if piece.3 != p.3 {
                                    moves.push(Piece(pos.x, pos.y, PieceType::ROOK, p.3));
                                }
                                break;
                            }
                            None => moves.push(Piece(pos.x, pos.y, PieceType::ROOK, p.3)),
                        }
                    }
                }
            }
            PieceType::QUEEN => {
                let mut positions = Vec::<Piece>::new();
                positions.append(&mut self.get_legal_moves(Piece(
                    p.0,
                    p.1,
                    PieceType::BISHOP,
                    p.3,
                )));
                positions.append(&mut self.get_legal_moves(Piece(p.0, p.1, PieceType::ROOK, p.3)));
                for pos in positions {
                    moves.push(Piece(pos.0, pos.1, PieceType::QUEEN, p.3));
                }
            }
            PieceType::PAWN => {
                // TODO: en passant
                // TODO: promotions
                // taking
                match p.3 {
                    PlayerColor::WHITE => match self.get_piece_at(Vec2::new(p.0 - 1, p.1 + 1)) {
                        None => {}
                        Some(i) => {
                            if i.3 != p.3 {
                                moves.push(Piece(p.0 - 1, p.1 + 1, PieceType::PAWN, p.3));
                            }
                        }
                    },
                    PlayerColor::BLACK => match self.get_piece_at(Vec2::new(p.0 - 1, p.1 - 1)) {
                        None => {}
                        Some(i) => {
                            if i.3 != p.3 {
                                moves.push(Piece(p.0 - 1, p.1 - 1, PieceType::PAWN, p.3));
                            }
                        }
                    },
                }
                match p.3 {
                    PlayerColor::WHITE => match self.get_piece_at(Vec2::new(p.0 + 1, p.1 + 1)) {
                        None => {}
                        Some(i) => {
                            if i.3 != p.3 {
                                moves.push(Piece(p.0 + 1, p.1 + 1, PieceType::PAWN, p.3));
                            }
                        }
                    },
                    PlayerColor::BLACK => match self.get_piece_at(Vec2::new(p.0 + 1, p.1 - 1)) {
                        None => {}
                        Some(i) => {
                            if i.3 != p.3 {
                                moves.push(Piece(p.0 + 1, p.1 - 1, PieceType::PAWN, p.3));
                            }
                        }
                    },
                }
                // normal move
                match p.3 {
                    PlayerColor::WHITE => {
                        match self.get_piece_at(Vec2::new(p.0, p.1 + 1)) {
                            None => {
                                moves.push(Piece(p.0, p.1 + 1, p.2, p.3));
                                // double step
                                if p.1 == 1 {
                                    match self.get_piece_at(Vec2::new(p.0, p.1 + 2)) {
                                        Some(_) => {}
                                        None => {
                                            moves.push(Piece(p.0, p.1 + 2, PieceType::PAWN, p.3))
                                        }
                                    }
                                }
                            }
                            Some(_) => {}
                        }
                    }
                    PlayerColor::BLACK => match self.get_piece_at(Vec2::new(p.0, p.1 - 1)) {
                        None => {
                            moves.push(Piece(p.0, p.1 - 1, p.2, p.3));
                            // double step
                            if p.1 == 6 {
                                match self.get_piece_at(Vec2::new(p.0, p.1 - 2)) {
                                    Some(_) => {}
                                    None => moves.push(Piece(p.0, p.1 - 2, PieceType::PAWN, p.3)),
                                }
                            }
                        }
                        Some(_) => {}
                    },
                }
            }
        }
        moves
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
        p(Piece(2, 0, PieceType::BISHOP, PlayerColor::WHITE));
        p(Piece(5, 0, PieceType::BISHOP, PlayerColor::WHITE));
        // DEBUG:
        p(Piece(4, 4, PieceType::BISHOP, PlayerColor::BLACK));
        // DEBUG:
        p(Piece(1, 4, PieceType::KNIGHT, PlayerColor::WHITE));
        // DEBUG:
        p(Piece(6, 4, PieceType::KING, PlayerColor::WHITE));
        // DEBUG:
        p(Piece(5, 3, PieceType::ROOK, PlayerColor::WHITE));
        // DEBUG:
        p(Piece(3, 2, PieceType::QUEEN, PlayerColor::WHITE));
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
#[derive(Clone, Copy)]
pub struct Piece(pub i8, pub i8, pub PieceType, pub PlayerColor);
#[derive(PartialEq, Clone, Copy)]
pub enum PieceType {
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    KING,
    QUEEN,
}
#[derive(PartialEq, Clone, Copy)]
pub enum PlayerColor {
    BLACK,
    WHITE,
}
