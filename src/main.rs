mod game;
mod ui;
use game::{Scene, MenuScene};
use tetra::{Context, State, graphics::text::{Text, VectorFontBuilder}, math::Vec2, window};

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
        match self.scenes.last_mut(){
            Some(active_scene) => match active_scene.update(ctx){
                game::Transition::None=>{}
                game::Transition::Push(s)=>{
                    self.scenes.push(s)
                }
                game::Transition::Pop => {
                    self.scenes.pop();
                }
            },
            None=>window::quit(ctx)
        }
        return Ok(());
    }
    fn draw(&mut self, ctx: &mut tetra::Context) -> tetra::Result{
        match self.scenes.last_mut(){
            Some(active_scene) => match active_scene.draw(ctx){
                game::Transition::None=>{}
                game::Transition::Push(s)=>{
                    self.scenes.push(s)
                }
                game::Transition::Pop => {
                    self.scenes.pop();
                }
            },
            None=>window::quit(ctx)
        }
        return Ok(());
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
