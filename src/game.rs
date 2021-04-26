use crate::game_types::*;
use tetra::math::Vec2;

pub struct GameContainer {
    history: GameHistory, // data
}

impl GameContainer {
    pub fn new() -> tetra::Result<GameContainer> {
        let history = GameHistory::new_game()?;
        Ok(GameContainer { history })
    }
    pub fn execute_move(&mut self, mv: ChessMove) -> Option<ChessMove> {
        // check for player color
        let current_color;
        match self.history.moves.last() {
            Some(m) => {
                current_color = if m.from.3 == PlayerColor::BLACK {
                    PlayerColor::WHITE
                } else {
                    PlayerColor::BLACK
                }
            }
            None => current_color = self.history.initial_p_to_move,
        }
        // check if consistent
        if mv.from.3 != current_color {
            // TODO: this means we have a cheater or a bug, do something clever here...
            println!("Big very bug");
            return None;
        }
        self.history.execute_move(mv);
        Some(mv)
    }
    pub fn current_pieces(&mut self) -> &Vec<Piece> {
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
        // NOTE: can be done simpler
        // FIXME: cloning here is stupid
        let pcs = self.current_pieces().clone();
        // TODO: use 2D array to precompute attacked squares
        for piece in pcs {
            if p.3 == piece.3 {
            } else if piece.2 == PieceType::KING {
                let moves = vec![
                    Vec2::new(piece.0, piece.1 - 1),
                    Vec2::new(piece.0, piece.1 + 1),
                    Vec2::new(piece.0 - 1, piece.1),
                    Vec2::new(piece.0 + 1, piece.1),
                    Vec2::new(piece.0 - 1, piece.1 + 1),
                    Vec2::new(piece.0 + 1, piece.1 + 1),
                    Vec2::new(piece.0 - 1, piece.1 - 1),
                    Vec2::new(piece.0 + 1, piece.1 - 1),
                ];
                for mv in moves {
                    if self.check_boundaries(mv) && mv.x == p.0 && mv.y == p.1 {
                        return false;
                    }
                }
            } else if piece.2 == PieceType::PAWN {
                let color_mult: i8 = if piece.3 == PlayerColor::WHITE { 1 } else { -1 };
                // TODO: optimize
                if (self.check_boundaries(Vec2::new(piece.0 + 1, piece.1 + color_mult))
                    && piece.0 + 1 == p.0
                    && piece.1 + color_mult == p.1)
                    || (self.check_boundaries(Vec2::new(piece.0 - 1, piece.1 + color_mult))
                        && piece.0 - 1 == p.0
                        && piece.1 + color_mult == p.1)
                {
                    return false;
                }
            } else {
                for mv in self.get_legal_moves(piece) {
                    if mv.0 == p.0 && mv.1 == p.1 {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn check_boundaries(&mut self, p: Vec2<i8>) -> bool {
        // TODO: cache
        if p.x >= 8 || p.y >= 8 || p.x < 0 || p.y < 0 {
            return false;
        }
        true
    }
    /*
    checks whether a square is occupied or isn't on the chessboard
    @return true means the square with given coordinates are plausible
    */
    fn check_move(&mut self, p: Vec2<i8>) -> bool {
        // TODO: use 2D array to precompute unoccupied squares
        // FIXME: retarded clone() usage
        let pcs = self.current_pieces().clone();
        if !self.check_boundaries(p) {
            return false;
        }
        for piece in pcs.clone() {
            if piece.0 == p.x && piece.1 == p.y {
                return false;
            }
        }
        true
    }
    fn has_moved(&mut self, pos: Vec2<i8>) -> bool {
        // FIXME: you can always castle if king and a rook are on the default positions
        match self.get_piece_at(pos).unwrap().2 {
            PieceType::BISHOP => return true,
            PieceType::KNIGHT => return true,
            PieceType::QUEEN => return true,
            _ => return false,
        }
    }
    pub fn get_legal_moves(&mut self, p: Piece) -> Vec<Piece> {
        // FIXME: detect illegal positions, including ignored checks, pawns on first ranks, castles
        let mut moves = Vec::<Piece>::new();
        match p.2 {
            // TODO: abstract
            PieceType::BISHOP => {
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.0 + i, p.1 + i);
                    if !self.check_move(pos) {
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
                    if !self.check_move(pos) {
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
                    if !self.check_move(pos) {
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
                    if !self.check_move(pos) {
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
                    if self.check_boundaries(pos) {
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
                for pos in positions {
                    let temp = Piece(pos.x, pos.y, PieceType::KING, p.3);
                    if self.check_move(Vec2::new(temp.0, temp.1)) && self.isnt_check(temp) {
                        moves.push(temp);
                    }
                }
                if self.isnt_check(p) {
                    match p.3 {
                        PlayerColor::WHITE => {
                            // FIXME: DRY
                            // queen side castle
                            if self.get_piece_at(Vec2::new(4, 0)).is_some()
                                && self.get_piece_at(Vec2::new(4, 0)).unwrap().2 == PieceType::KING
                                && !self.has_moved(Vec2::new(4, 0))
                                && self.get_piece_at(Vec2::new(0, 0)).is_some()
                                && self.get_piece_at(Vec2::new(0, 0)).unwrap().2 == PieceType::ROOK
                                && !self.has_moved(Vec2::new(0, 0))
                                && self.get_piece_at(Vec2::new(1, 0)).is_none()
                                && self.get_piece_at(Vec2::new(2, 0)).is_none()
                                && self.get_piece_at(Vec2::new(3, 0)).is_none()
                                && self.isnt_check(Piece(2, 0, PieceType::KING, PlayerColor::WHITE))
                                && self.isnt_check(Piece(3, 0, PieceType::KING, PlayerColor::WHITE))
                            {
                                moves.push(Piece(2, 0, PieceType::KING, p.3));
                            }
                            // king side castle
                            if self.get_piece_at(Vec2::new(4, 0)).is_some()
                                && self.get_piece_at(Vec2::new(4, 0)).unwrap().2 == PieceType::KING
                                && !self.has_moved(Vec2::new(4, 0))
                                && self.get_piece_at(Vec2::new(7, 0)).is_some()
                                && self.get_piece_at(Vec2::new(7, 0)).unwrap().2 == PieceType::ROOK
                                && !self.has_moved(Vec2::new(7, 0))
                                && self.get_piece_at(Vec2::new(5, 0)).is_none()
                                && self.get_piece_at(Vec2::new(6, 0)).is_none()
                                && self.isnt_check(Piece(6, 0, PieceType::KING, PlayerColor::WHITE))
                                && self.isnt_check(Piece(5, 0, PieceType::KING, PlayerColor::WHITE))
                            {
                                moves.push(Piece(6, 0, PieceType::KING, PlayerColor::WHITE));
                            }
                        }
                        PlayerColor::BLACK => {
                            if self.get_piece_at(Vec2::new(4, 7)).is_some()
                                && self.get_piece_at(Vec2::new(4, 7)).unwrap().2 == PieceType::KING
                                && !self.has_moved(Vec2::new(4, 7))
                                && self.get_piece_at(Vec2::new(0, 7)).is_some()
                                && self.get_piece_at(Vec2::new(0, 7)).unwrap().2 == PieceType::ROOK
                                && !self.has_moved(Vec2::new(0, 7))
                                && self.get_piece_at(Vec2::new(1, 7)).is_none()
                                && self.get_piece_at(Vec2::new(2, 7)).is_none()
                                && self.get_piece_at(Vec2::new(3, 7)).is_none()
                                && self.isnt_check(Piece(2, 7, PieceType::KING, PlayerColor::BLACK))
                                && self.isnt_check(Piece(3, 7, PieceType::KING, PlayerColor::WHITE))
                            {
                                moves.push(Piece(2, 7, PieceType::KING, PlayerColor::BLACK));
                            }
                            // king side castle
                            if self.get_piece_at(Vec2::new(4, 7)).is_some()
                                && self.get_piece_at(Vec2::new(4, 7)).unwrap().2 == PieceType::KING
                                && !self.has_moved(Vec2::new(4, 7))
                                && self.get_piece_at(Vec2::new(7, 7)).is_some()
                                && self.get_piece_at(Vec2::new(7, 7)).unwrap().2 == PieceType::ROOK
                                && !self.has_moved(Vec2::new(7, 7))
                                && self.get_piece_at(Vec2::new(5, 7)).is_none()
                                && self.get_piece_at(Vec2::new(6, 7)).is_none()
                                && self.isnt_check(Piece(6, 7, PieceType::KING, PlayerColor::BLACK))
                                && self.isnt_check(Piece(5, 7, PieceType::KING, PlayerColor::WHITE))
                            {
                                moves.push(Piece(6, 7, PieceType::KING, p.3));
                            }
                        }
                    }
                }
                // FIXME: what if it is check??
            }
            PieceType::ROOK => {
                // TODO: abstract, simplify
                for i in 1..8 {
                    let pos = Vec2::new(p.0 + i, p.1);
                    if self.check_boundaries(pos) {
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
                    if self.check_boundaries(pos) {
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
                    if self.check_boundaries(pos) {
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
                    if self.check_boundaries(pos) {
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
