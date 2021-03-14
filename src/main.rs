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
pub struct Assets{
    font: VectorFontBuilder,
    w_K:Texture,
    w_Q:Texture,
    w_R:Texture,
    w_N:Texture,
    w_B:Texture,
    w_P:Texture,
    b_K:Texture,
    b_Q:Texture,
    b_R:Texture,
    b_N:Texture,
    b_B:Texture,
    b_P:Texture
}
impl Assets{
    pub fn load_assets(ctx:&mut Context)->tetra::Result<Assets>{
        let assets:Assets;
        {
            let font = VectorFontBuilder::new("./res/fonts/Exo2.otf")?;
            let chess_font = VectorFontBuilder::new("./res/fonts/chess_font.ttf")?;
            let square_size: i32 = 50;
            let empty = Color::rgba(0., 0., 0., 0.);
            let mut drawpiece: Box<dyn FnMut(&str)->tetra::Result<Texture>>= Box::new(|letter: &str|{
                let cvs = tetra::graphics::Canvas::new(ctx, square_size, square_size)?;
                graphics::set_canvas(ctx, &cvs);
                graphics::clear(ctx, empty);
                let mut text = Text::new(letter, chess_font.with_size(ctx,48.)?);
                text.draw(ctx, Vec2::new(0.,0.));
                Ok(cvs.texture().clone())
            });
            let w_K = drawpiece("k")?;
            let w_Q = drawpiece("q")?;
            let w_R = drawpiece("r")?;
            let w_N = drawpiece("h")?;
            let w_B = drawpiece("b")?;
            let w_P = drawpiece("p")?;
            let b_K = drawpiece("l")?;
            let b_Q = drawpiece("w")?;
            let b_R = drawpiece("t")?;
            let b_N = drawpiece("j")?;
            let b_B = drawpiece("n")?;
            let b_P = drawpiece("o")?;
            assets = Assets{
                font,
                w_K,
                w_Q,
                w_R,
                w_N,
                w_B,
                w_P,
                b_K,
                b_Q,
                b_R,
                b_N,
                b_B,
                b_P
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
