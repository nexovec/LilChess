use tetra::{Context, State, graphics::text::{Text, VectorFontBuilder}, math::Vec2};


enum Transition{
    Push(Box<dyn Scene>),
    Pop,
    None
}
trait Scene{
    fn update(&mut self, ctx: &mut Context)->Transition;
    fn draw(&mut self, ctx:&mut Context)->Transition;
}
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
        self.scenes.last_mut().unwrap().update(ctx);
        return Ok(());
    }
    fn draw(&mut self, ctx: &mut tetra::Context) -> tetra::Result{
        tetra::graphics::clear(ctx, tetra::graphics::Color::BLUE);
        Ok(())
    }
}


struct MenuScene{
    sample_text: Text
}
impl MenuScene{
    fn new(ctx: &mut Context) -> tetra::Result<MenuScene>{
        let font = VectorFontBuilder::new("./res/font.ttf")?;
        let temp = MenuScene{
            sample_text: Text::new("Hello", font.with_size(ctx, 16.0)?),
        };
        Ok(temp)
    }
}
impl Scene for MenuScene{
    fn update(&mut self, _ctx: &mut Context) -> Transition{
        Transition::None
    }
    fn draw(&mut self, ctx:&mut Context) -> Transition{
        self.sample_text.draw(ctx, Vec2::new(16.0,16.0));
        Transition::None
    }
}
#[allow(unused_must_use)]
fn main()->tetra::Result{
    tetra::ContextBuilder::new("Hello, world!",1280,720)
    .quit_on_escape(true)
    .build()?
    .run(|ctx|GameState::new(ctx));
    tetra::Result::Ok(())
}
