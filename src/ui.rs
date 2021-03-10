use tetra::{graphics::{Color, Canvas, Shader, Texture}, math::Vec4};
use tetra::graphics;
use std::borrow::Borrow;

use tetra::{Context, graphics::text::Text, input::MouseButton, math::Vec2};
// TODO: Research Vec2.as_()

use crate::game::{ Scene, Transition};
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
    fn check_mouse_interaction(&mut self, ctx: &mut Context)->tetra::Result<Transition> {
        let temp = self.text.get_bounds(ctx).unwrap();
        if self.is_hovered(ctx, self.pos.borrow(), Vec2::<f32>::new(temp.width, temp.height)+(self.borders*2).as_()){
            if tetra::input::is_mouse_button_pressed(ctx, MouseButton::Left){
                return Ok((*self.on_click)(ctx));
            }
        }
        Ok(Transition::None)
    }
}
impl Scene for MenuButton{
    fn draw(&mut self, ctx: &mut Context)->tetra::Result<Transition> {
        let temp = self.text.get_bounds(ctx).unwrap();
        let size: Vec2<f32> = (self.borders*2).as_() + Vec2::new(temp.width,temp.height);
        if self.is_hovered(ctx, &self.pos, size){
            let data = std::iter::repeat(&[160,160,160,200]).take(size.x as usize*size.y as usize)
            .flatten().copied().collect::<Vec<u8>>();
            let texture = Texture::from_rgba(ctx, size.x as i32, size.y as i32, &data);
            texture.unwrap().draw(ctx, self.pos.as_());
        }
        self.text.draw(ctx,(self.pos + self.borders).as_());
        Ok(Transition::None)
    }

    fn update(&mut self, ctx: &mut Context)->tetra::Result<Transition> {
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
    fn check_mouse_interaction(&mut self, ctx: &mut Context)->tetra::Result<Transition>;
}
pub struct UIFlexBox{
    border_width:i32,
    border_color: Color,
    children: Vec<Box<dyn Scene>>,
    canvas: Canvas
}
impl UIFlexBox{
    pub fn new(ctx:&mut Context, )->tetra::Result<UIFlexBox>{
        // TODO: params
        let border_width: i32 = 3;

        let canvas = Canvas::new(ctx,400,500)?;
        let sh:Shader = Shader::from_fragment_file(ctx, "./res/shaders/box_border.frag")?;
        graphics::set_canvas(ctx, &canvas);
        graphics::set_shader(ctx, &sh);

        sh.set_uniform(ctx, "border_width", border_width);
        sh.set_uniform(ctx, "viewport", Vec2::new(400.,500.)); // FIXME: magic numbers
        sh.set_uniform(ctx, "border_color", Vec4::<f32>::new(1.0,0.0,0.0,1.0));
        canvas.draw(ctx, Vec2::new(0.,0.));

        graphics::reset_canvas(ctx);
        graphics::reset_shader(ctx);

        let children = Vec::<Box<dyn Scene>>::new();
        Ok(
            UIFlexBox{
                border_width: border_width,
                border_color: Color::BLUE,
                children,
                canvas
            }
        )
    }
}
impl Scene for UIFlexBox{
    fn draw(&mut self, ctx: &mut Context)->tetra::Result<Transition>{
        self.canvas.draw(ctx, Vec2::new(740.,100.));
        Ok(Transition::None)
    }
    fn update(&mut self, ctx: &mut Context)->tetra::Result<Transition>{
        Ok(Transition::None)
    }
}