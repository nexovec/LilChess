use crate::game::*;
use crate::game_types::*;
use crate::ui::{MenuButton, UIFlexBox, UIImage, UIText};
use crate::Assets;
use tetra::graphics;
use tetra::graphics::text::VectorFontBuilder;
use tetra::graphics::Texture;
use tetra::TetraError;
use tetra::{graphics::Canvas, math::Vec4};
use tetra::{
    graphics::{text::Text, Color},
    math::Vec2,
    Context,
};
pub enum Transition {
    Push(Box<dyn Scene>),
    Pop,
    None,
}
pub trait Scene {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
}
pub struct MenuScene {
    bcg_color: Color,
    buttons: Vec<MenuButton>,
}

impl MenuScene {
    pub fn new(ctx: &mut Context) -> tetra::Result<MenuScene> {
        let font = Assets::load_assets(ctx)?.font;
        let size = 32.0;
        let borders = Vec2::new(18, 18);
        let pos1 = Vec2::new(300, 200);
        let text1 = Text::new("New Game", font.with_size(ctx, size)?);
        let func1 = Box::new(|s: &mut _| Transition::Push(Box::new(GameScene::new(s).unwrap())));
        let btn1 = MenuButton::new(borders, pos1, text1, func1);

        let pos2 = Vec2::new(0, 70) + pos1;
        let text2 = Text::new("Quit nub", font.with_size(ctx, size)?);
        let func2 = Box::new(|_: &mut _| Transition::Pop);
        let btn2 = MenuButton::new(borders, pos2, text2, func2);
        let unit = 1.0 / 255.;
        Ok(MenuScene {
            bcg_color: Color::rgb(unit * 196., unit * 196., unit * 196.),
            buttons: vec![btn1, btn2],
        })
    }
}
impl Scene for MenuScene {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        graphics::clear(ctx, self.bcg_color);
        for i in self.buttons.iter_mut() {
            match i.draw(ctx) {
                Ok(Transition::Pop) => return Ok(Transition::Pop),
                Ok(Transition::Push(s)) => return Ok(Transition::Push(s)),
                _ => continue,
            }
        }
        Ok(Transition::None)
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        for i in self.buttons.iter_mut() {
            match i.update(ctx) {
                Ok(Transition::Pop) => return Ok(Transition::Pop),
                Ok(Transition::Push(s)) => return Ok(Transition::Push(s)),
                _ => continue,
            }
        }
        Ok(Transition::None)
    }
}
struct GameScene {
    assets: Assets,
    game: GameContainer,
    canvas: Canvas,
    history_box: UIFlexBox,
    pieces_box: UIFlexBox,
    notes_box: UIFlexBox,
    selected: Option<Piece>,
    should_rerender_pieces: bool,
    should_clear_notes: bool,
    white_time_limit: f32,
    black_time_limit: f32,
    white_time_remaining: f32,
    black_time_remaining: f32,
    player_whose_time_is_ticking: Option<PlayerColor>,
}
impl GameScene {
    fn new(ctx: &mut Context) -> tetra::Result<GameScene> {
        let assets = Assets::load_assets(ctx)?;
        let board_size = Vec2::<f32>::new(400.0, 400.0);
        let font = &assets.font;

        let shader =
            graphics::Shader::from_fragment_file(ctx, "./res/shaders/chessfrag.frag").unwrap();
        let board_canvas = Canvas::new(ctx, board_size.x as i32, board_size.y as i32)?;
        graphics::set_canvas(ctx, &board_canvas);
        graphics::clear(ctx, Color::WHITE);
        graphics::set_shader(ctx, &shader);
        shader.set_uniform(ctx, "viewport", board_size);
        board_canvas.draw(ctx, Vec2::<f32>::new(0.0, 0.0));
        graphics::reset_canvas(ctx);

        let pieces_box = UIFlexBox::new(
            ctx,
            board_size,
            Vec2::<f32>::new(100.0, 100.0),
            Vec4::<f32>::new(0.0, 0.0, 0.0, 0.0),
            2,
        )?;
        let game = GameContainer::new();
        let notes_box = UIFlexBox::new(
            ctx,
            board_size,
            Vec2::<f32>::new(100.0, 100.0),
            Vec4::<f32>::new(0.0, 0.0, 0.0, 0.0),
            2,
        )?;

        let mut history_box = UIFlexBox::new(
            ctx,
            Vec2::new(400., 500.),
            Vec2::new(740., 100.),
            Vec4::<f32>::new(1.0, 0.0, 0.0, 1.0),
            3,
        )?;
        let text1 = Text::new("bruh", font.with_size(ctx, 16.0)?);
        history_box.children.push(Box::new(UIText::new(
            ctx,
            Vec2::<f32>::new(0., 0.),
            text1,
            Box::new(|_: &mut _| Transition::None),
            Box::new(|_: &mut _| Transition::None),
        )?));
        let white_time_limit = 500.0;
        let black_time_limit = 500.0;
        Ok(GameScene {
            assets,
            game,
            canvas: board_canvas,
            history_box,
            pieces_box,
            notes_box,
            selected: None,
            should_rerender_pieces: true,
            should_clear_notes: true,
            white_time_limit: white_time_limit,
            black_time_limit: black_time_limit,
            white_time_remaining: white_time_limit,
            black_time_remaining: black_time_limit,
            player_whose_time_is_ticking: None,
        })
    }
    pub fn draw_timers(&self, ctx: &mut Context) -> tetra::Result<Transition> {
        // FIXME: No, don't copy the font for each timer
        // FIXME: And no, don't use VectorFontBuilder here
        // FIXME: white timer is the black timer and the black timer is the white timer
        let font = VectorFontBuilder::new("./res/fonts/Exo2.otf")?.with_size(ctx, 32.0)?;
        let black_text = format!(
            "{}:{}",
            self.black_time_remaining as i32 / 60,
            self.black_time_remaining as i32 % 60
        );
        let mut white_timer: MenuButton = MenuButton::new(
            Vec2::<i32>::new(10, 10),
            Vec2::<i32>::new(530, 430),
            Text::new(black_text, font.clone()),
            Box::new(|ctx: &mut Context| Transition::None),
        );
        white_timer.draw(ctx)?;
        let white_text = format!(
            "{}:{}",
            self.white_time_remaining as i32 / 60,
            self.white_time_remaining as i32 % 60
        );
        let mut white_timer: MenuButton = MenuButton::new(
            Vec2::<i32>::new(10, 10),
            Vec2::<i32>::new(530, 120),
            Text::new(white_text, font.clone()),
            Box::new(|ctx: &mut Context| Transition::None),
        );
        white_timer.draw(ctx)?;
        Ok(Transition::None)
    }
    pub fn on_check(&self) {
        println!("It's check!");
    }
    pub fn on_checkmate(&self) {
        println!("It's checkmate!");
    }
    pub fn on_piece_taken(&mut self) -> () {
        println!("I've taken a piece");
    }
    pub fn execute_move(&mut self, mv: ChessMove) -> Option<ChessMove> {
        if self
            .game
            .get_board()
            .pieces
            .iter()
            .any(|piece_there| mv.to.pos() == piece_there.pos())
        {
            self.on_piece_taken();
        }
        self.game.history.execute_move(mv);
        if self.game.get_board().is_check(None) {
            self.on_check();
        }
        if self.game.get_board().get_all_legal_moves().len() == 0 {
            self.on_checkmate();
        }
        Some(mv)
    }
    pub fn handle_move(
        &mut self,
        board: &BoardState,
        newly_selected_square: Vec2<i8>,
        newly_selected_piece: &Piece,
        ctx: &mut Context,
    ) {
        self.selected = board.get_piece_at_square(newly_selected_square);
        let new_moves = board.get_legal_moves(&newly_selected_piece);
        self.highlight_squares(&new_moves, ctx);
        self.should_rerender_pieces = true;
    }
    fn highlight_squares(&mut self, moves: &Vec<ChessMove>, ctx: &mut Context) {
        graphics::set_canvas(ctx, &self.notes_box.canvas);
        graphics::clear(ctx, Color::rgba(0., 0., 0., 0.));
        for mv in moves {
            self.assets.green_square.draw(
                ctx,
                Vec2::new(50 * mv.to.x as i32, 400 - 50 * (mv.to.y + 1) as i32).as_(),
            );
        }
        graphics::reset_canvas(ctx);
    }
    fn post_update(
        &mut self,
        move_to_make: Option<ChessMove>,
        ctx: &mut Context,
    ) -> tetra::Result<Transition> {
        let board = self.game.get_board();
        if let Some(k) = move_to_make {
            self.should_rerender_pieces = self.execute_move(k).is_some();
            self.selected = None;
            self.player_whose_time_is_ticking =
                Some(PlayerColor::opposite(self.game.get_board().player_to_move));
        }
        if self.should_rerender_pieces {
            let mut new_pieces: Vec<Box<dyn Scene>> = Vec::new();
            for i in board.pieces.iter() {
                new_pieces.push(Box::new(GameScene::get_image(i, &mut self.assets, ctx)?));
            }
            let _ = std::mem::replace(&mut self.pieces_box.children, new_pieces);
        }
        if self.should_clear_notes {
            // FIXME: don't use graphics in update
            graphics::set_canvas(ctx, &self.notes_box.canvas);
            graphics::clear(ctx, Color::rgba(0., 0., 0., 0.));
            graphics::reset_canvas(ctx);
            self.should_clear_notes = false;
        }
        Ok(Transition::None)
    }
    fn get_image(piece: &Piece, a: &Assets, ctx: &mut Context) -> tetra::Result<UIImage> {
        let i: &Texture;
        type P = PieceType;
        match piece.color {
            PlayerColor::BLACK => match piece.piece_type {
                P::BISHOP => i = &a.b_b,
                P::KNIGHT => i = &a.b_n,
                P::ROOK => i = &a.b_r,
                P::KING => i = &a.b_k,
                P::QUEEN => i = &a.b_q,
                P::PAWN => i = &a.b_p,
            },
            PlayerColor::WHITE => match piece.piece_type {
                P::BISHOP => i = &a.w_b,
                P::KNIGHT => i = &a.w_n,
                P::ROOK => i = &a.w_r,
                P::KING => i = &a.w_k,
                P::QUEEN => i = &a.w_q,
                P::PAWN => i = &a.w_p,
            },
        }
        let back = UIImage::new(
            ctx,
            Vec2::new(
                (piece.x as i32 * 50) as f32,
                ((7 - piece.y) as i32 * 50) as f32,
            ),
            i.clone(),
            Box::new(|_: &mut _| Transition::None),
            Box::new(|_: &mut _| Transition::None),
        )?;
        Ok(back)
    }
    fn is_hovered(&self, ctx: &mut Context, pos: &Vec2<i32>, size: Vec2<f32>) -> bool {
        let pos = pos.as_();
        let mp = tetra::input::get_mouse_position(ctx);
        mp.x >= pos.x && mp.x < pos.x + size.x && mp.y >= pos.y && mp.y < pos.y + size.y
    }
    fn get_selected_square(&mut self, ctx: &mut Context) -> Option<Vec2<i8>> {
        if tetra::input::is_mouse_button_pressed(ctx, tetra::input::MouseButton::Left) {
            if self.is_hovered(ctx, &self.pieces_box.pos.as_(), self.pieces_box.size.as_()) {
                let mp = tetra::input::get_mouse_position(ctx);
                let x = mp.x - self.pieces_box.pos.x;
                let y = self.pieces_box.pos.y + self.pieces_box.size.y - mp.y;
                if x > 400. || y > 400. {
                    return None;
                }
                return Some(Vec2::<i8>::new((x / 50.) as i8, (y / 50.) as i8));
            }
        }
        None
    }
}
impl Scene for GameScene {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        let unit = 1.0 / 255.;
        graphics::clear(ctx, Color::rgb(unit * 196., unit * 196., unit * 196.));
        self.canvas.draw(ctx, Vec2::<f32>::new(100.0, 100.0));
        self.notes_box.draw(ctx)?;
        if self.should_rerender_pieces {
            graphics::set_canvas(ctx, &self.pieces_box.canvas);
            graphics::clear(ctx, self.assets.alpha_color);
            graphics::reset_canvas(ctx);
        }
        self.pieces_box.draw(ctx)?;
        self.history_box.draw(ctx)?;
        self.draw_timers(ctx)?;
        Ok(Transition::None)
    }
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        // TODO: clean up
        let mut move_to_make: Option<ChessMove> = None;
        let board: BoardState = self.game.get_board();
        let increment: f32 = tetra::time::get_delta_time(ctx).as_millis() as f32 / 1000.0;
        if let Some(player_color) = self.player_whose_time_is_ticking {
            let timer_ref = match player_color {
                PlayerColor::BLACK => &mut self.black_time_remaining,
                PlayerColor::WHITE => &mut self.white_time_remaining,
            };
            *timer_ref -= increment;
        }
        if let Some(newly_selected_square) = self.get_selected_square(ctx) {
            if let Some(selected_piece) = self.selected {
                // make a move if you can here:
                let moves = board.get_legal_moves(&selected_piece);
                for avlbl_move in moves {
                    if avlbl_move.to.pos() == newly_selected_square {
                        move_to_make = Some(avlbl_move);
                        self.should_clear_notes = true;
                    }
                }
                if let Some(newly_selected_piece) = self
                    .game
                    .get_board()
                    .get_piece_at_square(newly_selected_square)
                {
                    if move_to_make.is_none() {
                        if newly_selected_piece.color == selected_piece.color {
                            self.handle_move(
                                &board,
                                newly_selected_square,
                                &newly_selected_piece,
                                ctx,
                            );
                        } else {
                            self.should_clear_notes = true;
                            self.selected = None;
                        }
                    }
                } else {
                    self.should_clear_notes = true;
                    self.selected = None;
                }
            } else {
                if let Some(newly_selected_piece) = self
                    .game
                    .get_board()
                    .get_piece_at_square(newly_selected_square)
                {
                    if newly_selected_piece.color == board.player_to_move {
                        self.handle_move(&board, newly_selected_square, &newly_selected_piece, ctx);
                    }
                }
            }
        }
        self.post_update(move_to_make, ctx)
    }
}
