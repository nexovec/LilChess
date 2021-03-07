use tetra::graphics::Canvas;
use tetra::{Context, graphics::{Color, text::{Text, VectorFontBuilder}}, math::Vec2};
use tetra::graphics;
use crate::ui::MenuButton;

pub enum Transition{
    Push(Box<dyn Scene>),
    Pop,
    None
}
pub trait Scene{
    fn draw(&mut self, ctx: &mut Context)->tetra::Result<Transition>;
    fn update(&mut self, ctx: &mut Context)->tetra::Result<Transition>;
}
pub struct MenuScene{
    bcg_color: Color,
    buttons: Vec<MenuButton>
}

impl MenuScene {
    pub fn new(ctx: &mut Context)->tetra::Result<MenuScene>{
        let font = VectorFontBuilder::new("./res/fonts/Exo2.otf")?;
        let size = 32.0;
        let borders = Vec2::new(18,18);

        let pos1 = Vec2::new(300,200);
        let text1 = Text::new("New Game", font.with_size(ctx,size)?);
        let func1 = Box::new(|s: &mut _|{Transition::Push(Box::new(GameScene::new(s).unwrap()))});
        let btn1 = MenuButton::new(borders,pos1,text1,func1);

        let pos2 = Vec2::new(0,70)+pos1;
        let text2 = Text::new("Quit nub", font.with_size(ctx, size)?);
        let func2 = Box::new(|_: &mut _|{Transition::Pop});
        let btn2 = MenuButton::new(borders,pos2,text2,func2);

        Ok(MenuScene{
            bcg_color: Color::rgb(0.2,0.8,0.4),
            buttons: vec![btn1,btn2]
        })
    }
}
// TODO: refactor
impl Scene for MenuScene{
    fn draw(&mut self, ctx: &mut Context)->tetra::Result<Transition> {
        graphics::clear(ctx, self.bcg_color);
        for i in self.buttons.iter_mut(){
            // TODO: use option instead
            match i.draw(ctx){
                Ok(Transition::Pop)=>return Ok(Transition::Pop),
                Ok(Transition::Push(s))=>return Ok(Transition::Push(s)),
                _=>continue
            }
        }
        Ok(Transition::None)
    }

    fn update(&mut self, ctx: &mut Context)->tetra::Result<Transition> {
        for i in self.buttons.iter_mut(){
            match i.update(ctx){
                Ok(Transition::Pop)=>return Ok(Transition::Pop),
                Ok(Transition::Push(s))=>return Ok(Transition::Push(s)),
                _=>continue
            }
        }
        Ok(Transition::None)
    }
}
struct GameScene{
    canvas: Canvas,
    shader: tetra::graphics::Shader
}
impl GameScene{
    fn new(ctx:&mut Context)->tetra::Result<GameScene>{
        let shader = graphics::Shader::from_fragment_file(ctx,"./res/shaders/chessfrag.frag").unwrap();
        let canvas = Canvas::new(ctx,400,400)?;
        graphics::set_canvas(ctx, &canvas);
        graphics::clear(ctx, Color::WHITE);

        graphics::set_shader(ctx, &shader);
        shader.set_uniform(ctx, "viewport", Vec2::<f32>::new(400.0,400.0)); // FIXME: magic numbers
        canvas.draw(ctx, Vec2::<f32>::new(0.0,0.0));
        graphics::reset_canvas(ctx);
        graphics::reset_shader(ctx);
        Ok(GameScene{
            canvas,
            shader
        })
    }
}
impl Scene for GameScene{
    fn draw(&mut self, ctx:&mut Context)->tetra::Result<Transition>{
        graphics::clear(ctx, Color::rgb(180.0, 160.0, 180.0));
        self.canvas.draw(ctx,Vec2::<f32>::new(100.0,100.0));
        Ok(Transition::None)
    }
    fn update(&mut self, ctx: &mut Context)->tetra::Result<Transition>{
        Ok(Transition::None)
    }
}