
use std::{borrow::{Borrow, BorrowMut}, ops::{Index, IndexMut}};

use graphics::{Rectangle, Texture};
use tetra::{Context, graphics::{self, Color, text::{Text, VectorFontBuilder}}, math::Vec2};
pub enum Transition{
    Push(Box<dyn Scene>),
    Pop,
    None
}
pub trait Scene{
    fn update(&mut self, ctx: &mut Context)->Transition;
    fn draw(&mut self, ctx:&mut Context)->Transition;
}
pub struct MenuScene{
    bcg_color: Color,
    text_options: Vec<Text>
}
impl MenuScene{
    pub fn new(ctx: &mut Context) -> tetra::Result<MenuScene>{
        let font = VectorFontBuilder::new("./res/font.ttf")?;
        let size:f32 = 24.0;
        let new_game_txt = Text::new("New Game", font.with_size(ctx, size)?);
        let settings_txt = Text::new("Settings", font.with_size(ctx, size)?);
        let quit_txt = Text::new("Quit LOL", font.with_size(ctx, size)?);
        let temp = MenuScene{
            bcg_color: Color::rgb(0.2, 0.8, 0.2),
            text_options: vec![new_game_txt,settings_txt,quit_txt]
        };
        Ok(temp)
    }
    fn get_hovered_menu_item_index(&mut self)->Option<i32>{
        Some(0)
    }
}
impl Scene for MenuScene{
    fn update(&mut self, _ctx: &mut Context) -> Transition{
        Transition::None
    }
    fn draw(&mut self, ctx:&mut Context) -> Transition{
        let x_offset = 800/2 - 30;
        let y_offset = 260;
        let x_border = 10;
        let y_border = 1;
        let y_spacing = 10;
        let box_r = (255.0*0.5) as u8;
        let box_g = (255.0*0.3) as u8;
        let box_b = (255.0*0.8) as u8;
        let box_a = (255.0*0.8) as u8;
        let mut dim:Rectangle;
        graphics::clear(ctx, self.bcg_color);
        #[allow(irrefutable_let_patterns)]
        if let item = self.get_hovered_menu_item_index(){
            //TODO: draw a rectangle around hovered UI item
            let i = item.unwrap();
            dim = self.text_options.index_mut(item.unwrap() as usize).get_bounds(ctx).unwrap();
            let y_tex = dim.height as i32+y_border*2;
            let x_tex = dim.width as i32+x_border*2;
            // FIXME: boo, slow!
            let pixels = std::iter::repeat(&[box_r,box_g,box_b,box_a])
            .take(x_tex as usize*y_tex as usize).flatten().copied().collect::<Vec<u8>>();
            let tex = Texture::from_rgba(ctx,x_tex,y_tex,&pixels);
            let x_coord = x_offset-x_border;
            let y_coord = y_offset-y_border+i*(dim.width as i32+y_border*2);
            tex.unwrap().draw(ctx, Vec2::<f32>::new(x_coord as f32,y_coord as f32));
        }
        for i in 0..self.text_options.len(){
            dim = self.text_options.index_mut(i as usize).get_bounds(ctx).unwrap();
            let text = self.text_options.index_mut(i);
            let x_coord = x_offset-x_border;
            let y_coord= y_offset+i as i32*(dim.height as i32+y_spacing+y_border*2);
            text.draw(ctx, Vec2::new( x_coord as f32, y_coord as f32));
        }
        Transition::None
    }
}