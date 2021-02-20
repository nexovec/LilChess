
use std::{borrow::{Borrow, BorrowMut}, ops::{Index, IndexMut}};

use graphics::{Rectangle, Texture};
use tetra::{Context, graphics::{self, Color, text::{Text, VectorFontBuilder}}, math::{Vec2, Vec4}};
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
    fn get_hovered_menu_item_index(&mut self,ctx: &mut Context)->Option<i32>{
        let x_offset = 800/2-30;
        let y_offset= 260;
        let x_border = 10;
        let y_border = 10;
        let y_spacing = 5;

        let x_coord = x_offset-x_border;
        let mut h = 0;
        for i in 0..self.text_options.len(){
            let dims = self.text_options.index_mut(i).get_bounds(ctx).unwrap();
            let x_tex = dims.width as i32+x_border*2;
            let y_tex = dims.height as i32+y_border*2;
            h+=dims.height as i32;
            let y_coord = y_offset+h+i as i32*
            (y_spacing + y_border * 2); // FIXME: dims.height isn't constant
            if self.is_mouse_hovered_over_rectangle(ctx,
                Vec2::new(x_coord as f32,y_coord as f32),
                Vec2::new(x_tex as f32,y_tex as f32)){
                return Some(i as i32);
            }
        }
        None
    }
    fn is_mouse_hovered_over_rectangle(&mut self, ctx:&Context, pos: Vec2<f32>, size: Vec2<f32>)->bool{
        let mp = tetra::input::get_mouse_position(ctx);
        println!("x:{}|y:{}", pos.x + size.x, mp.x);
        mp.x >= pos.x && mp.x < pos.x + size.x && mp.y >= pos.y && mp.y < pos.y + size.y
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
        let y_border = 10;
        let y_spacing = 5;
        let box_r = (255.0*0.2) as u8;
        let box_g = (255.0*0.2) as u8;
        let box_b = (255.0*0.2) as u8;
        let box_a = (255.0*0.6) as u8;
        let mut dim:Rectangle;
        graphics::clear(ctx, self.bcg_color);
        // if let Some(item) = Some(1){
        if let Some(item) = self.get_hovered_menu_item_index(ctx){
            //TODO: draw a rectangle around hovered UI item
            // println!("{}", item);
            dim = self.text_options.index_mut(item as usize).get_bounds(ctx).unwrap();
            let y_tex = dim.height as i32+y_border*2;
            let x_tex = dim.width as i32+x_border*2;
            // FIXME: boo, slow!
            let pixels = std::iter::repeat(&[box_r,box_g,box_b,box_a])
            .take(x_tex as usize*y_tex as usize).flatten().copied().collect::<Vec<u8>>();
            let tex = Texture::from_rgba(ctx,x_tex,y_tex,&pixels);
            let x_coord = x_offset-x_border;
            let y_coord = y_offset-y_border+item*(dim.height as i32+y_border*2);
            tex.unwrap().draw(ctx, Vec2::<f32>::new(x_coord as f32,y_coord as f32));
        }
        // draw text
        for i in 0..self.text_options.len(){
            let text = self.text_options.index_mut(i);
            dim = text.get_bounds(ctx).unwrap();
            let x_coord = x_offset;
            let y_coord = y_offset + i as i32 * (dim.height as i32 + y_spacing + y_border * 2);
            text.draw(ctx, Vec2::new( x_coord as f32, y_coord as f32));
        }
        Transition::None
    }
}