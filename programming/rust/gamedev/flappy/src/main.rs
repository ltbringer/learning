use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
enum GameMode {
    Menu,
    Playing,
    End
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32
}

impl Player {
    fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
            velocity: 0.0
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity  += 0.2
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}


fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Hello Rust World!")
        .build()?;
    main_loop(ctx, State::new())
}
