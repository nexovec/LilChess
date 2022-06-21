use crate::game_types::*;
use std::sync::mpsc;
use std::thread;
pub struct Engine {
    sender: mpsc::Sender<ChessMove>,
    receiver: mpsc::Receiver<ChessMove>,
    computing_thread_handle: Option<thread::JoinHandle<()>>,
}
// #[derive(Clone)]
// pub struct PositionEvaluationResult {
//     scored_mv: ChessMove,
//     score: f32,
// }
impl Engine {
    pub fn new() -> Engine {
        let (sx, rx) = mpsc::channel();
        Engine {
            sender: sx,
            receiver: rx,
            computing_thread_handle: None,
        }
    }
    fn static_position_evaluation(board: &BoardState, mv: &ChessMove) -> f32 {
        let mut result = 0.0f32;
        // TODO: Precompute MoveDescription per move
        if board.evaluate_is_check(None) {
            result += board.score_for_current_player(-1.0f32);
        }
        if board.evaluate_is_checkmate() {
            result -= board.score_for_current_player(-1000.0f32);
        }
        for piece in board.get_pieces_vec().iter() {
            match piece.color {
                PlayerColor::WHITE => result += Engine::get_piece_worth(piece),
                PlayerColor::BLACK => result -= Engine::get_piece_worth(piece),
            }
        }
        result
    }
    fn get_piece_worth(p: &Piece) -> f32 {
        let result = match p.piece_type {
            PieceType::BISHOP => 3.2,
            PieceType::KNIGHT => 3.0,
            PieceType::ROOK => 5.0,
            PieceType::QUEEN => 9.0,
            PieceType::PAWN => 1.0,
            PieceType::KING => 0.0,
        };
        result
    }
    fn compute_position_score(board_state: &BoardState, mv: &ChessMove, depth: u32) -> f32 {
        // TODO: Pick the maximum if white to move, minimum if black to move
        // TODO: Get rid of Engine::compute_move()
        // TODO: Track checks past depth
        // TODO: Track capture sequences past depth
        // TODO: Track mate threats past depth
        // TODO: Track capture threats past depth
        let moves = board_state.get_all_legal_moves();
        if moves.len() == 0 {
            return Engine::static_position_evaluation(board_state, mv);
        }
        if depth == 1 {
            let best_move = moves
                .iter()
                .min_by(|mv1, mv2| {
                    let static_position_score_1 =
                        Engine::static_position_evaluation(&board_state.after_move(*mv1), mv1);
                    let static_position_score_2 =
                        Engine::static_position_evaluation(&board_state.after_move(*mv2), mv2);
                    static_position_score_1
                        .partial_cmp(&static_position_score_2)
                        .unwrap()
                })
                .unwrap();
            return Engine::static_position_evaluation(
                &board_state.after_move(best_move),
                best_move,
            );
        }
        let subnode_scores = moves
            .iter()
            .map(|mv| {
                let new_position = board_state.after_move(mv);
                Engine::compute_position_score(&new_position, mv, depth - 1)
            })
            .collect::<Vec<_>>();
        if board_state.player_to_move == PlayerColor::WHITE {
            return *subnode_scores
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
        } else {
            return *subnode_scores
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
        }
    }
    pub fn maybe_calculate_move(&mut self, board_state: BoardState) -> Option<ChessMove> {
        if self.computing_thread_handle.is_none() {
            let tx = self.sender.clone();
            self.computing_thread_handle = Some(thread::spawn(move || {
                // thread::sleep(time::Duration::from_millis(1500));
                let moves = board_state.get_all_legal_moves();
                let depth = 2;
                // let best_move = moves.last().unwrap();
                let best_move = moves
                    .iter()
                    .min_by(|mv1, mv2| {
                        let score1 = Engine::compute_position_score(
                            &board_state.after_move(*mv1),
                            mv1,
                            depth,
                        );
                        let score2 = Engine::compute_position_score(
                            &board_state.after_move(*mv2),
                            mv2,
                            depth,
                        );
                        score1.partial_cmp(&score2).unwrap()
                    })
                    .unwrap()
                    .to_owned();
                let _ = tx.send(best_move);
                drop(tx);
            }));
        }
        let maybe_result = self.receiver.try_recv();
        if maybe_result.is_ok() {
            self.computing_thread_handle = None;
            return Some(maybe_result.unwrap());
        }
        None
    }
}
