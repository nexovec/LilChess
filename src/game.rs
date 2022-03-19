use crate::game_types::*;
use tetra::math::Vec2;

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
            if piece.x == p.x && piece.y == p.y {
                // TODO: get rid of GameContainer.initial_p_to_move
                if piece.color == self.history.board_states.last().unwrap().player_to_move {
                    return false;
                }
            }
        }
        true
    }
    pub fn get_legal_moves(&mut self, p: Piece) -> Vec<Piece> {
        // FIXME: detect illegal positions, including ignored checks, pawns on first ranks, castles
        let mut moves = Vec::<Piece>::new();
        match p.piece_type {
            PieceType::BISHOP => {
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x + i, p.y + i);
                    if !self.check_move(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(
                                        pos.x,
                                        pos.y,
                                        PieceType::BISHOP,
                                        p.color,
                                    ));
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
                                    moves.push(construct_piece(
                                        pos.x,
                                        pos.y,
                                        PieceType::BISHOP,
                                        p.color,
                                    ));
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
                                    moves.push(construct_piece(
                                        pos.x,
                                        pos.y,
                                        PieceType::BISHOP,
                                        p.color,
                                    ));
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
                                    moves.push(construct_piece(
                                        pos.x,
                                        pos.y,
                                        PieceType::BISHOP,
                                        p.color,
                                    ));
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
                fn can_queen_side_castle(
                    container: &mut GameContainer,
                    player_color: PlayerColor,
                ) -> bool {
                    // TODO: test
                    let mut y = 0;
                    if player_color == PlayerColor::BLACK {
                        y = 7;
                    }

                    if player_color == PlayerColor::WHITE
                        && container
                            .history
                            .board_states
                            .last()
                            .unwrap()
                            .white_can_castle_q
                            == false
                    {
                        return false;
                    }
                    if player_color == PlayerColor::BLACK
                        && container
                            .history
                            .board_states
                            .last()
                            .unwrap()
                            .black_can_castle_q
                            == false
                    {
                        return false;
                    }

                    // TODO: check for attacked squares.
                    if container
                        .history
                        .board_states
                        .last()
                        .unwrap()
                        .white_can_castle_q
                        && container.get_piece_at_square(Vec2::new(4, y)).is_some()
                        && container
                            .get_piece_at_square(Vec2::new(4, y))
                            .unwrap()
                            .piece_type
                            == PieceType::KING
                        && container.get_piece_at_square(Vec2::new(0, y)).is_some()
                        && container
                            .get_piece_at_square(Vec2::new(0, y))
                            .unwrap()
                            .piece_type
                            == PieceType::ROOK
                        && container.get_piece_at_square(Vec2::new(1, y)).is_none()
                        && container.get_piece_at_square(Vec2::new(2, y)).is_none()
                        && container.isnt_check(construct_piece(
                            0,
                            y,
                            PieceType::KING,
                            player_color,
                        ))
                        && container.isnt_check(construct_piece(
                            3,
                            0,
                            PieceType::ROOK,
                            player_color,
                        ))
                    {
                        return true;
                    }
                    false
                }

                fn can_king_side_castle(
                    container: &mut GameContainer,
                    player_color: PlayerColor,
                ) -> bool {
                    // TODO: test
                    let mut y = 0;
                    if player_color == PlayerColor::BLACK {
                        y = 7;
                    }

                    if player_color == PlayerColor::WHITE
                        && container
                            .history
                            .board_states
                            .last()
                            .unwrap()
                            .white_can_castle_k
                            == false
                    {
                        return false;
                    }
                    if player_color == PlayerColor::BLACK
                        && container
                            .history
                            .board_states
                            .last()
                            .unwrap()
                            .black_can_castle_k
                            == false
                    {
                        return false;
                    }

                    // TODO: check for attacked squares.
                    if container
                        .history
                        .board_states
                        .last()
                        .unwrap()
                        .white_can_castle_q
                        && container.get_piece_at_square(Vec2::new(4, y)).is_some()
                        && container
                            .get_piece_at_square(Vec2::new(4, y))
                            .unwrap()
                            .piece_type
                            == PieceType::KING
                        && container.get_piece_at_square(Vec2::new(7, y)).is_some()
                        && container
                            .get_piece_at_square(Vec2::new(7, y))
                            .unwrap()
                            .piece_type
                            == PieceType::ROOK
                        && container.get_piece_at_square(Vec2::new(5, y)).is_none()
                        && container.get_piece_at_square(Vec2::new(6, y)).is_none()
                        && container.isnt_check(construct_piece(
                            7,
                            y,
                            PieceType::KING,
                            player_color,
                        ))
                        && container.isnt_check(construct_piece(
                            4,
                            0,
                            PieceType::ROOK,
                            player_color,
                        ))
                    {
                        return true;
                    }
                    false
                }
                if self.isnt_check(p) {
                    match p.color {
                        PlayerColor::WHITE => {
                            // FIXME: DRY

                            // queen side castle
                            if can_queen_side_castle(self, p.color) {
                                moves.push(construct_piece(2, 0, PieceType::KING, p.color));
                            }
                            // king side castle
                            if can_king_side_castle(self, p.color) {
                                moves.push(construct_piece(6, 0, PieceType::KING, p.color));
                            }
                        }
                        PlayerColor::BLACK => {
                            if can_queen_side_castle(self, p.color) {
                                moves.push(construct_piece(2, 7, PieceType::KING, p.color));
                            }
                            if can_king_side_castle(self, p.color) {
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
                                    moves.push(construct_piece(
                                        pos.x,
                                        pos.y,
                                        PieceType::ROOK,
                                        p.color,
                                    ));
                                }
                                break;
                            }
                            None => {
                                moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color))
                            }
                        }
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x - i, p.y);
                    if self.check_boundaries(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(piece) => {
                                if piece.color != p.color {
                                    moves.push(construct_piece(
                                        pos.x,
                                        pos.y,
                                        PieceType::ROOK,
                                        p.color,
                                    ));
                                }
                                break;
                            }
                            None => {
                                moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color))
                            }
                        }
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x, p.y + i);
                    if self.check_boundaries(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(piece) => {
                                if piece.color != p.color {
                                    moves.push(construct_piece(
                                        pos.x,
                                        pos.y,
                                        PieceType::ROOK,
                                        p.color,
                                    ));
                                }
                                break;
                            }
                            None => {
                                moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color))
                            }
                        }
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x, p.y - i);
                    if self.check_boundaries(pos) {
                        match self.get_piece_at_square(pos) {
                            Some(piece) => {
                                if piece.color != p.color {
                                    moves.push(construct_piece(
                                        pos.x,
                                        pos.y,
                                        PieceType::ROOK,
                                        p.color,
                                    ));
                                }
                                break;
                            }
                            None => {
                                moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color))
                            }
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
                                    moves.push(construct_piece(
                                        p.x - 1,
                                        p.y + 1,
                                        PieceType::PAWN,
                                        p.color,
                                    ));
                                }
                            }
                        }
                    }
                    PlayerColor::BLACK => {
                        match self.get_piece_at_square(Vec2::new(p.x - 1, p.y - 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(
                                        p.x - 1,
                                        p.y - 1,
                                        PieceType::PAWN,
                                        p.color,
                                    ));
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
                                    moves.push(construct_piece(
                                        p.x + 1,
                                        p.y + 1,
                                        PieceType::PAWN,
                                        p.color,
                                    ));
                                }
                            }
                        }
                    }
                    PlayerColor::BLACK => {
                        match self.get_piece_at_square(Vec2::new(p.x + 1, p.y - 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != p.color {
                                    moves.push(construct_piece(
                                        p.x + 1,
                                        p.y - 1,
                                        PieceType::PAWN,
                                        p.color,
                                    ));
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
                                    None => moves.push(construct_piece(
                                        p.x,
                                        p.y - 2,
                                        PieceType::PAWN,
                                        p.color,
                                    )),
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
