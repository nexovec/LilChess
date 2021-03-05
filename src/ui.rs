use tetra::graphics::Texture;
use std::{borrow::{Borrow, BorrowMut}, process::exit};

use tetra::{Context, graphics::text::Text, input::MouseButton, math::Vec2};
// TODO: Research Vec2.as_()

use crate::game::{self, Scene, Transition};
pub struct MenuButton{
    borders: Vec2<i32>,
    pos: Vec2<i32>,
    text:Text,
    on_click:Box<dyn Fn(&mut Context) -> Transition>
}
impl MenuButton{
    pub fn new(borders: Vec2<i32>, pos: Vec2<i32>, text:Text, on_click:Box<dyn Fn(&mut Context) -> Transition>)->MenuButton{
        MenuButton{
            borders,
            pos,
            text:text,
            on_click,
        }
    }
}
impl UIMouseInteractableRect for MenuButton{
    fn check_mouse_interaction(&mut self, ctx: &mut Context)->Transition {
        let temp = self.text.get_bounds(ctx).unwrap();
        if self.is_hovered(ctx, self.pos.borrow(), Vec2::<f32>::new(temp.width, temp.height)+(self.borders*2).as_()){
            if tetra::input::is_mouse_button_pressed(ctx, MouseButton::Left){
                return (*self.on_click)(ctx);
            }
        }
        Transition::None
    }
}
impl Scene for MenuButton{
    fn draw(&mut self, ctx: &mut Context)->Transition {
        let temp = self.text.get_bounds(ctx).unwrap();
        let size: Vec2<f32> = (self.borders*2).as_() + Vec2::new(temp.width,temp.height);
        if self.is_hovered(ctx, &self.pos, size){
            let data = std::iter::repeat(&[160,160,160,200]).take(size.x as usize*size.y as usize)
            .flatten().copied().collect::<Vec<u8>>();
            let texture = Texture::from_rgba(ctx, size.x as i32, size.y as i32, &data);
            texture.unwrap().draw(ctx, self.pos.as_());
        }
        self.text.draw(ctx,(self.pos + self.borders).as_());
        Transition::None
    }

    fn update(&mut self, ctx: &mut Context)->Transition {
        self.check_mouse_interaction(ctx)
    }
}
trait UIMouseInteractableRect{
    fn is_hovered(& self, ctx: &mut Context, pos: &Vec2<i32>, size: Vec2<f32>)->bool{
        let pos = pos.as_();
        let mp = tetra::input::get_mouse_position(ctx);
        mp.x >= pos.x && mp.x < pos.x + size.x && mp.y >= pos.y && mp.y < pos.y + size.y
    }
    fn is_clicked()->bool{
        false
    }
    fn check_mouse_interaction(&mut self, ctx: &mut Context)->game::Transition;
}