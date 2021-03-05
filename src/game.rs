use tetra::{Context, graphics::{Color, text::{Text, VectorFontBuilder}}, math::Vec2};
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

        let borders1 = Vec2::new(10,10);
        let pos1 = Vec2::new(10,10);
        let text1 = Text::new("Quit nub", font.with_size(ctx, size)?);
        let func1 = Box::new(||{Transition::Pop});
        let btn1 = MenuButton::new(borders1,pos1,text1,func1);
        Ok(MenuScene{
            bcg_color: Color::rgb(0.2,0.8,0.4),
            buttons: vec![btn1]
        })
    }
}
// TODO: refactor
impl Scene for MenuScene{
    fn draw(&mut self, ctx: &mut Context)->Transition {
        for i in self.buttons.iter_mut(){
            match i.draw(ctx){
                Transition::Pop=>return Transition::Pop,
                _=>continue
            }
        }
        Transition::None
    }

    fn update(&mut self, ctx: &mut Context)->Transition {
        for i in self.buttons.iter_mut(){
            match i.update(ctx){
                Transition::Pop=>return Transition::Pop,
                _=>continue
            }
        }
        Transition::None
    }
}