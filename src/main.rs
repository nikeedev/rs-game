use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{self, Key};
use tetra::math::*;
use tetra::math::num_traits::ToPrimitive;
use tetra::math::Vec2;
use tetra::{window, Context, ContextBuilder, State};

struct Assets {
    player: Texture,
    mouse: Texture,
}

struct GameState {   
    assets: Assets,
    player_position: Vec2<f32>,
    player_speed: f32,
    mouse_position: Vec2<f32>
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            assets: Assets {
                player: Texture::new(ctx, "./res/pixel.png")?,
                mouse: Texture::new(ctx, "./res/mouse.png")?
            },
            player_position: Vec2::new(32.0, 32.0),
            player_speed: 14.0,
            mouse_position: input::get_mouse_position(ctx)
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let _window_size = window::get_size(ctx);
        self.mouse_position = input::get_mouse_position(ctx);


        if self.player_position.x < 0.0 {
            self.player_position.x = 10.0;
        }

        if self.player_position.y < 0.0 {
            self.player_position.y = 10.0;
        }

        if self.player_position.x + self.assets.player.width().to_f32().unwrap() > _window_size.0.to_f32().unwrap() {
            self.player_position.x = _window_size.0.to_f32().unwrap() - self.assets.player.width().to_f32().unwrap() - 10.0;
        }

        if self.player_position.y + self.assets.player.height().to_f32().unwrap() > _window_size.1.to_f32().unwrap() {
            self.player_position.y = _window_size.1.to_f32().unwrap() - 10.0 - self.assets.player.height().to_f32().unwrap();
        }



        if input::is_key_down(ctx, Key::A) || input::is_key_down(ctx, Key::Left) {
            self.player_position.x -= self.player_speed;
        }

        if input::is_key_down(ctx, Key::D) || input::is_key_down(ctx, Key::Right) {
            self.player_position.x += self.player_speed;
        }

        if input::is_key_down(ctx, Key::W) || input::is_key_down(ctx, Key::Up) {
            self.player_position.y -= self.player_speed;
        }

        if input::is_key_down(ctx, Key::S) || input::is_key_down(ctx, Key::Down) {
            self.player_position.y += self.player_speed;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::hex("#6d2f6a"));

        self.assets.player.draw(
            ctx,
            DrawParams::new()
                .position(self.player_position)
                .origin(Vec2::new(8.0, 8.0)),
        );

        self.assets.mouse.draw(
            ctx,
            DrawParams::new()
                .position(self.mouse_position)
                .scale(Vec2::new(5.0, 5.0))
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
