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
#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub x: i8,
    pub y: i8,
    pub piece_type: PieceType,
    pub color: PlayerColor,
}
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
impl GameHistory {
    pub fn new(board_states: Vec<BoardState>, moves: Option<Vec<ChessMove>>) -> GameHistory {
        // let board_states = vec![BoardState::default_board()?];
        let board_states = board_states;
        let moves_unwrapped = match moves {
            Some(thing) => thing,
            None => Vec::<ChessMove>::new(),
        };
        GameHistory {
            board_states: board_states,
            moves: moves_unwrapped,
        }
    }
    // FIXME: duplicate of GameContainer::get_board()
    pub fn get_board(&mut self) -> &BoardState {
        self.board_states.last().unwrap()
    }
    /**
     * Assumes the move is already checked
     */
    pub fn on_piece_taken(&mut self) -> () {
        // TODO: print something nice to the screen
        println!("I've taken a piece");
    }
    // TODO: sync BoardState.color with GameHistory.initial_p_to_move
    pub fn execute_move(&mut self, mv: ChessMove) -> bool {
        self.moves.push(mv);
        let mut new_state = self.get_board().clone();
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
        new_state.player_to_move = PlayerColor::opposite(new_state.player_to_move);

        self.board_states.push(new_state);
        true
    }
}
impl BoardState {
    pub fn new(pieces: Vec<Piece>, player_to_move: PlayerColor) -> BoardState {
        let mut state = BoardState {
            pieces: pieces,
            player_to_move: player_to_move,
            white_can_castle_q: false,
            white_can_castle_k: false,
            black_can_castle_q: false,
            black_can_castle_k: false,
        };
        state.white_can_castle_q =
            BoardState::evaluate_can_queen_side_castle(&state, PlayerColor::WHITE);
        state.black_can_castle_q =
            BoardState::evaluate_can_queen_side_castle(&state, PlayerColor::BLACK);
        state.white_can_castle_k =
            BoardState::evaluate_can_king_side_castle(&state, PlayerColor::WHITE);
        state.black_can_castle_k =
            BoardState::evaluate_can_king_side_castle(&state, PlayerColor::WHITE);
        state
    }
    pub fn after_move(&self, mv: &ChessMove) -> BoardState {
        let mut pieces = Vec::<Piece>::new();
        for i in &self.pieces {
            if mv.to.pos() == i.pos() {
                continue;
            }
            if mv.from.pos() == i.pos() {
                continue;
            }
            pieces.push(i.clone());
        }
        pieces.push(mv.to);
        BoardState::new(pieces, PlayerColor::opposite(self.player_to_move))
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
    fn get_move_position_plausibility(&self, p: Vec2<i8>) -> MovePlausibility {
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
    pub fn is_check(&self, color: Option<PlayerColor>) -> bool {
        // TODO:
        let moves = self.get_all_plausible_moves();
        if color.is_none() {
            return self.is_check(Some(self.player_to_move));
        }
        if let Some(king) = self
            .pieces
            .iter()
            .find(|thing| thing.piece_type == PieceType::KING && thing.color == self.player_to_move)
        {
            let king_pos = king.pos();
            for mv in moves {
                if mv.to.pos() == king_pos
                    && mv.to.color == PlayerColor::opposite(self.player_to_move)
                {
                    return true;
                }
            }
        } else {
            panic!("There is no king!");
        }
        false
    }
    // returns boolean to assist with breaking out of loops where it's used.
    fn add_move_if_legal(
        &self,
        piece_to: &Piece,
        moves: &mut Vec<ChessMove>,
        piece_from: &Piece,
    ) -> bool {
        let move_plausibility =
            self.get_move_position_plausibility(Vec2::<i8>::new(piece_to.x, piece_to.y));
        if move_plausibility == MovePlausibility::IMPOSSIBLE {
            return false;
        }
        moves.push(ChessMove::new(piece_from.clone(), piece_to.clone()));
        if move_plausibility == MovePlausibility::TAKES {
            return false;
        }
        true
    }
    pub fn get_plausible_moves(&self, p: &Piece) -> Vec<ChessMove> {
        // TODO: return Vec<ChessMove> instead
        // FIXME: detect illegal positions, including ignored checks, pawns on first ranks, castles
        let mut moves = Vec::<ChessMove>::new();
        let mut in_piece: Piece = p.clone();
        match in_piece.piece_type {
            PieceType::BISHOP => {
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(in_piece.x + i, in_piece.y + i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &in_piece) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(in_piece.x + i, in_piece.y - i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &in_piece) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(in_piece.x - i, in_piece.y + i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &in_piece) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(in_piece.x - i, in_piece.y - i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &in_piece) {
                        break;
                    }
                }
            }
            PieceType::KNIGHT => {
                let positions = vec![
                    Vec2::new(in_piece.x + 2, in_piece.y + 1),
                    Vec2::new(in_piece.x - 2, in_piece.y + 1),
                    Vec2::new(in_piece.x + 2, in_piece.y - 1),
                    Vec2::new(in_piece.x + 1, in_piece.y + 2),
                    Vec2::new(in_piece.x - 1, in_piece.y + 2),
                    Vec2::new(in_piece.x - 1, in_piece.y - 2),
                    Vec2::new(in_piece.x + 1, in_piece.y - 2),
                    Vec2::new(in_piece.x - 2, in_piece.y - 1),
                ];
                for pos in positions {
                    if !is_within_chessboard(pos) {
                        continue;
                    }
                    let piece_there = self.get_piece_at_square(pos);
                    if piece_there.is_some() && piece_there.unwrap().color == in_piece.color {
                        continue;
                    }
                    let piece_to = Piece::new(pos.x, pos.y, PieceType::KNIGHT, in_piece.color);
                    moves.push(ChessMove::new(in_piece, piece_to));
                }
            }
            PieceType::KING => {
                let positions = vec![
                    Vec2::new(in_piece.x, in_piece.y + 1),
                    Vec2::new(in_piece.x, in_piece.y - 1),
                    Vec2::new(in_piece.x + 1, in_piece.y - 1),
                    Vec2::new(in_piece.x + 1, in_piece.y + 1),
                    Vec2::new(in_piece.x - 1, in_piece.y + 1),
                    Vec2::new(in_piece.x - 1, in_piece.y - 1),
                    Vec2::new(in_piece.x + 1, in_piece.y),
                    Vec2::new(in_piece.x - 1, in_piece.y),
                ];
                for pos in positions {
                    let piece_to = Piece::new(pos.x, pos.y, PieceType::KING, in_piece.color);
                    if self.get_move_position_plausibility(Vec2::new(piece_to.x, piece_to.y))
                        != MovePlausibility::IMPOSSIBLE
                    // NOTE: check check after this move?
                    {
                        moves.push(ChessMove::new(in_piece, piece_to));
                    }
                }
            }
            PieceType::ROOK => {
                for i in 1..8 {
                    let pos = Vec2::new(in_piece.x, in_piece.y + i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &in_piece) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(in_piece.x, in_piece.y - i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &in_piece) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(in_piece.x + i, in_piece.y);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &in_piece) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(in_piece.x - i, in_piece.y);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &in_piece) {
                        break;
                    }
                }
            }
            PieceType::QUEEN => {
                let bishop_moves = self.get_plausible_moves(&Piece::new(
                    in_piece.x,
                    in_piece.y,
                    PieceType::BISHOP,
                    in_piece.color,
                ));
                // TODO: replace with for loop
                let convert_move = |b_move: &ChessMove| {
                    let piece_from = in_piece;
                    let piece_to =
                        Piece::new(b_move.to.x, b_move.to.y, PieceType::QUEEN, b_move.to.color);
                    ChessMove::new(piece_from, piece_to)
                };
                let queen_diagonal_moves = bishop_moves.iter().map(convert_move);
                let rook_moves = self.get_plausible_moves(&Piece::new(
                    in_piece.x,
                    in_piece.y,
                    PieceType::ROOK,
                    in_piece.color,
                ));
                let queen_straight_moves = rook_moves.iter().map(convert_move);
                moves.append(&mut queen_diagonal_moves.collect());
                moves.append(&mut queen_straight_moves.collect());
            }
            PieceType::PAWN => {
                // TODO: en passant
                // TODO: promotions
                // taking
                match in_piece.color {
                    PlayerColor::WHITE => {
                        match self.get_piece_at_square(Vec2::new(in_piece.x - 1, in_piece.y + 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != in_piece.color {
                                    let piece_to = Piece::new(
                                        in_piece.x - 1,
                                        in_piece.y + 1,
                                        PieceType::PAWN,
                                        in_piece.color,
                                    );
                                    moves.push(ChessMove::new(in_piece, piece_to));
                                }
                            }
                        }
                    }
                    PlayerColor::BLACK => {
                        match self.get_piece_at_square(Vec2::new(in_piece.x - 1, in_piece.y - 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != in_piece.color {
                                    let piece_to = Piece::new(
                                        in_piece.x - 1,
                                        in_piece.y - 1,
                                        PieceType::PAWN,
                                        in_piece.color,
                                    );
                                    moves.push(ChessMove::new(in_piece, piece_to));
                                }
                            }
                        }
                    }
                }
                match in_piece.color {
                    PlayerColor::WHITE => {
                        match self.get_piece_at_square(Vec2::new(in_piece.x + 1, in_piece.y + 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != in_piece.color {
                                    let piece_to = Piece::new(
                                        in_piece.x + 1,
                                        in_piece.y + 1,
                                        PieceType::PAWN,
                                        in_piece.color,
                                    );
                                    moves.push(ChessMove::new(in_piece, piece_to));
                                }
                            }
                        }
                    }
                    PlayerColor::BLACK => {
                        match self.get_piece_at_square(Vec2::new(in_piece.x + 1, in_piece.y - 1)) {
                            None => {}
                            Some(i) => {
                                if i.color != in_piece.color {
                                    let piece_to = Piece::new(
                                        in_piece.x + 1,
                                        in_piece.y - 1,
                                        PieceType::PAWN,
                                        in_piece.color,
                                    );
                                    moves.push(ChessMove::new(in_piece, piece_to));
                                }
                            }
                        }
                    }
                }
                // normal move
                match in_piece.color {
                    PlayerColor::WHITE => {
                        match self.get_piece_at_square(Vec2::new(in_piece.x, in_piece.y + 1)) {
                            None => {
                                let piece_to = Piece::new(
                                    in_piece.x,
                                    in_piece.y + 1,
                                    in_piece.piece_type,
                                    in_piece.color,
                                );
                                moves.push(ChessMove::new(in_piece, piece_to));
                                // double step
                                if in_piece.y == 1 {
                                    match self
                                        .get_piece_at_square(Vec2::new(in_piece.x, in_piece.y + 2))
                                    {
                                        Some(_) => {}
                                        None => {
                                            let piece_to = Piece::new(
                                                in_piece.x,
                                                in_piece.y + 2,
                                                PieceType::PAWN,
                                                in_piece.color,
                                            );
                                            moves.push(ChessMove::new(in_piece, piece_to));
                                        }
                                    }
                                }
                            }
                            Some(_) => {}
                        }
                    }
                    PlayerColor::BLACK => {
                        match self.get_piece_at_square(Vec2::new(in_piece.x, in_piece.y - 1)) {
                            None => {
                                let piece_to = Piece::new(
                                    in_piece.x,
                                    in_piece.y - 1,
                                    in_piece.piece_type,
                                    in_piece.color,
                                );
                                moves.push(ChessMove::new(in_piece, piece_to));
                                // double step
                                if in_piece.y == 6 {
                                    match self
                                        .get_piece_at_square(Vec2::new(in_piece.x, in_piece.y - 2))
                                    {
                                        Some(_) => {}
                                        None => {
                                            let piece_to = Piece::new(
                                                in_piece.x,
                                                in_piece.y - 2,
                                                PieceType::PAWN,
                                                in_piece.color,
                                            );
                                            moves.push(ChessMove::new(in_piece, piece_to));
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
    fn get_all_plausible_moves(&self) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::new();
        let pieces = &self.pieces;
        for piece in pieces {
            moves.append(&mut self.get_plausible_moves(&piece));
        }
        moves
    }
    fn get_legal_moves(&self, p: &Piece) -> Vec<ChessMove> {
        // TODO:
        let plausible_moves: Vec<ChessMove> = self.get_plausible_moves(p);
        let legal_moves: Vec<ChessMove> = plausible_moves
            .iter()
            .filter(|mv: &&ChessMove| -> bool {
                self.after_move(*mv).is_check(Some(self.player_to_move))
            })
            .cloned()
            .collect();
        legal_moves
    }
}
impl Piece {
    pub fn new(x: i8, y: i8, piece_type: PieceType, color: PlayerColor) -> Piece {
        Piece {
            x: x,
            y: y,
            piece_type,
            color: color,
        }
    }
    pub fn pos(&self) -> Vec2<i8> {
        Vec2::<i8>::new(self.x, self.y)
    }
}
impl ChessMove {
    pub fn new(from: Piece, to: Piece) -> ChessMove {
        ChessMove { from: from, to: to }
    }
}
impl PlayerColor {
    pub fn opposite(color: PlayerColor) -> PlayerColor {
        if color == PlayerColor::WHITE {
            return PlayerColor::BLACK;
        }
        PlayerColor::WHITE
    }
}
