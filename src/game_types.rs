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
    pub castling_rules: CastlingRules,
    pub can_castle_k_this_move: bool,
    pub can_castle_q_this_move: bool,
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
#[derive(Clone)]
pub struct CastlingRules {
    pub white_can_still_castle_q: bool,
    pub white_can_still_castle_k: bool,
    pub black_can_still_castle_q: bool,
    pub black_can_still_castle_k: bool,
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
impl CastlingRules {
    pub fn new(
        white_can_still_castle_q: bool,
        white_can_still_castle_k: bool,
        black_can_still_castle_q: bool,
        black_can_still_castle_k: bool,
    ) -> CastlingRules {
        CastlingRules {
            white_can_still_castle_q: white_can_still_castle_q,
            white_can_still_castle_k: white_can_still_castle_k,
            black_can_still_castle_q: black_can_still_castle_q,
            black_can_still_castle_k: black_can_still_castle_k,
        }
    }
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
    pub fn get_board(&mut self) -> &BoardState {
        self.board_states.last().unwrap()
    }
    /**
     * Assumes the move is already checked
     */
    pub fn on_piece_taken(&mut self) -> () {
        println!("I've taken a piece");
    }
    pub fn execute_move(&mut self, mv: ChessMove) -> bool {
        // TODO: why is this returning a bool wtf
        self.moves.push(mv.clone());
        let board = self.get_board().to_owned();
        self.board_states.push(board.after_move(mv));
        true
    }
}

impl BoardState {
    pub fn new(
        pieces: Vec<Piece>,
        player_to_move: PlayerColor,
        castling_rules: CastlingRules,
    ) -> BoardState {
        let mut state = BoardState {
            pieces: pieces,
            player_to_move: player_to_move,
            castling_rules: castling_rules,
            can_castle_k_this_move: false,
            can_castle_q_this_move: false,
        };
        state.can_castle_k_this_move = state.evaluate_can_king_side_castle(state.player_to_move);
        state.can_castle_q_this_move = state.evaluate_can_queen_side_castle(state.player_to_move);
        state
    }
    /** This is an unsafe funtion, validate the moves yourself! */
    pub fn after_move(&self, mv: ChessMove) -> BoardState {
        // FIXME: castles disable castles for the other player
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
        let did_q_castle: bool = mv.is_queen_side_castles();
        let did_k_castle: bool = mv.is_king_side_castles();

        let position_after_move = BoardState::new(
            pieces,
            PlayerColor::opposite(self.player_to_move),
            CastlingRules::new(
                self.castling_rules.white_can_still_castle_q
                    && (!did_q_castle || self.player_to_move != PlayerColor::WHITE),
                self.castling_rules.white_can_still_castle_k
                    && (!did_k_castle || self.player_to_move != PlayerColor::WHITE),
                self.castling_rules.black_can_still_castle_q
                    && (!did_q_castle || self.player_to_move != PlayerColor::BLACK),
                self.castling_rules.black_can_still_castle_k
                    && (!did_k_castle || self.player_to_move != PlayerColor::BLACK),
            ),
        );
        if did_k_castle {
            let y = match self.player_to_move {
                PlayerColor::WHITE => 0,
                PlayerColor::BLACK => 7,
            };
            let piece_from = Piece::new(7, y, PieceType::ROOK, self.player_to_move);
            let piece_to = Piece::new(5, y, PieceType::ROOK, self.player_to_move);
            let mut rook_moved =
                position_after_move.after_move(ChessMove::new(piece_from, piece_to));
            rook_moved.player_to_move = PlayerColor::opposite(self.player_to_move);
            return rook_moved;
        }
        if did_q_castle {
            let y = match self.player_to_move {
                PlayerColor::WHITE => 0,
                PlayerColor::BLACK => 7,
            };
            let piece_from = Piece::new(0, y, PieceType::ROOK, self.player_to_move);
            let piece_to = Piece::new(3, y, PieceType::ROOK, self.player_to_move);
            let mut rook_moved =
                position_after_move.after_move(ChessMove::new(piece_from, piece_to));
            rook_moved.player_to_move = PlayerColor::opposite(self.player_to_move);
            return rook_moved;
        }
        position_after_move
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
        if player_color == PlayerColor::WHITE
            && self.castling_rules.white_can_still_castle_k == false
        {
            return false;
        }
        if player_color == PlayerColor::BLACK
        // TODO: replace with history.now() or GameContainer::position()
            && self.castling_rules.black_can_still_castle_k
                == false
        {
            return false;
        }
        // TODO: check for attacked squares.
        let piece_at_4 = self.get_piece_at_square(Vec2::new(4, y));
        let piece_at_7 = self.get_piece_at_square(Vec2::new(7, y));
        if piece_at_4.is_some()
            && piece_at_4.unwrap().piece_type == PieceType::KING
            && piece_at_7.is_some()
            && piece_at_7.unwrap().piece_type == PieceType::ROOK
            && self.get_piece_at_square(Vec2::new(5, y)).is_none()
            && self.get_piece_at_square(Vec2::new(6, y)).is_none()
        {
            return true;
        }
        false
    }
    fn evaluate_can_queen_side_castle(&self, player_color: PlayerColor) -> bool {
        // TODO: test
        let y = match player_color {
            PlayerColor::WHITE => 0,
            PlayerColor::BLACK => 7,
        };

        if player_color == PlayerColor::WHITE
            && self.castling_rules.white_can_still_castle_q == false
        {
            return false;
        }
        if player_color == PlayerColor::BLACK
        // TODO: replace with history.now() or GameContainer::position()
            && self.castling_rules.black_can_still_castle_q
                == false
        {
            return false;
        }

        // TODO: check for attacked squares.
        let piece_there_ey = self.get_piece_at_square(Vec2::new(4, y));
        let piece_there_ay = self.get_piece_at_square(Vec2::new(0, y));
        if piece_there_ey.is_some()
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
            .find(|thing| thing.piece_type == PieceType::KING && thing.color == color.unwrap())
        {
            let king_pos = king.pos();
            for mv in moves {
                if mv.to.pos() == king_pos && mv.to.color == PlayerColor::opposite(color.unwrap()) {
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
        let mut moves = Vec::<ChessMove>::new();
        let mut in_piece: Piece = p.clone();
        match in_piece.piece_type {
            PieceType::BISHOP => {
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x + i, p.y + i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &p) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x + i, p.y - i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &p) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x - i, p.y + i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &p) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::<i8>::new(p.x - i, p.y - i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &p) {
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
                // TODO: castling
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
                    {
                        moves.push(ChessMove::new(in_piece, piece_to));
                    }
                }
                // TODO: intercept castling execution
                if self.player_to_move == PlayerColor::BLACK {
                    if self.can_castle_k_this_move {
                        moves.push(ChessMove::new(
                            p.clone(),
                            Piece::new(6, 7, p.piece_type, p.color),
                        ));
                    }
                    if self.can_castle_q_this_move {
                        moves.push(ChessMove::new(
                            p.clone(),
                            Piece::new(2, 7, p.piece_type, p.color),
                        ));
                    }
                }
                if self.player_to_move == PlayerColor::WHITE {
                    if self.can_castle_k_this_move {
                        moves.push(ChessMove::new(
                            p.clone(),
                            Piece::new(6, 0, p.piece_type, p.color),
                        ));
                    }
                    if self.can_castle_q_this_move {
                        moves.push(ChessMove::new(
                            p.clone(),
                            Piece::new(2, 0, p.piece_type, p.color),
                        ));
                    }
                }
            }
            PieceType::ROOK => {
                for i in 1..8 {
                    let pos = Vec2::new(p.x, p.y + i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &p) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x, p.y - i);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &p) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x + i, p.y);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &p) {
                        break;
                    }
                }
                for i in 1..8 {
                    let pos = Vec2::new(p.x - i, p.y);
                    in_piece.x = pos.x;
                    in_piece.y = pos.y;
                    if !self.add_move_if_legal(&in_piece, &mut moves, &p) {
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
            if piece.color == self.player_to_move {
                moves.append(&mut self.get_plausible_moves(&piece));
            }
        }
        moves
    }
    pub fn get_legal_moves(&self, p: &Piece) -> Vec<ChessMove> {
        // TODO:
        let plausible_moves: Vec<ChessMove> = self.get_plausible_moves(p);
        let legal_moves: Vec<ChessMove> = plausible_moves
            .into_iter()
            .filter(|mv: &ChessMove| -> bool {
                !self.after_move(*mv).is_check(Some(self.player_to_move))
            })
            .collect();
        legal_moves
    }
    pub fn get_all_legal_moves(&self) -> Vec<ChessMove> {
        let mut legal_moves = Vec::<ChessMove>::new();
        for p in self.pieces.iter() {
            if p.color != self.player_to_move {
                continue;
            }
            legal_moves.append(&mut self.get_legal_moves(&p));
        }
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
    pub fn is_king_side_castles(&self) -> bool {
        if self.from.piece_type != PieceType::KING {
            return false;
        }
        if self.from.x != 4 {
            return false;
        }
        if self.from.y != 0 && self.from.y != 7 {
            return false;
        }
        if self.to.y != 0 && self.to.y != 7 {
            return false;
        }
        if self.to.x != 6 {
            return false;
        }
        true
    }
    pub fn is_queen_side_castles(&self) -> bool {
        if self.from.piece_type != PieceType::KING {
            return false;
        }
        if self.from.x != 4 {
            return false;
        }
        if self.from.y != 0 && self.from.y != 7 {
            return false;
        }
        if self.to.y != 0 && self.to.y != 7 {
            return false;
        }
        if self.to.x != 2 {
            return false;
        }
        true
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
