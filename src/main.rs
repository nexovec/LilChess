mod game;
mod ui;
use tetra::math::Vec2;
use tetra::graphics::text::Text;
use tetra::graphics::text::VectorFontBuilder;
use tetra::graphics::Color;
use tetra::graphics;
use tetra::Context;
use tetra::graphics::Texture;
use game::{Scene, MenuScene, Transition};
use tetra::{State, window};

struct GameState{
    scenes: Vec<Box<dyn Scene>>
}
impl GameState{
    fn new(ctx: &mut tetra::Context)-> tetra::Result<GameState>{
        let initial_scene = MenuScene::new(ctx)?;
        Ok(GameState{
            scenes: vec![Box::new(initial_scene)]
        })
    }
}
impl State for GameState{
    fn update(&mut self, ctx: &mut tetra::Context) -> tetra::Result{
        // TODO: DRY refactor
        match self.scenes.last_mut(){
            Some(active_scene) => match active_scene.update(ctx){
                Ok(Transition::None)=>{}
                Ok(Transition::Push(s))=>{
                    self.scenes.push(s)
                }
                Ok(Transition::Pop) => {
                    self.scenes.pop();
                },
                Err(_)=>{
                    // TODO: error logging
                    window::quit(ctx);
                }
            },
            None=>window::quit(ctx)
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut tetra::Context) -> tetra::Result{
        match self.scenes.last_mut(){
            Some(active_scene) => match active_scene.draw(ctx){
                Ok(Transition::None)=>{}
                Ok(Transition::Push(s))=>{
                    self.scenes.push(s)
                }
                Ok(Transition::Pop) => {
                    self.scenes.pop();
                },
                Err(_)=>{
                    // FIXME: error logging
                    window::quit(ctx);
                }
            },
            None=>window::quit(ctx)
        }
        return Ok(());
    }
}
#[allow(dead_code)]
pub struct Assets{
    font: VectorFontBuilder,
    w_k:Texture,
    w_q:Texture,
    w_r:Texture,
    w_n:Texture,
    w_b:Texture,
    w_p:Texture,
    b_k:Texture,
    b_q:Texture,
    b_r:Texture,
    b_n:Texture,
    b_b:Texture,
    b_p:Texture
}
impl Assets{
    pub fn load_assets(ctx:&mut Context)->tetra::Result<Assets>{
        let assets:Assets;
        {
            let font = VectorFontBuilder::new("./res/fonts/Exo2.otf")?;
            let chess_font = VectorFontBuilder::new("./res/fonts/chess_font.ttf")?;
            let square_size: i32 = 50;
            let empty = Color::rgba(0., 0., 0., 0.);
            let mut draw_piece: Box<dyn FnMut(&str)->tetra::Result<Texture>>= Box::new(|letter: &str|{
                let cvs = tetra::graphics::Canvas::new(ctx, square_size, square_size)?;
                graphics::set_canvas(ctx, &cvs);
                graphics::clear(ctx, empty);
                let mut text = Text::new(letter, chess_font.with_size(ctx,48.)?);
                text.draw(ctx, Vec2::new(0.,0.));
                Ok(cvs.texture().clone())
            });
            let w_k = draw_piece("k")?;
            let w_q = draw_piece("q")?;
            let w_r = draw_piece("r")?;
            let w_n = draw_piece("h")?;
            let w_b = draw_piece("b")?;
            let w_p = draw_piece("p")?;
            let b_k = draw_piece("l")?;
            let b_q = draw_piece("w")?;
            let b_r = draw_piece("t")?;
            let b_n = draw_piece("j")?;
            let b_b = draw_piece("n")?;
            let b_p = draw_piece("o")?;
            assets = Assets{
                font,
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
                b_p
            };
        }
        tetra::graphics::reset_canvas(ctx);
        Ok(assets)
    }
}



#[allow(unused_must_use)]
fn main()->tetra::Result{
    tetra::ContextBuilder::new("Lil' chess client",1280,720)
    .quit_on_escape(true).show_mouse(true)
    .build()?
    .run(|ctx|GameState::new(ctx));
    tetra::Result::Ok(())
}
