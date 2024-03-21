//! A very simple shader example.

use crevice::std140::AsStd140;
// use ggegui::{egui, Gui};
use ggez::event;
use ggez::glam::{Vec2, Vec3};
use ggez::graphics::{
    self, BlendMode, Camera3d, Canvas3d, Color, DrawMode, DrawParam, DrawParam3d, FillOptions,
    Image, ImageFormat, Mesh, Mesh3dBuilder, MeshBuilder, Rect, ScreenImage,
};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use std::path;
use std::{env, fs};

#[derive(AsStd140)]
struct Circle {
    size: f32,
    res_x: f32,
    res_y: f32,
}

struct MainState {
    // image: Image,
    circle: Circle,
    shader: graphics::Shader,
    params: graphics::ShaderParams<Circle>,
    cube: graphics::Mesh3d,
    camera: Camera3d,
    // gui: Gui,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut camera = Camera3d::default();
        camera.transform.yaw = 90.0;

        let circle = Circle {
            size: 6.5,
            res_x: 100.0,
            res_y: 100.0,
        };
        let shader = graphics::ShaderBuilder::new()
            .fragment_path("/circle.wgsl")
            .build(&ctx.gfx)?;
        let params = graphics::ShaderParamsBuilder::new(&circle).build(ctx);
        let image = Image::from_path(ctx, "/test.jpg")?;
        let cube = Mesh3dBuilder::new()
            .cube(Vec3::splat(1.0))
            .texture(image)
            .build(ctx);
        camera.transform.yaw = 0.0;
        camera.transform.pitch = 0.0;
        camera.projection.zfar = 1000.0;
        ggez::input::mouse::set_cursor_hidden(ctx, true);
        ggez::input::mouse::set_cursor_grabbed(ctx, true)?;

        Ok(MainState {
            // image,
            circle,
            shader,
            params,
            cube,
            camera,
            // gui: Gui::new(ctx),
        })
    }
}

impl event::EventHandler for MainState {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn resize_event(&mut self, _: &mut Context, width: f32, height: f32) -> GameResult {
        self.camera.projection.resize(width as u32, height as u32);
        self.circle.res_x = width;
        self.circle.res_y = height;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // let gui_ctx = self.gui.ctx();

        // egui::Window::new("UI").show(&gui_ctx, |ui| {
        //     if ui.button("Upload Image").clicked() {
        //         let path = ctx.fs.resources_dir();

        //         let res = rfd::FileDialog::new()
        //             .add_filter("image", &["png", "jpg"])
        //             .set_directory(&path)
        //             .pick_file();
        //         if let Some(res) = res {
        //             if let Ok(image_bytes) = fs::read(res) {
        //                 if let Ok(image) = Image::from_bytes(ctx, image_bytes.as_slice()) {
        //                     self.cube.texture = Some(image);
        //                 }
        //             }
        //         }
        //     }
        // });
        // self.gui.update(ctx);

        let k_ctx = &ctx.keyboard.clone();
        let (yaw_sin, yaw_cos) = self.camera.transform.yaw.sin_cos();
        //let (pitch_sin, pitch_cos) = self.camera.transform.pitch.sin_cos();
        let dt = ctx.time.delta().as_secs_f32();
        let speed = 5.0;
        let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize() * speed * dt;
        let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize() * speed * dt;
        if k_ctx.is_key_pressed(KeyCode::Space) {
            self.camera.transform.position.y += speed * dt;
        }
        if k_ctx.is_key_pressed(KeyCode::C) {
            self.camera.transform.position.y -= speed * dt;
        }
        if k_ctx.is_key_pressed(KeyCode::W) {
            self.camera.transform = self.camera.transform.translate(forward);
        }
        if k_ctx.is_key_pressed(KeyCode::S) {
            self.camera.transform = self.camera.transform.translate(-forward);
        }
        if k_ctx.is_key_pressed(KeyCode::D) {
            self.camera.transform = self.camera.transform.translate(right);
        }
        if k_ctx.is_key_pressed(KeyCode::A) {
            self.camera.transform = self.camera.transform.translate(-right);
        }
        if k_ctx.is_key_pressed(KeyCode::Right) {
            self.camera.transform.yaw += 1.0_f32.to_radians() * dt * 75.0;
        }
        if k_ctx.is_key_pressed(KeyCode::Left) {
            self.camera.transform.yaw -= 1.0_f32.to_radians() * dt * 75.0;
        }
        if k_ctx.is_key_pressed(KeyCode::Up) {
            self.camera.transform.pitch += 1.0_f32.to_radians() * dt * 75.0;
        }
        if k_ctx.is_key_pressed(KeyCode::Down) {
            self.camera.transform.pitch -= 1.0_f32.to_radians() * dt * 75.0;
        }

