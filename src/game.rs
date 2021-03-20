#[allow(dead_code)]
pub struct GameContainer{
    history: GameHistory // data
}

impl GameContainer{
    pub fn new()->tetra::Result<GameContainer>{
        let history = GameHistory::new_game()?;
        Ok(GameContainer{
            history
        })
    }
}
#[allow(dead_code)]
struct GameHistory{
    board_states: Vec<BoardState>
}
impl GameHistory{
    pub fn new_game()->tetra::Result<GameHistory>{
        todo!()
    }
}
#[allow(dead_code)]
struct BoardState{
    pieces:Vec<Piece>
}
struct Piece(u8,u8, PieceType, PlayerColor);
#[allow(dead_code)]
enum PieceType{
    PAWN,
    ROOK,
    BISHOP,
    KNIGHT,
    KING,
    QUEEN
}
#[allow(dead_code)]
enum PlayerColor{
    BLACK,
    WHITE
}