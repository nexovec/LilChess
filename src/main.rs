mod game;
mod ui;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::math::Vec2;
use tetra::graphics::text::Text;
use tetra::graphics::text::VectorFontBuilder;
use tetra::graphics::Color;
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
pub struct Assets{
    font: VectorFontBuilder,
    chess_font: VectorFontBuilder,
    w_K:Texture
}
impl Assets{
    pub fn load_assets(ctx:&mut Context)->tetra::Result<Assets>{
        let font = VectorFontBuilder::new("./res/fonts/Exo2.otf")?;
        let chess_font = VectorFontBuilder::new("./res/fonts/chess_font.ttf")?;
        let square_size: i32 = 50;
        let cvs = tetra::graphics::Canvas::new(ctx, square_size, square_size)?;
        tetra::graphics::set_canvas(ctx, &cvs);
        tetra::graphics::clear(ctx, Color::rgba(0., 0., 0., 0.));

        let mut w_K_Text = Text::new("k", chess_font.with_size(ctx,48.)?);
        w_K_Text.draw(ctx, Vec2::new(0.,0.));

        let w_K = cvs.texture().clone();
        tetra::graphics::reset_canvas(ctx);
        Ok(
            Assets{
                font,
                chess_font,
                w_K
            }
        )
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
