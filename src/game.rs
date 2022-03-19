use crate::game_types::*;
use tetra::math::Vec2;

pub struct GameContainer {
    pub history: GameHistory, // data
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
                current_color = if m.from.color == PlayerColor::BLACK {
                    PlayerColor::WHITE
                } else {
                    PlayerColor::BLACK
                }
            }
            None => current_color = self.history.initial_p_to_move,
        }
        // check if the right player is attempting the move
        if mv.from.color != current_color {
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
    pub fn get_piece_at_square(&mut self, pos: Vec2<i8>) -> Option<Piece> {
        let list = self.current_pieces();
        for l in list {
            if l.x == pos.x && l.y == pos.y {
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
            if p.color == piece.color {
            } else if piece.piece_type == PieceType::KING {
                let moves = vec![
                    Vec2::new(piece.x, piece.y - 1),
                    Vec2::new(piece.x, piece.y + 1),
                    Vec2::new(piece.x - 1, piece.y),
                    Vec2::new(piece.x + 1, piece.y),
                    Vec2::new(piece.x - 1, piece.y + 1),
                    Vec2::new(piece.x + 1, piece.y + 1),
                    Vec2::new(piece.x - 1, piece.y - 1),
                    Vec2::new(piece.x + 1, piece.y - 1),
                ];
                for mv in moves {
                    if self.check_boundaries(mv) && mv.x == p.x && mv.y == p.y {
                        return false;
                    }
                }
            } else if piece.piece_type == PieceType::PAWN {
                let color_mult: i8 = if piece.color == PlayerColor::WHITE {
                    1
                } else {
                    -1
                };
                // TODO: optimize
                if (self.check_boundaries(Vec2::new(piece.x + 1, piece.y + color_mult))
                    && piece.x + 1 == p.x
                    && piece.y + color_mult == p.y)
                    || (self.check_boundaries(Vec2::new(piece.x - 1, piece.y + color_mult))
                        && piece.x - 1 == p.x
                        && piece.y + color_mult == p.y)
                {
                    return false;
                }
            } else {
                for mv in self.get_legal_moves(piece) {
                    if mv.x == p.x && mv.y == p.y {
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
            // TODO: if the piece is unguarded and of the opposite color, return true
            if piece.x == p.x && piece.y == p.y {
                return false;
            }
        }
        true
    }
    fn has_moved(&mut self, pos: Vec2<i8>) -> bool {
        // FIXME: you can always castle if king and a rook are on the default positions
        match self.get_piece_at_square(pos).unwrap().piece_type {
            PieceType::BISHOP => return true,
            PieceType::KNIGHT => return true,
            PieceType::QUEEN => return true,
            _ => return false,
        }
    }
    pub fn get_legal_moves(&mut self, p: Piece) -> Vec<Piece> {
        // FIXME: detect illegal positions, including ignored checks, pawns on first ranks, castles
        let mut moves = Vec::<Piece>::new();
        match p.piece_type {
            // TODO: abstract
            PieceType::BISHOP => {
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x + i, p.y + i);
                    if !self.check_move(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(pos.x, pos.y, PieceType::BISHOP, p.color));
                                }
                            }
                            None => {}
                        }
                        break;
                    } else {
                        moves.push(construct_piece(pos.x, pos.y, PieceType::BISHOP, p.color));
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x + i, p.y - i);
                    if !self.check_move(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(pos.x, pos.y, PieceType::BISHOP, p.color));
                                }
                            }
                            None => {}
                        }
                        break;
                    } else {
                        moves.push(construct_piece(pos.x, pos.y, PieceType::BISHOP, p.color));
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x - i, p.y + i);
                    if !self.check_move(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(pos.x, pos.y, PieceType::BISHOP, p.color));
                                }
                            }
                            None => {}
                        }
                        break;
                    } else {
                        moves.push(construct_piece(pos.x, pos.y, PieceType::BISHOP, p.color));
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x - i, p.y - i);
                    if !self.check_move(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(pos.x, pos.y, PieceType::BISHOP, p.color));
                                }
                            }
                            None => {}
                        }
                        break;
                    } else {
                        moves.push(construct_piece(pos.x, pos.y, PieceType::BISHOP, p.color));
                    }
                }
            }
            PieceType::KNIGHT => {
                let positions = vec![
                    Vec2::new(p.x + 2, p.y + 1),
                    Vec2::new(p.x - 2, p.y + 1),
                    Vec2::new(p.x + 2, p.y - 1),
                    Vec2::new(p.x + 1, p.y + 2),
                    Vec2::new(p.x - 1, p.y + 2),
                    Vec2::new(p.x - 1, p.y - 2),
                    Vec2::new(p.x + 1, p.y - 2),
                ];
                for pos in positions {
                    if self.check_boundaries(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(i) => {
                                if i.color == p.color {
                                    continue;
                                }
                            }
                            None => {}
                        }
                        moves.push(construct_piece(pos.x, pos.y, PieceType::KNIGHT, p.color));
                    }
                }
            }
            PieceType::KING => {
                let positions = vec![
                    Vec2::new(p.x, p.y + 1),
                    Vec2::new(p.x, p.y - 1),
                    Vec2::new(p.x + 1, p.y - 1),
                    Vec2::new(p.x + 1, p.y + 1),
                    Vec2::new(p.x - 1, p.y + 1),
                    Vec2::new(p.x - 1, p.y - 1),
                    Vec2::new(p.x + 1, p.y),
                    Vec2::new(p.x - 1, p.y),
                ];
                for pos in positions {
                    let temp = construct_piece(pos.x, pos.y, PieceType::KING, p.color);
                    if self.check_move(Vec2::new(temp.x, temp.y)) && self.isnt_check(temp) {
                        moves.push(temp);
                    }
                }
                if self.isnt_check(p) {
                    match p.color {
                        PlayerColor::WHITE => {
                            // FIXME: DRY
                            // queen side castle
                            if self.get_piece_at_square(Vec2::new(4, 0)).is_some()
                                && self.get_piece_at_square(Vec2::new(4, 0)).unwrap().piece_type
                                    == PieceType::KING
                                && !self.has_moved(Vec2::new(4, 0))
                                && self.get_piece_at_square(Vec2::new(0, 0)).is_some()
                                && self.get_piece_at_square(Vec2::new(0, 0)).unwrap().piece_type
                                    == PieceType::ROOK
                                && !self.has_moved(Vec2::new(0, 0))
                                && self.get_piece_at_square(Vec2::new(1, 0)).is_none()
                                && self.get_piece_at_square(Vec2::new(2, 0)).is_none()
                                && self.get_piece_at_square(Vec2::new(3, 0)).is_none()
                                && self.isnt_check(construct_piece(2, 0, PieceType::KING, PlayerColor::WHITE))
                                && self.isnt_check(construct_piece(3, 0, PieceType::KING, PlayerColor::WHITE))
                            {
                                moves.push(construct_piece(2, 0, PieceType::KING, p.color));
                            }
                            // king side castle
                            if self.get_piece_at_square(Vec2::new(4, 0)).is_some()
                                && self.get_piece_at_square(Vec2::new(4, 0)).unwrap().piece_type
                                    == PieceType::KING
                                && !self.has_moved(Vec2::new(4, 0))
                                && self.get_piece_at_square(Vec2::new(7, 0)).is_some()
                                && self.get_piece_at_square(Vec2::new(7, 0)).unwrap().piece_type
                                    == PieceType::ROOK
                                && !self.has_moved(Vec2::new(7, 0))
                                && self.get_piece_at_square(Vec2::new(5, 0)).is_none()
                                && self.get_piece_at_square(Vec2::new(6, 0)).is_none()
                                && self.isnt_check(construct_piece(6, 0, PieceType::KING, PlayerColor::WHITE))
                                && self.isnt_check(construct_piece(5, 0, PieceType::KING, PlayerColor::WHITE))
                            {
                                moves.push(construct_piece(6, 0, PieceType::KING, PlayerColor::WHITE));
                            }
                        }
                        PlayerColor::BLACK => {
                            if self.get_piece_at_square(Vec2::new(4, 7)).is_some()
                                && self.get_piece_at_square(Vec2::new(4, 7)).unwrap().piece_type
                                    == PieceType::KING
                                && !self.has_moved(Vec2::new(4, 7))
                                && self.get_piece_at_square(Vec2::new(0, 7)).is_some()
                                && self.get_piece_at_square(Vec2::new(0, 7)).unwrap().piece_type
                                    == PieceType::ROOK
                                && !self.has_moved(Vec2::new(0, 7))
                                && self.get_piece_at_square(Vec2::new(1, 7)).is_none()
                                && self.get_piece_at_square(Vec2::new(2, 7)).is_none()
                                && self.get_piece_at_square(Vec2::new(3, 7)).is_none()
                                && self.isnt_check(construct_piece(2, 7, PieceType::KING, PlayerColor::BLACK))
                                && self.isnt_check(construct_piece(3, 7, PieceType::KING, PlayerColor::WHITE))
                            {
                                moves.push(construct_piece(2, 7, PieceType::KING, PlayerColor::BLACK));
                            }
                            // king side castle
                            if self.get_piece_at_square(Vec2::new(4, 7)).is_some()
                                && self.get_piece_at_square(Vec2::new(4, 7)).unwrap().piece_type
                                    == PieceType::KING
                                && !self.has_moved(Vec2::new(4, 7))
                                && self.get_piece_at_square(Vec2::new(7, 7)).is_some()
                                && self.get_piece_at_square(Vec2::new(7, 7)).unwrap().piece_type
                                    == PieceType::ROOK
                                && !self.has_moved(Vec2::new(7, 7))
                                && self.get_piece_at_square(Vec2::new(5, 7)).is_none()
                                && self.get_piece_at_square(Vec2::new(6, 7)).is_none()
                                && self.isnt_check(construct_piece(6, 7, PieceType::KING, PlayerColor::BLACK))
                                && self.isnt_check(construct_piece(5, 7, PieceType::KING, PlayerColor::WHITE))
                            {
                                moves.push(construct_piece(6, 7, PieceType::KING, p.color));
                            }
                        }
                    }
                }
                // FIXME: what if it is check??
            }
            PieceType::ROOK => {
                // TODO: abstract, simplify
                for i in 1..8 {
                    let pos = Vec2::new(p.x + i, p.y);
                    if self.check_boundaries(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(piece) => {
                                if piece.color != p.color {
                                    moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color));
                                }
                                break;
                            }
                            None => moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color)),
                        }
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x - i, p.y);
                    if self.check_boundaries(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(piece) => {
                                if piece.color != p.color {
                                    moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color));
                                }
                                break;
                            }
                            None => moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color)),
                        }
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x, p.y + i);
                    if self.check_boundaries(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(piece) => {
                                if piece.color != p.color {
                                    moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color));
                                }
                                break;
                            }
                            None => moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color)),
                        }
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x, p.y - i);
                    if self.check_boundaries(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(piece) => {
                                if piece.color != p.color {
                                    moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color));
                                }
                                break;
                            }
                            None => moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color)),
                        }
                    }
                }
            }
            PieceType::QUEEN => {
                let mut positions = Vec::<Piece>::new();
                positions.append(&mut self.get_legal_moves(construct_piece(
                    p.x,
                    p.y,
                    PieceType::BISHOP,
                    p.color,
                )));
                positions.append(&mut self.get_legal_moves(construct_piece(
                    p.x,
                    p.y,
                    PieceType::ROOK,
                    p.color,
                )));
                for pos in positions {
                    moves.push(construct_piece(pos.x, pos.y, PieceType::QUEEN, p.color));
                }
            }
            PieceType::PAWN => {
                // TODO: en passant
                // TODO: promotions
                // taking
                match p.color {
                    PlayerColor::WHITE => {
                        match self.get_piece_at_square(Vec2::new(p.x - 1, p.y + 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(p.x - 1, p.y + 1, PieceType::PAWN, p.color));
                                }
                            }
                        }
                    }
                    PlayerColor::BLACK => {
                        match self.get_piece_at_square(Vec2::new(p.x - 1, p.y - 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(p.x - 1, p.y - 1, PieceType::PAWN, p.color));
                                }
                            }
                        }
                    }
                }
                match p.color {
                    PlayerColor::WHITE => {
                        match self.get_piece_at_square(Vec2::new(p.x + 1, p.y + 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(p.x + 1, p.y + 1, PieceType::PAWN, p.color));
                                }
                            }
                        }
                    }
                    PlayerColor::BLACK => {
                        match self.get_piece_at_square(Vec2::new(p.x + 1, p.y - 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(p.x + 1, p.y - 1, PieceType::PAWN, p.color));
                                }
                            }
                        }
                    }
                }
                // normal move
                match p.color {
                    PlayerColor::WHITE => {
                        match self.get_piece_at_square(Vec2::new(p.x, p.y + 1)) {
                            None => {
                                moves.push(construct_piece(p.x, p.y + 1, p.piece_type, p.color));
                                // double step
                                if p.y == 1 {
                                    match self.get_piece_at_square(Vec2::new(p.x, p.y + 2)) {
                                        Some(_) => {}
                                        None => moves.push(construct_piece(
                                            p.x,
                                            p.y + 2,
                                            PieceType::PAWN,
                                            p.color,
                                        )),
                                    }
                                }
                            }
                            Some(_) => {}
                        }
                    }
                    PlayerColor::BLACK => match self.get_piece_at_square(Vec2::new(p.x, p.y - 1)) {
                        None => {
                            moves.push(construct_piece(p.x, p.y - 1, p.piece_type, p.color));
                            // double step
                            if p.y == 6 {
                                match self.get_piece_at_square(Vec2::new(p.x, p.y - 2)) {
                                    Some(_) => {}
                                    None => {
                                        moves.push(construct_piece(p.x, p.y - 2, PieceType::PAWN, p.color))
                                    }
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
