use tetra::{graphics::{Canvas, Shader, Texture, UniformValue}, math::Vec4};
use tetra::{Context, graphics::{Color, text::{Text, VectorFontBuilder}}, math::Vec2};
use tetra::graphics;
use crate::ui::{MenuButton, UIFlexBox, UIImage, UIText};

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
        let unit = 1.0/255.;
        Ok(MenuScene{
            bcg_color: Color::rgb(unit*196.,unit*196.,unit*196.),
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
    history_box: UIFlexBox,
    pieces_box: UIFlexBox
}
impl GameScene{
    fn new(ctx:&mut Context)->tetra::Result<GameScene>{
        let board_size = Vec2::<f32>::new(400.0,400.0);
        let font = VectorFontBuilder::new("./res/fonts/Exo2.otf")?;
        let chess_font = VectorFontBuilder::new("./res/fonts/chess_font.ttf")?;

        let shader = graphics::Shader::from_fragment_file(ctx,"./res/shaders/chessfrag.frag").unwrap();
        let board_canvas = Canvas::new(ctx,board_size.x as i32,board_size.y as i32)?;
        graphics::set_canvas(ctx, &board_canvas);
        graphics::clear(ctx, Color::WHITE);
        graphics::set_shader(ctx, &shader);
        shader.set_uniform(ctx, "viewport", board_size);
        board_canvas.draw(ctx, Vec2::<f32>::new(0.0,0.0));
        graphics::reset_canvas(ctx);
        graphics::reset_shader(ctx);
        // TODO: chessboard and pieces into one UIFlexBox

        let mut chess_text_1  = Text::new("k",chess_font.with_size(ctx, 25.)?);
        let chess_piece_image = Canvas::new(ctx, 50,50)?;
        graphics::set_canvas(ctx, &chess_piece_image);
        chess_text_1.draw(ctx, Vec2::new(0.,0.));
        graphics::reset_canvas(ctx);
        let chess_piece = UIImage::new(
            ctx,
            Vec2::new(0.,0.),
            chess_piece_image.texture().clone(),
            Box::new(|_:&mut _|{Transition::None}), Box::new(|_: &mut _|{Transition::None})
        )?;

        let mut pieces_box = UIFlexBox::new(ctx, board_size,Vec2::<f32>::new(100.0,100.0),Vec4::<f32>::new(0.0,0.0,0.0,0.0),2)?;
        pieces_box.children.push(Box::new(chess_piece));

        let mut flex_box= UIFlexBox::new(
            ctx, Vec2::new(400.,500.),Vec2::new(740.,100.), Vec4::<f32>::new(1.0,0.0,0.0,1.0), 3)?;
        // FIXME: dry... Assets struct?
        let text1 = Text::new("bruh", font.with_size(ctx, 16.0)?);
        flex_box.children.push(Box::new(UIText::new(ctx, Vec2::<f32>::new(0.,0.),text1,Box::new(|_:&mut _|{Transition::None}),Box::new(|_:&mut _|{Transition::None}))?));
        Ok(GameScene{
            canvas: board_canvas,
            history_box: flex_box,
            pieces_box
        })
    }
}
impl Scene for GameScene{
    fn draw(&mut self, ctx:&mut Context)->tetra::Result<Transition>{
        let unit = 1.0/255.;
        graphics::clear(ctx, Color::rgb(unit*196.,unit*196.,unit*196.));
        self.canvas.draw(ctx,Vec2::<f32>::new(100.0,100.0)); // FIXME: DRY
        self.pieces_box.draw(ctx);
        // TODO: draw pieces
        self.history_box.draw(ctx)?;
        Ok(Transition::None)
    }
    fn update(&mut self, ctx: &mut Context)->tetra::Result<Transition>{
        Ok(Transition::None)
    }
}