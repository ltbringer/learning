use bracket_lib::prelude::*;

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
