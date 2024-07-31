use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{self, Key};
use tetra::math::num_traits::ToPrimitive;
use tetra::math::Vec2;
use tetra::{window, Context, ContextBuilder, State};

struct GameState {
    texture: Texture,
    position: Vec2<f32>,
    speed: f32,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            texture: Texture::new(ctx, "./res/pixel.png")?,
            position: Vec2::new(32.0, 32.0),
            speed: 14.0,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let _window_size = window::get_current_monitor_size(ctx);

        if self.position.x < 0.0 {
            self.position.x = 10.0;
        }

        if input::is_key_down(ctx, Key::A) {
            self.position.x -= self.speed;
        }

        if input::is_key_down(ctx, Key::D) {
            self.position.x += self.speed;
        }

        if input::is_key_down(ctx, Key::W) {
            self.position.y -= self.speed;
        }

        if input::is_key_down(ctx, Key::S) {
            self.position.y += self.speed;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));

        self.texture.draw(
            ctx,
            DrawParams::new()
                .position(self.position)
                .origin(Vec2::new(8.0, 8.0)),
        );

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Keyboard Input", 900, 600)
        .quit_on_escape(true)
        .resizable(true)
        .maximized(true)
        .build()?
        .run(GameState::new)
}
