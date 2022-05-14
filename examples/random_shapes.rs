use ggez::*;
use oorandom::Rand32;
use ggez::graphics::Rect;
use ggez::conf::WindowMode;

struct State {
    shapes: Vec<Shape>,
}

enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        for shape in &self.shapes {
            // Make the shape...
            let mesh = match shape {
                &Shape::Rectangle(rect) => {
                    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::WHITE)?
                }
                &Shape::Circle(origin, radius) => {
                    graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), origin, radius, 0.1, graphics::Color::WHITE)?
                }
            };
    
            // ...and then draw it.
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }

}

fn main() {
    // let mut rng = Rand32::new(4);
    // let mut shapes = Vec::new();
    // for _ in 0..8 {
    //     if rng.rand_i32() >= 0 {
    //         shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
    //             rng.rand_float() * 800.0,
    //             rng.rand_float() * 600.0,
    //             rng.rand_float() * 800.0,
    //             rng.rand_float() * 600.0,
    //         )));
    //     } else {
    //         shapes.push(Shape::Circle(
    //             mint::Point2{
    //                 x: rng.rand_float() * 800.0,
    //                 y: rng.rand_float() * 600.0,
    //             },
    //             rng.rand_float() * 300.0,
    //         ));
    //     }
    // }
    let mut shapes = Vec::new();
    // shapes.push(Shape::Circle(
    //     mint::Point2{
    //         x: 960.0,
    //         y: 540.0,
    //     },
    //     700.0,
    // ));
    shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
        192.0,
        108.0,
        1536.0,
        864.0,
    )));

    let state = State { shapes: shapes };
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("generative_art", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();
    let width = 1920f32;
    let height = 1080f32;
    let window_mode = WindowMode {
        width,
        height,
        resizable: true,
        ..Default::default()
    };
    graphics::set_mode(&mut ctx, window_mode);
    graphics::set_screen_coordinates(&mut ctx, graphics::Rect{x: 0.0, y: 0.0, w: width, h: height}).unwrap();        
    event::run(ctx, event_loop, state);
}