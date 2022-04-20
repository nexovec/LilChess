mod game;
mod game_types;
mod scenes;
mod ui;
use scenes::{MenuScene, Scene, Transition};
use tetra::graphics;
use tetra::graphics::text::Text;
use tetra::graphics::text::VectorFontBuilder;
use tetra::graphics::Color;
use tetra::graphics::Texture;
use tetra::math::Vec2;
use tetra::Context;
use tetra::{window, State};

struct GameState {
    scenes: Vec<Box<dyn Scene>>,
}
impl GameState {
    fn new(ctx: &mut tetra::Context) -> tetra::Result<GameState> {
        let initial_scene = MenuScene::new(ctx)?;
        Ok(GameState {
            scenes: vec![Box::new(initial_scene)],
        })
    }
}
impl State for GameState {
    fn update(&mut self, ctx: &mut tetra::Context) -> tetra::Result {
        // TODO: DRY refactor
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(ctx) {
                Ok(Transition::None) => {}
                Ok(Transition::Push(s)) => self.scenes.push(s),
                Ok(Transition::Pop) => {
                    self.scenes.pop();
                }
                Err(_) => {
                    // TODO: error logging
                    window::quit(ctx);
                }
            },
            None => window::quit(ctx),
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut tetra::Context) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.draw(ctx) {
                Ok(Transition::None) => {}
                Ok(Transition::Push(s)) => self.scenes.push(s),
                Ok(Transition::Pop) => {
                    self.scenes.pop();
                }
                Err(_) => {
                    // FIXME: error logging
                    window::quit(ctx);
                }
            },
            None => window::quit(ctx),
        }
        return Ok(());
    }
}
#[allow(dead_code)]
pub struct Assets {
    font: VectorFontBuilder,
    green_square: Texture,
    w_k: Texture,
    w_q: Texture,
    w_r: Texture,
    w_n: Texture,
    w_b: Texture,
    w_p: Texture,
    b_k: Texture,
    b_q: Texture,
    b_r: Texture,
    b_n: Texture,
    b_b: Texture,
    b_p: Texture,
    alpha_color: Color,
}
impl Assets {
    pub fn load_assets(ctx: &mut Context) -> tetra::Result<Assets> {
        let assets: Assets;
        let alpha_color = Color::rgba(0., 0., 0., 0.);
        {
            let font = VectorFontBuilder::new("./res/fonts/Exo2.otf")?;
            let chess_font = VectorFontBuilder::new("./res/fonts/chess_font.ttf")?;
            let square_size: usize = 50;

            let blacken =
                tetra::graphics::Shader::from_fragment_file(ctx, "./res/shaders/blacken.frag")?;
            let mut draw_piece: Box<dyn FnMut(&str, bool) -> tetra::Result<Texture>> =
                Box::new(|letter: &str, is_black: bool| {
                    let cvs =
                        tetra::graphics::Canvas::new(ctx, square_size as i32, square_size as i32)?;
                    graphics::set_canvas(ctx, &cvs);
                    graphics::clear(ctx, alpha_color);
                    let mut text = Text::new(letter, chess_font.with_size(ctx, 48.)?);
                    text.draw(ctx, Vec2::new(0., 0.));
                    if is_black {
                        graphics::set_shader(ctx, &blacken);
                        cvs.draw(ctx, Vec2::new(0., 0.));
                        graphics::reset_shader(ctx);
                    }
                    Ok(cvs.texture().clone())
                });
            let w_k = draw_piece("l", false)?;
            let w_q = draw_piece("w", false)?;
            let w_r = draw_piece("t", false)?;
            let w_n = draw_piece("j", false)?;
            let w_b = draw_piece("n", false)?;
            let w_p = draw_piece("o", false)?;
            let b_k = draw_piece("l", true)?;
            let b_q = draw_piece("w", true)?;
            let b_r = draw_piece("t", true)?;
            let b_n = draw_piece("j", true)?;
            let b_b = draw_piece("n", true)?;
            let b_p = draw_piece("o", true)?;

            std::mem::drop(draw_piece);
            tetra::graphics::reset_canvas(ctx);
            let data = std::iter::repeat(&[
                (0.1 * 255.) as u8,
                (0.8 * 255.) as u8,
                (0.1 * 255.) as u8,
                (0.8 * 255.) as u8,
            ])
            .take(square_size * square_size)
            .flatten()
            .copied()
            .collect::<Vec<u8>>();
            let green_square = Texture::from_rgba(ctx, 50, 50, &data)?;
            assets = Assets {
                font,
                green_square,
                w_k,
                w_q,
                w_r,
                w_n,
                w_b,
                w_p,
                b_k,
                b_q,
                b_r,
                b_n,
                b_b,
                b_p,
                alpha_color,
            };
        }
        Ok(assets)
    }
}

#[allow(unused_must_use)]
fn main() -> tetra::Result {
    tetra::ContextBuilder::new("Lil' chess client", 1280, 720)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(|ctx| GameState::new(ctx))
}
