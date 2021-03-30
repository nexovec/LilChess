use std::borrow::Borrow;
use tetra::graphics;
use tetra::graphics::Color;
use tetra::{
    graphics::{Canvas, Shader, Texture},
    math::Vec4,
};

use tetra::{graphics::text::Text, input::MouseButton, math::Vec2, Context};
// TODO: Research Vec2.as_()

use crate::scenes::{Scene, Transition};
pub struct MenuButton {
    borders: Vec2<i32>,
    pos: Vec2<i32>,
    text: Text,
    on_click: Box<dyn Fn(&mut Context) -> Transition>,
}
impl MenuButton {
    pub fn new(
        borders: Vec2<i32>,
        pos: Vec2<i32>,
        text: Text,
        on_click: Box<dyn Fn(&mut Context) -> Transition>,
    ) -> MenuButton {
        MenuButton {
            borders,
            pos,
            text: text,
            on_click,
        }
    }
}
impl UIMouseInteractiveRect for MenuButton {
    fn check_mouse_interaction(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        let temp = self.text.get_bounds(ctx).unwrap();
        if self.is_hovered(
            ctx,
            self.pos.borrow(),
            Vec2::<f32>::new(temp.width, temp.height) + (self.borders * 2).as_(),
        ) {
            if tetra::input::is_mouse_button_pressed(ctx, MouseButton::Left) {
                return Ok((*self.on_click)(ctx));
            }
        }
        Ok(Transition::None)
    }
}
impl Scene for MenuButton {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        let temp = self.text.get_bounds(ctx).unwrap();
        let size: Vec2<f32> = (self.borders * 2).as_() + Vec2::new(temp.width, temp.height);
        if self.is_hovered(ctx, &self.pos, size) {
            let data = std::iter::repeat(&[160, 160, 160, 200])
                .take(size.x as usize * size.y as usize)
                .flatten()
                .copied()
                .collect::<Vec<u8>>();
            let texture = Texture::from_rgba(ctx, size.x as i32, size.y as i32, &data);
            texture.unwrap().draw(ctx, self.pos.as_());
        }
        self.text.draw(ctx, (self.pos + self.borders).as_());
        Ok(Transition::None)
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        self.check_mouse_interaction(ctx)
    }
}
trait UIMouseInteractiveRect {
    fn is_hovered(&self, ctx: &mut Context, pos: &Vec2<i32>, size: Vec2<f32>) -> bool {
        let pos = pos.as_();
        let mp = tetra::input::get_mouse_position(ctx);
        mp.x >= pos.x && mp.x < pos.x + size.x && mp.y >= pos.y && mp.y < pos.y + size.y
    }
    fn is_clicked() -> bool {
        false
    }
    fn check_mouse_interaction(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
}
#[allow(dead_code)]
pub struct UIFlexBox {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    border_width: i32,
    pub children: Vec<Box<dyn Scene>>,
    texture: Texture,
    pub canvas: Canvas,
}
impl UIFlexBox {
    pub fn new(
        ctx: &mut Context,
        size: Vec2<f32>,
        pos: Vec2<f32>,
        border_color: Vec4<f32>,
        border_width: i32,
    ) -> tetra::Result<UIFlexBox> {
        let canvas = Canvas::new(ctx, 400, 500)?;
        let sh: Shader = Shader::from_fragment_file(ctx, "./res/shaders/box_border.frag")?;
        graphics::set_canvas(ctx, &canvas);
        graphics::set_shader(ctx, &sh);
        sh.set_uniform(ctx, "border_width", border_width);
        sh.set_uniform(ctx, "viewport", size);
        sh.set_uniform(ctx, "border_color", border_color);
        canvas.draw(ctx, Vec2::new(0., 0.));
        graphics::reset_canvas(ctx);
        graphics::reset_shader(ctx);

        let children = Vec::<Box<dyn Scene>>::new();
        Ok(UIFlexBox {
            pos,
            size,
            border_width,
            children,
            texture: canvas.texture().clone(),
            canvas,
        })
    }
}
impl Scene for UIFlexBox {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        graphics::set_canvas(ctx, &self.canvas);
        self.texture.draw(ctx, Vec2::new(0., 0.));
        let children: &mut Vec<Box<dyn Scene>> = &mut self.children;
        for child in children {
            child.draw(ctx)?; // TODO: match return
            graphics::set_canvas(ctx, &self.canvas);
        }
        graphics::reset_canvas(ctx);
        self.canvas.draw(ctx, self.pos.as_());
        Ok(Transition::None)
    }
    fn update(&mut self, _ctx: &mut Context) -> tetra::Result<Transition> {
        Ok(Transition::None)
    }
}
#[allow(dead_code)]
pub struct UIText {
    pos: Vec2<f32>,
    contents: Text,
    on_hover: Box<dyn Fn(&mut Context) -> Transition>,
    on_click: Box<dyn Fn(&mut Context) -> Transition>,
}
impl UIText {
    pub fn new(
        _ctx: &mut Context,
        pos: Vec2<f32>,
        contents: Text,
        on_hover: Box<dyn Fn(&mut Context) -> Transition>,
        on_click: Box<dyn Fn(&mut Context) -> Transition>,
    ) -> tetra::Result<UIText> {
        Ok(UIText {
            pos,
            contents,
            on_hover,
            on_click,
        })
    }
}
impl Scene for UIText {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        self.contents.draw(ctx, self.pos);
        Ok(Transition::None)
    }

    fn update(&mut self, _ctx: &mut Context) -> tetra::Result<Transition> {
        Ok(Transition::None)
    }
}
#[allow(dead_code)]
pub struct UIImage {
    pos: Vec2<f32>,
    contents: Texture,
    on_hover: Box<dyn Fn(&mut Context) -> Transition>,
    on_click: Box<dyn Fn(&mut Context) -> Transition>,
}
impl UIImage {
    pub fn new(
        _ctx: &mut Context,
        pos: Vec2<f32>,
        contents: Texture,
        on_hover: Box<dyn Fn(&mut Context) -> Transition>,
        on_click: Box<dyn Fn(&mut Context) -> Transition>,
    ) -> tetra::Result<UIImage> {
        Ok(UIImage {
            pos,
            contents,
            on_hover,
            on_click,
        })
    }
}
impl Scene for UIImage {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        self.contents.draw(ctx, self.pos);
        Ok(Transition::None)
    }

    fn update(&mut self, _ctx: &mut Context) -> tetra::Result<Transition> {
        // TODO:
        Ok(Transition::None)
    }
}
