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
    pub fn get_board(&mut self) -> BoardState {
        self.history.board_states.last_mut().unwrap().clone()
    }
    fn isnt_check(&mut self, p: Piece) -> bool {
        // TODO: replace with BoardState::is_check()
        // NOTE: can be done simpler
        // FIXME: cloning here is stupid
        let pcs = self.current_pieces().clone();
        // TODO: use 2D array to precompute attacked squares
        for piece in pcs {
            if p.color == piece.color {
                continue;
            }
            if piece.piece_type == PieceType::KING {
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
                    if is_within_chessboard(mv) && mv.x == p.x && mv.y == p.y {
                        return false;
                    }
                }
            } else if piece.piece_type == PieceType::PAWN {
                let color_mult = match piece.color {
                    PlayerColor::WHITE => 1,
                    PlayerColor::BLACK => -1,
                };
                // TODO: optimize
                if (is_within_chessboard(Vec2::new(piece.x + 1, piece.y + color_mult))
                    && piece.x + 1 == p.x
                    && piece.y + color_mult == p.y)
                    || (is_within_chessboard(Vec2::new(piece.x - 1, piece.y + color_mult))
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
    // TODO: use get_piece_at_square() like add_move_if_legal() does

    pub fn get_legal_moves(&mut self, p: Piece) -> Vec<Piece> {
        // FIXME: detect illegal positions, including ignored checks, pawns on first ranks, castles
        let mut moves = Vec::<Piece>::new();
        let board = self.get_board();
        match p.piece_type {
            PieceType::BISHOP => {
                let add_move_if_legal =
                    |pos: Vec2<i8>, moves: &mut Vec<Piece>, c: &mut GameContainer| {
                        let check = c.get_board().is_move_plausible(pos);
                        if check == MovePlausibility::IMPOSSIBLE {
                            return false;
                        }
                        moves.push(construct_piece(pos.x, pos.y, PieceType::BISHOP, p.color));
                        if check == MovePlausibility::TAKES {
                            return false;
                        }
                        true
                    };
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x + i, p.y + i);
                    if !add_move_if_legal(pos, &mut moves, self) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x + i, p.y - i);
                    if !add_move_if_legal(pos, &mut moves, self) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x - i, p.y + i);
                    if !add_move_if_legal(pos, &mut moves, self) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x - i, p.y - i);
                    if !add_move_if_legal(pos, &mut moves, self) {
                        break;
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
                    if !is_within_chessboard(pos) {
                        continue;
                    }
                    let piece_there = board.get_piece_at_square(pos);
                    if piece_there.is_some() && piece_there.unwrap().color == p.color {
                        continue;
                    }
                    moves.push(construct_piece(pos.x, pos.y, PieceType::KNIGHT, p.color));
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
                    if board.is_move_plausible(Vec2::new(temp.x, temp.y))
                        != MovePlausibility::IMPOSSIBLE
                        && board.is_check() // NOTE: check check after this move?
                    {
                        moves.push(temp);
                    }
                }
                if board.is_check() {
                    return moves;
                }
                let y_pos = match p.color {
                    PlayerColor::WHITE => 0,
                    PlayerColor::BLACK => 7,
                };
                if board.can_queen_side_castle(p.color) {
                    moves.push(construct_piece(1, y_pos, PieceType::KING, p.color));
                    moves.push(construct_piece(2, y_pos, PieceType::ROOK, p.color));
                }
                if board.can_king_side_castle(p.color) {
                    moves.push(construct_piece(6, y_pos, PieceType::KING, p.color));
                    moves.push(construct_piece(5, y_pos, PieceType::ROOK, p.color));
                }

                // FIXME: what if it is check??
            }
            PieceType::ROOK => {
                let add_move_if_legal = |board: &BoardState,
                                         pos: Vec2<i8>,
                                         moves: &mut Vec<Piece>,
                                         p: Piece|
                 -> MovePlausibility {
                    if !is_within_chessboard(pos) {
                        return MovePlausibility::IMPOSSIBLE;
                    }
                    let piece_there = board.get_piece_at_square(pos);
                    if piece_there.is_some() && piece_there.unwrap().color == p.color {
                        return MovePlausibility::IMPOSSIBLE;
                    }
                    moves.push(construct_piece(pos.x, pos.y, PieceType::ROOK, p.color));
                    if piece_there.is_some() {
                        return MovePlausibility::TAKES;
                    }
                    MovePlausibility::MOVE
                };
                for i in 1..8 {
                    let pos = Vec2::new(p.x, p.y + i);
                    let move_plausibility = add_move_if_legal(&board, pos, &mut moves, p);
                    if move_plausibility == MovePlausibility::IMPOSSIBLE
                        || move_plausibility == MovePlausibility::TAKES
                    {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x, p.y - i);
                    let move_plausibility = add_move_if_legal(&board, pos, &mut moves, p);
                    if move_plausibility == MovePlausibility::IMPOSSIBLE
                        || move_plausibility == MovePlausibility::TAKES
                    {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x + i, p.y);
                    let move_plausibility = add_move_if_legal(&board, pos, &mut moves, p);
                    if move_plausibility == MovePlausibility::IMPOSSIBLE
                        || move_plausibility == MovePlausibility::TAKES
                    {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x - i, p.y);
                    let move_plausibility = add_move_if_legal(&board, pos, &mut moves, p);
                    if move_plausibility == MovePlausibility::IMPOSSIBLE
                        || move_plausibility == MovePlausibility::TAKES
                    {
                        break;
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
                        match board.get_piece_at_square(Vec2::new(p.x - 1, p.y + 1)) {
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
                        match board.get_piece_at_square(Vec2::new(p.x - 1, p.y - 1)) {
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
                        match board.get_piece_at_square(Vec2::new(p.x + 1, p.y + 1)) {
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
                        match board.get_piece_at_square(Vec2::new(p.x + 1, p.y - 1)) {
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
                        match board.get_piece_at_square(Vec2::new(p.x, p.y + 1)) {
                            None => {
                                moves.push(construct_piece(p.x, p.y + 1, p.piece_type, p.color));
                                // double step
                                if p.y == 1 {
                                    match board.get_piece_at_square(Vec2::new(p.x, p.y + 2)) {
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
                    PlayerColor::BLACK => {
                        match board.get_piece_at_square(Vec2::new(p.x, p.y - 1)) {
                            None => {
                                moves.push(construct_piece(p.x, p.y - 1, p.piece_type, p.color));
                                // double step
                                if p.y == 6 {
                                    match board.get_piece_at_square(Vec2::new(p.x, p.y - 2)) {
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
                        }
                    }
                }
            }
        }
        moves
    }
}
