use std::{borrow::{Borrow, BorrowMut}, process::exit};

use tetra::{Context, graphics::text::Text, input::MouseButton, math::Vec2};

use crate::game::{self, Scene, Transition};
pub struct MenuButton{
    borders: Vec2<i32>,
    pos: Vec2<i32>,
    text:Text,
    on_click:Box<dyn Fn() -> ()>
}
impl MenuButton{
    pub fn new(borders: Vec2<i32>, pos: Vec2<i32>, text:Text, on_click:Box<dyn Fn() -> ()>)->MenuButton{
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
        if self.is_hovered(ctx, self.pos.borrow(), Vec2::<f32>::new(temp.width, temp.height)){
            if tetra::input::is_mouse_button_pressed(ctx, MouseButton::Left){
                exit(0);
            }
        }
        Transition::None
    }

    fn on_click()->tetra::Result<game::Transition> {
        todo!()
    }
}
impl Scene for MenuButton{
    fn draw(&mut self, ctx: &mut Context)->Transition {
        self.text.draw(ctx,self.pos.as_());
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
        println!("newp {} {}", mp.x, mp.y);
        mp.x >= pos.x && mp.x < pos.x + size.x && mp.y >= pos.y && mp.y < pos.y + size.y
    }
    fn is_clicked()->bool{
        false
    }
    fn check_mouse_interaction(&mut self, ctx: &mut Context)->game::Transition;
    fn on_click()->tetra::Result<game::Transition>;
}