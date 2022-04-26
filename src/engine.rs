use crate::game_types::*;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time;
pub struct Engine {
    sender: mpsc::Sender<ChessMove>,
    receiver: mpsc::Receiver<ChessMove>,
    computing_thread_handle: Option<thread::JoinHandle<()>>,
}
#[derive(Clone)]
pub struct PositionEvaluationResult {
    scored_mv: ChessMove,
    score: f32,
}
impl Engine {
    pub fn new() -> Engine {
        let (sx, rx) = mpsc::channel();
        Engine {
            sender: sx,
            receiver: rx,
            computing_thread_handle: None,
        }
    }
    fn static_position_evaluation(board: &BoardState) -> f32 {
        1.0f32
    }
    fn compute_position_score(board_state: &BoardState, depth: u32) -> f32 {
        let moves = board_state.get_all_legal_moves();
        if moves.len() == 0 {
            return Engine::static_position_evaluation(board_state);
        }
        if depth == 1 {
            let best_move = moves
                .iter()
                .min_by(|mv1, mv2| {
                    let static_position_score_1 =
                        Engine::static_position_evaluation(&board_state.after_move(**mv1));
                    let static_position_score_2 =
                        Engine::static_position_evaluation(&board_state.after_move(**mv2));
                    static_position_score_1
                        .partial_cmp(&static_position_score_2)
                        .unwrap()
                })
                .unwrap();
            return Engine::static_position_evaluation(&board_state.after_move(*best_move));
        }
        let subnode_scores = moves
            .iter()
            .map(|mv| {
                let new_position = board_state.after_move(*mv);
                Engine::compute_position_score(&new_position, depth - 1)
            })
            .collect::<Vec<_>>();
        *subnode_scores
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
    fn compute_move(board_state: BoardState, tx: Sender<ChessMove>) {
        // thread::sleep(time::Duration::from_millis(1500));
        let moves = board_state.get_all_legal_moves();
        // FIXME: iterator performance is oof.
        let depth = 2;
        // let best_move = moves.last().unwrap();
        let best_move = moves
            .iter()
            .max_by(|mv1, mv2| {
                let score1 = Engine::compute_position_score(&board_state.after_move(**mv1), depth);
                let score2 = Engine::compute_position_score(&board_state.after_move(**mv2), depth);
                score1.partial_cmp(&score2).unwrap()
            })
            .unwrap()
            .to_owned();
        let _ = tx.send(best_move);
        drop(tx);
    }
    pub fn maybe_calculate_move(&mut self, board_state: BoardState) -> Option<ChessMove> {
        if self.computing_thread_handle.is_none() {
            let tx = self.sender.clone();
            self.computing_thread_handle = Some(thread::spawn(move || {
                Engine::compute_move(board_state, tx);
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