        if k_ctx.is_key_just_pressed(KeyCode::Escape) {
            ggez::input::mouse::set_cursor_hidden(ctx, !ctx.mouse.cursor_hidden());
            ggez::input::mouse::set_cursor_grabbed(ctx, !ggez::input::mouse::cursor_grabbed(ctx))?;
        }

        if ctx.mouse.cursor_hidden() {
            let mouse_delta = ctx.mouse.raw_delta();
            let speed = 0.5;
            let mouse_delta_y = mouse_delta.y as f32 * speed * dt * -1.0;
            let mouse_delta_x = mouse_delta.x as f32 * speed * dt;
            self.camera.transform.yaw += mouse_delta_x;
            self.camera.transform.pitch += mouse_delta_y;
        }

        if ctx
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Minus)
        {
            self.circle.size += 5.0 * ctx.time.delta().as_secs_f32();
        }
        if ctx
            .keyboard
            .is_key_pressed(ggez::winit::event::VirtualKeyCode::Equals)
        {
            self.circle.size -= 5.0 * ctx.time.delta().as_secs_f32();
        }

        self.circle.size = self.circle.size.clamp(0.1, 100.0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas_image = ScreenImage::new(ctx, ImageFormat::Bgra8UnormSrgb, 1.0, 1.0, 1);
        let mut canvas3d =
            Canvas3d::from_screen_image(ctx, &mut canvas_image, Color::from_rgba(0, 0, 0, 0));

        canvas3d.set_projection(self.camera.to_matrix());

        let draw_param = DrawParam3d::default();
        canvas3d.draw(&self.cube, draw_param);

        canvas3d.finish(ctx)?;

        // let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgba(255, 255, 255, 0));
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgba(0, 0, 0, 0));
        // canvas.set_blend_mode(BlendMode::PREMULTIPLIED);
        // let middle = Vec2::new(
        //     ctx.gfx.drawable_size().0 / 2.0,
        //     ctx.gfx.drawable_size().1 / 2.0,
        // );
        // let circle = Mesh::new_circle(
        //     ctx,
        //     DrawMode::Fill(FillOptions::DEFAULT),
        //     middle,
        //     ctx.gfx.drawable_size().1 / 2.0,
        //     0.5,
        //     Color::BLACK,
        // )?;
        // canvas.draw(&circle, DrawParam::default().dest(Vec2::ZERO));
        // canvas.draw(&self.gui, DrawParam::default().dest(Vec2::ZERO));

        // canvas.draw(&self.image, draw_param);
        self.params.set_uniforms(ctx, &self.circle);
        canvas.set_shader(&self.shader);
        canvas.set_shader_params(&self.params);

        // let draw_param = DrawParam::default()
        //     .scale(Vec2::splat(2.0))
        //     .dest(Vec2::new(100.0, 100.0));
        canvas.draw(&canvas_image.image(ctx), DrawParam::default());

        canvas.finish(ctx)?;
        Ok(())
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

    let cb = ggez::ContextBuilder::new("circle", "ggez")
        .add_resource_path(resource_dir)
        .window_mode(
            ggez::conf::WindowMode::default()
                .transparent(true)
                .visible(false)
                .borderless(true)
                .resizable(true),
        );
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
