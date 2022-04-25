use crate::game_types::*;
use std::sync::mpsc;
use std::thread;
use std::time;
pub struct Engine {
    sender: mpsc::Sender<ChessMove>,
    receiver: mpsc::Receiver<ChessMove>,
    computing_thread_handle: Option<thread::JoinHandle<()>>,
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
    pub fn maybe_calculate_move(&mut self, board_state: BoardState) -> Option<ChessMove> {
        if self.computing_thread_handle.is_none() {
            let tx = self.sender.clone();
            self.computing_thread_handle = Some(thread::spawn(move || {
                thread::sleep(time::Duration::from_millis(1500));
                let moves = board_state.get_all_legal_moves();
                assert!(moves.len() != 0);
                let _ = tx.send(moves.last().unwrap().clone());
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
