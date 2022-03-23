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
    pub fn get_board(&mut self) -> BoardState {
        self.history.board_states.last_mut().unwrap().clone()
    }
    fn get_all_legal_moves(&mut self) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::new();
        for piece in self.get_board().pieces {
            moves.append(&mut self.get_legal_moves(piece));
        }
        moves
    }
    // TODO: use get_piece_at_square() like add_move_if_legal() does

    pub fn get_legal_moves(&mut self, p: Piece) -> Vec<ChessMove> {
        // TODO: return Vec<ChessMove> instead
        // FIXME: detect illegal positions, including ignored checks, pawns on first ranks, castles
        let mut moves = Vec::<ChessMove>::new();
        let board = self.get_board();
        match p.piece_type {
            PieceType::BISHOP => {
                let add_move_if_legal =
                    |pos: Vec2<i8>, moves: &mut Vec<ChessMove>, c: &mut GameContainer| {
                        let move_plausibility = c.get_board().get_move_position_plausibility(pos);
                        if move_plausibility == MovePlausibility::IMPOSSIBLE {
                            return false;
                        }
                        let piece_to = construct_piece(pos.x, pos.y, PieceType::BISHOP, p.color);
                        moves.push(ChessMove::new(p, piece_to));
                        if move_plausibility == MovePlausibility::TAKES {
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
                    let piece_to = construct_piece(pos.x, pos.y, PieceType::KNIGHT, p.color);
                    moves.push(ChessMove::new(p, piece_to));
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
                    let piece_to = construct_piece(pos.x, pos.y, PieceType::KING, p.color);
                    if board.get_move_position_plausibility(Vec2::new(piece_to.x, piece_to.y))
                        != MovePlausibility::IMPOSSIBLE
                    // NOTE: check check after this move?
                    {
                        moves.push(ChessMove::new(p, piece_to));
                    }
                }
            }
            PieceType::ROOK => {
                let add_move_if_legal = |board: &BoardState,
                                         pos: Vec2<i8>,
                                         moves: &mut Vec<ChessMove>,
                                         p: Piece|
                 -> MovePlausibility {
                    if !is_within_chessboard(pos) {
                        return MovePlausibility::IMPOSSIBLE;
                    }
                    let piece_there = board.get_piece_at_square(pos);
                    if piece_there.is_some() && piece_there.unwrap().color == p.color {
                        return MovePlausibility::IMPOSSIBLE;
                    }
                    let piece_to = construct_piece(pos.x, pos.y, PieceType::ROOK, p.color);
                    moves.push(ChessMove::new(p, piece_to));
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
                let bishop_moves =
                    self.get_legal_moves(construct_piece(p.x, p.y, PieceType::BISHOP, p.color));
                let convert_move = |b_move: &ChessMove| {
                    let piece_from = p;
                    let piece_to = construct_piece(
                        b_move.to.x,
                        b_move.to.y,
                        PieceType::QUEEN,
                        b_move.to.color,
                    );
                    ChessMove::new(piece_from, piece_to)
                };
                let queen_diagonal_moves = bishop_moves.iter().map(convert_move);
                let rook_moves =
                    self.get_legal_moves(construct_piece(p.x, p.y, PieceType::ROOK, p.color));
                let queen_straight_moves = rook_moves.iter().map(convert_move);
                moves.append(&mut queen_diagonal_moves.collect());
                moves.append(&mut queen_straight_moves.collect());
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
                                    let piece_to =
                                        construct_piece(p.x - 1, p.y + 1, PieceType::PAWN, p.color);
                                    moves.push(ChessMove::new(p, piece_to));
                                }
                            }
                        }
                    }
                    PlayerColor::BLACK => {
                        match board.get_piece_at_square(Vec2::new(p.x - 1, p.y - 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != p.color {
                                    let piece_to =
                                        construct_piece(p.x - 1, p.y - 1, PieceType::PAWN, p.color);
                                    moves.push(ChessMove::new(p, piece_to));
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
                                    let piece_to =
                                        construct_piece(p.x + 1, p.y + 1, PieceType::PAWN, p.color);
                                    moves.push(ChessMove::new(p, piece_to));
                                }
                            }
                        }
                    }
                    PlayerColor::BLACK => {
                        match board.get_piece_at_square(Vec2::new(p.x + 1, p.y - 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != p.color {
                                    let piece_to =
                                        construct_piece(p.x + 1, p.y - 1, PieceType::PAWN, p.color);
                                    moves.push(ChessMove::new(p, piece_to));
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
                                let piece_to = construct_piece(p.x, p.y + 1, p.piece_type, p.color);
                                moves.push(ChessMove::new(p, piece_to));
                                // double step
                                if p.y == 1 {
                                    match board.get_piece_at_square(Vec2::new(p.x, p.y + 2)) {
                                        Some(_) => {}
                                        None => {
                                            let piece_to = construct_piece(
                                                p.x,
                                                p.y + 2,
                                                PieceType::PAWN,
                                                p.color,
                                            );
                                            moves.push(ChessMove::new(p, piece_to));
                                        }
                                    }
                                }
                            }
                            Some(_) => {}
                        }
                    }
                    PlayerColor::BLACK => {
                        match board.get_piece_at_square(Vec2::new(p.x, p.y - 1)) {
                            None => {
                                let piece_to = construct_piece(p.x, p.y - 1, p.piece_type, p.color);
                                moves.push(ChessMove::new(p, piece_to));
                                // double step
                                if p.y == 6 {
                                    match board.get_piece_at_square(Vec2::new(p.x, p.y - 2)) {
                                        Some(_) => {}
                                        None => {
                                            let piece_to = construct_piece(
                                                p.x,
                                                p.y - 2,
                                                PieceType::PAWN,
                                                p.color,
                                            );
                                            moves.push(ChessMove::new(p, piece_to));
                                        }
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
