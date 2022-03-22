use crate::game::*;
use crate::game_types::*;
use crate::ui::{MenuButton, UIFlexBox, UIImage, UIText};
use crate::Assets;
use tetra::graphics;
use tetra::graphics::Texture;
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
// TODO: refactor
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
    selected: Option<Vec2<i8>>,
    should_rerender_pieces: bool,
    should_clear_notes: bool,
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
        graphics::reset_shader(ctx);
        // TODO: chessboard and pieces into one UIFlexBox

        let pieces_box = UIFlexBox::new(
            ctx,
            board_size,
            Vec2::<f32>::new(100.0, 100.0),
            Vec4::<f32>::new(0.0, 0.0, 0.0, 0.0),
            2,
        )?;
        let game = GameContainer::new()?;
        let notes_box = UIFlexBox::new(
            ctx,
            board_size,
            Vec2::<f32>::new(100.0, 100.0),
            Vec4::<f32>::new(0.0, 0.0, 0.0, 0.0),
            2,
        )?;

        let mut flex_box = UIFlexBox::new(
            ctx,
            Vec2::new(400., 500.),
            Vec2::new(740., 100.),
            Vec4::<f32>::new(1.0, 0.0, 0.0, 1.0),
            3,
        )?;
        let text1 = Text::new("bruh", font.with_size(ctx, 16.0)?);
        flex_box.children.push(Box::new(UIText::new(
            ctx,
            Vec2::<f32>::new(0., 0.),
            text1,
            Box::new(|_: &mut _| Transition::None),
            Box::new(|_: &mut _| Transition::None),
        )?));
        Ok(GameScene {
            assets,
            game,
            canvas: board_canvas,
            history_box: flex_box,
            pieces_box,
            notes_box,
            selected: None,
            should_rerender_pieces: true,
            should_clear_notes: true,
        })
    }
    fn post_update(
        &mut self,
        move_to_make: Option<ChessMove>,
        ctx: &mut Context,
    ) -> tetra::Result<Transition> {
        match move_to_make {
            Some(k) => {
                match self.game.execute_move(k) {
                    Some(_) => {
                        self.should_rerender_pieces = true;
                    }
                    None => self.should_rerender_pieces = false,
                }
                self.selected = None;
            }
            None => {}
        }
        if self.should_rerender_pieces {
            let mut new_pieces: Vec<Box<dyn Scene>> = Vec::new();
            for i in self.game.get_board().pieces.iter() {
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
        Ok(Transition::None)
    }
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        let mut move_to_make: Option<ChessMove> = None;
        let board: BoardState = self.game.get_board();
        let square_opt = self.get_selected_square(ctx);
        if square_opt.is_none() {
            return self.post_update(move_to_make, ctx);
        }
        let square = square_opt.unwrap();
        let selected_piece_opt = board.get_piece_at_square(Vec2::new(square.x, square.y));
        if self.selected.is_some() && selected_piece_opt.is_none() {
            let piece = board.get_piece_at_square(self.selected.unwrap()).unwrap();
            let moves = self.game.get_legal_moves(piece);
            for mv in moves {
                if square == Vec2::new(mv.x, mv.y) {
                    move_to_make = Some(ChessMove {
                        from: piece,
                        to: construct_piece(mv.x, mv.y, piece.piece_type, piece.color),
                    });
                    break;
                }
            }
        }
        if selected_piece_opt.is_none() {
            self.selected = None;
            self.should_clear_notes = true;
            return self.post_update(move_to_make, ctx);
        }
        let selected_piece = selected_piece_opt.unwrap();
        if self.game.get_board().player_to_move != selected_piece.color {
            return Ok(Transition::None);
        }
        if self.selected.is_none() {
            let moves = self.game.get_legal_moves(selected_piece);
            self.selected = Some(Vec2::new(selected_piece.x, selected_piece.y).as_());
            // FIXME: don't use graphics in update
            graphics::set_canvas(ctx, &self.notes_box.canvas);
            graphics::clear(ctx, Color::rgba(0., 0., 0., 0.));
            for mv in moves {
                self.assets.green_square.draw(
                    ctx,
                    Vec2::new(50 * mv.x as i32, 400 - 50 * (mv.y + 1) as i32).as_(),
                );
            }
            graphics::reset_canvas(ctx);
            return self.post_update(move_to_make, ctx);
        }
        let currently_selected = self.selected.unwrap();
        let currently_selected_piece = board.get_piece_at_square(currently_selected);
        if currently_selected_piece.is_none() {
            return self.post_update(move_to_make, ctx);
        }
        let piece = currently_selected_piece.unwrap();
        let moves = self.game.get_legal_moves(piece);
        if moves
            .iter()
            .position(|x| Vec2::new(x.x, x.y) == Vec2::new(selected_piece.x, selected_piece.y))
            .is_none()
        {
            return self.post_update(move_to_make, ctx);
        }
        move_to_make = Some(ChessMove {
            from: piece,
            to: construct_piece(
                selected_piece.x,
                selected_piece.y,
                piece.piece_type,
                piece.color,
            ),
        });
        self.should_clear_notes = true;
        self.post_update(move_to_make, ctx)
    }
}
