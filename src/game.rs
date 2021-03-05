use tetra::{Context, graphics::{Color, text::{Text, VectorFontBuilder}}, math::Vec2};
use tetra::graphics;
use crate::ui::MenuButton;

pub enum Transition{
    Push(Box<dyn Scene>),
    Pop,
    None
}
pub trait Scene{
    fn draw(&mut self, ctx: &mut Context)->Transition;
    fn update(&mut self, ctx: &mut Context)->Transition;
}
pub struct MenuScene{
    bcg_color: Color,
    buttons: Vec<MenuButton>
}

impl MenuScene {
    pub fn new(ctx: &mut Context)->tetra::Result<MenuScene>{
        let font = VectorFontBuilder::new("./res/Exo2-regular.otf")?;
        let size = 32.0;
        let borders = Vec2::new(18,18);

        let pos1 = Vec2::new(300,200);
        let text1 = Text::new("New Game", font.with_size(ctx,size)?);
        let func1 = Box::new(||{Transition::Push(Box::new(GameScene::new()))});
        let btn1 = MenuButton::new(borders,pos1,text1,func1);

        let pos2 = Vec2::new(0,70)+pos1;
        let text2 = Text::new("Quit nub", font.with_size(ctx, size)?);
        let func2 = Box::new(||{Transition::Pop});
        let btn2 = MenuButton::new(borders,pos2,text2,func2);

        Ok(MenuScene{
            bcg_color: Color::rgb(0.2,0.8,0.4),
            buttons: vec![btn1,btn2]
        })
    }
}
// TODO: refactor
impl Scene for MenuScene{
    fn draw(&mut self, ctx: &mut Context)->Transition {
        graphics::clear(ctx, self.bcg_color);
        for i in self.buttons.iter_mut(){
            // TODO: use option instead
            match i.draw(ctx){
                Transition::Pop=>return Transition::Pop,
                Transition::Push(s)=>return Transition::Push(s),
                _=>continue
            }
        }
        Transition::None
    }

    fn update(&mut self, ctx: &mut Context)->Transition {
        for i in self.buttons.iter_mut(){
            match i.update(ctx){
                Transition::Pop=>return Transition::Pop,
                Transition::Push(s)=>return Transition::Push(s),
                _=>continue
            }
        }
        Transition::None
    }
}
struct GameScene;
impl GameScene{
    fn new()->GameScene{
        GameScene
    }
}
impl Scene for GameScene{
    fn draw(&mut self, ctx:&mut Context)->Transition{
        graphics::clear(ctx, Color::BLUE);
        Transition::None
    }
    fn update(&mut self, ctx: &mut Context)->Transition{
        Transition::None
    }
}