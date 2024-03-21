//! A very simple shader example.

use crevice::std140::AsStd140;
use ggez::event;
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Image, Rect};
use ggez::{Context, GameResult};
use std::env;
use std::path;

#[derive(AsStd140)]
struct Circle {
    size: f32,
}

struct MainState {
    image: Image,
    circle: Circle,
    shader: graphics::Shader,
    params: graphics::ShaderParams<Circle>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let circle = Circle { size: 25.0 };
        let shader = graphics::ShaderBuilder::new()
            .fragment_path("/circle.wgsl")
            .build(&ctx.gfx)?;
        let params = graphics::ShaderParamsBuilder::new(&circle).build(ctx);
        let image = Image::from_path(ctx, "/test.jpg")?;
        Ok(MainState {
            image,
            circle,
            shader,
            params,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Up)
        {
            self.circle.size += 5.0 * ctx.time.delta().as_secs_f32();
        }
        if ctx
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Down)
        {
            self.circle.size -= 5.0 * ctx.time.delta().as_secs_f32();
        }

        self.circle.size = self.circle.size.clamp(0.1, 100.0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        self.params.set_uniforms(ctx, &self.circle);
        canvas.set_shader(&self.shader);
        canvas.set_shader_params(&self.params);

        // let rect = graphics::Mesh::new_rectangle(
        //     ctx,
        //     DrawMode::fill(),
        //     Rect::new(
        //         0.0,
        //         0.0,
        //         self.image.width() as f32,
        //         self.image.height() as f32,
        //     ),
        //     Color::WHITE,
        // )?;
        let draw_param = DrawParam::default()
            .scale(Vec2::splat(2.0))
            .dest(Vec2::new(100.0, 100.0));
        canvas.draw(&self.image, draw_param);

        canvas.finish(ctx)
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("circle", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
