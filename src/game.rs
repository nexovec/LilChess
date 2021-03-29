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
    pub fn get_legal_moves(&mut self, p: Piece) -> Vec<Piece> {
        // TODO:
        let pcs = self.current_pieces();
        let check_move = |p: Vec2<i8>| -> bool {
            if p.x >= 8 || p.y >= 8 || p.x < 0 || p.y < 0 {
                return false;
            }
            for piece in pcs {
                if piece.0 == p.x && piece.1 == p.y {
                    // DEBUG:
                    println!("I'm returning false");
                    return false;
                } else {
                    println!("valid move: {:?}", p);
                }
            }
            true
        };
        let mut moves = Vec::<Piece>::new();
        match p.2 {
            PieceType::BISHOP => {
                // DEBUG:
                println!("I should print when bishop is clicked");
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.0 + i, p.1 + i);
                    if !check_move(pos) {
                        break;
                    } else {
                        moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.0 + i, p.1 - i);
                    if !check_move(pos) {
                        break;
                    } else {
                        moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.0 - i, p.1 + i);
                    if !check_move(pos) {
                        break;
                    } else {
                        moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.0 - i, p.1 - i);
                    if !check_move(pos) {
                        break;
                    } else {
                        moves.push(Piece(pos.x, pos.y, PieceType::BISHOP, p.3));
                    }
                }
            }
            PieceType::KNIGHT => {}
            PieceType::KING => {}
            PieceType::QUEEN => {}
            PieceType::ROOK => {}
            PieceType::PAWN => {}
        }
        // moves.push(Piece(5, 3, PieceType::PAWN, PlayerColor::WHITE)); // this works, but the rest doesn't??
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
        p(Piece(4, 4, PieceType::BISHOP, PlayerColor::WHITE));
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
